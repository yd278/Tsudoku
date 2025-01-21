use crate::solvers::solution::Action::{self, Confirmation, Elimination};
use crate::solvers::solution::{ConfirmationDetails, EliminationDetails};
use crate::utils::Coord;
use crate::utils::{BitMap, HouseType};
pub mod blank_cell;
pub mod dlx_solver;
use blank_cell::BlankCell;

#[derive(Clone, Copy)]
pub enum Cell {
    Printed(usize),
    Blank(BlankCell),
}
type CellHardLink = [Option<(usize, usize)>; 9];
pub struct GameBoard {
    grid: [[Cell; 9]; 9],
    hard_links: [[[CellHardLink; 9]; 9]; 3],
    occupied: [[BitMap; 9]; 3], // row_occupied[i] .contains(j) : row-j is occupied by number i
}

///  This section contains game board information inquiries
impl GameBoard {
    /// Get the cell
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[x][y]
    }

    /// Get the expected answer of cell (x,y)
    pub fn get_answer(&self, x: usize, y: usize) -> usize {
        match &self.grid[x][y] {
            Cell::Printed(num) => *num,
            Cell::Blank(blank_cell) => blank_cell.get_answer(),
        }
    }

    /// Get the candidate bitmap of cell (x,y) if it's not printed
    pub fn get_candidates(&self, x: usize, y: usize) -> Option<BitMap> {
        if let Cell::Blank(cell) = self.grid[x][y] {
            if !cell.is_pen_mark() {
                let res = cell.get_candidates();
                return Some(*res);
            } else {
                return None;
            }
        }
        None
    }

    ///Returns true if cell (x,y) contains the target as candidate
    pub fn contains_candidate(&self, x: usize, y: usize, target: usize) -> bool {
        match &self.grid[x][y] {
            Cell::Blank(cell) if !cell.is_pen_mark() => cell.contains_candidate(target),
            _ => false,
        }
    }

    /// Returns true if cell (x,y) is a clue
    pub fn is_clue(&self, x: usize, y: usize, target: usize) -> bool {
        match &self.grid[x][y] {
            Cell::Printed(num) => *num == target,
            Cell::Blank(blank_cell) => {
                if let Some(num) = blank_cell.get_pen_mark() {
                    num == target
                } else {
                    false
                }
            }
        }
    }

    /// Returns a bitmap indicating which line are occupied by the target
    pub fn line_occupied_by(&self, dim: &HouseType, target: usize) -> &BitMap {
        &self.occupied[dim.as_index()][target]
    }

    pub fn occupied(&self) -> &[[BitMap; 9]; 3] {
        &self.occupied
    }
}

///  This section contains game board operation sanity checks
impl GameBoard {
    /// Checks if a deletion of a pencil mark is incorrect
    /// i.e. it deletes the expected answer as a candidate
    ///
    /// This function will *do nothing* if the cell
    /// - is not an pencil mark cell
    /// - doesn't contains the target candidate
    pub fn check_pencil_mark_deletion_error(&self, x: usize, y: usize, target: usize) -> bool {
        matches!(self.grid[x][y], Cell::Blank(ref cell) if target == cell.get_answer())
    }

    /// Checks if a pencil mark collides with seeable clues
    ///
    /// returns empty vector if no collision detected
    ///
    pub fn get_collided_seeable_clues(
        &self,
        x: usize,
        y: usize,
        target: usize,
    ) -> Vec<(usize, usize)> {
        Coord::seeable_cells(x, y)
            .filter_map(|(xi, yi)| self.target_collides_with_clue(xi, yi, target))
            .collect()
    }

    /// Checks if setting cell (x,y) to be target is valid, i.e. it matches the expected answer
    ///
    /// returns
    /// - a vector of coordinates of collided clues if it's invalid
    /// - an empty vector if it's invalid but no current clue collision
    /// - None if it's valid

    pub fn get_pen_mark_addition_collisions(
        &self,
        x: usize,
        y: usize,
        target: usize,
    ) -> Option<Vec<(usize, usize)>> {
        match &self.grid[x][y] {
            Cell::Blank(cell) if target != cell.get_answer() => {
                Some(self.get_collided_seeable_clues(x, y, target))
            }
            _ => None,
        }
    }
}

/// This section contains game board edit operations
impl GameBoard {
    /// Erase an pencil mark in given cell by user
    ///
    /// This function will *do nothing* if the cell
    /// - is not an pencil mark cell
    /// - doesn't contains the target candidate
    pub fn erase_pencil_mark(&mut self, x: usize, y: usize, target: usize) {
        self.delete_candidate(x, y, target, true);
        self.update_cell_hard_link(x, y, target);
    }

    /// Add an pencil mark in given cell by user
    ///
    /// it removes the user deleted flag as well.
    ///
    /// This function will *do nothing* if the cell
    /// - is not an pencil mark cell
    /// - already contained the target candidate
    pub fn add_pencil_mark(&mut self, x: usize, y: usize, target: usize) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            if !cell.is_pen_mark() {
                cell.modify(|candidates, user_deleted| {
                    if !candidates.contains(target) {
                        candidates.insert(target);
                        user_deleted.remove(target);
                    }
                });
                self.update_cell_hard_link(x, y, target);
            }
        }
    }

    /// Set a cell to pen mark and removes corresponding candidate in all seeable cells.
    ///
    /// This function will *do nothing* if the cell
    /// - is a pen mark cell or a printed cell
    pub fn set_pen_mark(&mut self, x: usize, y: usize, target: usize) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            if cell.is_pen_mark() {
                return;
            }
            cell.set_pen_mark(target);
            
            let components = Coord::components_array(x, y);
            for i in 0..3 {
                self.occupied[i][target].insert(components[i]);
            }

            Coord::seeable_cells(x, y)
                .for_each(|(xi, yi)| self.delete_candidate(xi, yi, target, false));
            self.update_grid_hard_link();

        }
    }
    /// Erase the pen mark in cell (x,y)
    ///
    /// This function will:
    /// - erase the pen mark,
    /// - re-compute the pencil marks in this cell, all the candidates which are valid (no collision with current clues) and not deleted by user will appear.
    /// - put the candidate back in all the seeable cells if it's valid (no collision with other clues) and not deleted by user.
    pub fn erase_pen_mark(&mut self, x: usize, y: usize) {
        let mut possible_candidates = BitMap::all();
        let mut to_put_back = Vec::new();

        let target = {
            if let Cell::Blank(ref mut cell) = self.grid[x][y] {
                if !cell.is_pen_mark() {
                    return;
                }
                if let Some(target) = cell.get_pen_mark() {
                    cell.erase_pen_mark();
                    let components = Coord::components_array(x, y);
                    for i in 0..3 {
                        if HouseType::from_index(i)
                            .house(components[i])
                            .to_iter()
                            .filter(|&(x, y)| self.is_clue(x, y, target))
                            .count()
                            == 0
                        {
                            self.occupied[i][target].remove(x);
                        }
                    }
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(target) = target {
            Coord::seeable_cells(x, y).for_each(|(xi, yi)| match self.grid[xi][yi] {
                Cell::Printed(ans) => {
                    possible_candidates.remove(ans);
                }
                Cell::Blank(ref mut cell) => {
                    if let Some(mark) = cell.get_pen_mark() {
                        possible_candidates.remove(mark);
                    } else if self.get_collided_seeable_clues(xi, yi, target).is_empty() {
                        to_put_back.push((xi, yi));
                    }
                }
            });

            for (xi, yi) in to_put_back {
                if let Cell::Blank(ref mut cell) = self.grid[xi][yi] {
                    cell.set_pencil_mark(target);
                }
            }
        }

        if let Cell::Blank(ref mut cell) = self.grid[x][y] {
            cell.update_candidates(&possible_candidates);
        }
        self.update_grid_hard_link();
    }
}

/// This section contains game board solving operations
impl GameBoard {
    /// Returns true if the game is already finished
    pub fn finished(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Blank(cell) = self.grid[i][j] {
                    if !cell.is_pen_mark() {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Execute an action

    fn execute(&mut self, action: Action) {
        match action {
            Confirmation(ConfirmationDetails { x, y, target }) => {
                self.set_pen_mark(x, y, target);
            }
            Elimination(EliminationDetails { x, y, target }) => {
                for i in 0..9 {
                    if target.contains(i) {
                        self.erase_pencil_mark(x, y, i);
                    }
                }
            }
        }
    }
}

/// This section contains some private APIs for internal use
impl GameBoard {
    fn update_grid_hard_link(&mut self){
        self.hard_links = [[[[None;9];9];9];3];
        for dim in 0..3{
            for house_index in 0..9 {
                for target in 0..9 {
                    let appearance: Vec<_> = HouseType::from_index(dim)
                        .house(house_index)
                        .to_iter()
                        .filter(|&(x, y)| 
                        self.contains_candidate(x, y, target))
                        .collect();

                    if appearance.len() == 2 {
                        let (x1, y1) = appearance[0];
                        let (x2, y2) = appearance[1];
                        self.hard_links[dim][x1][y1][target] = Some((x2, y2));
                        self.hard_links[dim][x2][y2][target] = Some((x1, y1));
                    }
                }
            }
        }
    }
    fn update_cell_hard_link(&mut self, x: usize, y: usize, target: usize) {
        let components = Coord::components_array(x, y);
        for dim in 0..3 {
            let mut appearance = Vec::with_capacity(9);
            for (x,y) in HouseType::from_index(dim)
                .house(components[dim])
                .to_iter(){
                    self.hard_links[dim][x][y][target] = None;
                    if self.contains_candidate(x, y, target){
                        appearance.push((x,y));
                    }
                }
                
            if appearance.len() == 2 {
                let (x1, y1) = appearance[0];
                let (x2, y2) = appearance[1];
                self.hard_links[dim][x1][y1][target] = Some((x2, y2));
                self.hard_links[dim][x2][y2][target] = Some((x1, y1));
            }
        }
    }


    // delete target in a cell's candidate list
    // and mark it as user deleted if user_deleted_flag is true
    fn delete_candidate(&mut self, x: usize, y: usize, target: usize, user_deleted_flag: bool) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            if !cell.is_pen_mark() {
                cell.modify(|candidates, user_deleted| {
                    if candidates.contains(target) {
                        candidates.remove(target);
                        if user_deleted_flag {
                            user_deleted.insert(target);
                        }
                    }
                });
            }
        }
    }

    // check if the target collides with cell (x,y) as a clue
    fn target_collides_with_clue(
        &self,
        x: usize,
        y: usize,
        target: usize,
    ) -> Option<(usize, usize)> {
        match &self.grid[x][y] {
            Cell::Printed(ans) if target == *ans => Some((x, y)),
            Cell::Blank(blank_cell) if blank_cell.check_collision(target) => Some((x, y)),
            _ => None,
        }
    }
}

#[cfg(test)]
pub mod game_board_test {

    use crate::solvers::easy;

    use super::*;
    fn compute_hard_links(grid: [[Cell; 9]; 9]) -> [[[CellHardLink; 9]; 9]; 3] {
        let mut res = [[[[None; 9]; 9]; 9]; 3];
        for dim in 0..3 {
            for house_index in 0..9 {
                for target in 0..9 {
                    let appearance: Vec<_> = HouseType::from_index(dim)
                        .house(house_index)
                        .to_iter()
                        .filter(|&(x, y)| match grid[x][y] {
                            Cell::Printed(_) => false,
                            Cell::Blank(blank_cell) => {
                                !blank_cell.is_pen_mark() && blank_cell.contains_candidate(target)
                            }
                        })
                        .collect();

                    if appearance.len() == 2 {
                        let (x1, y1) = appearance[0];
                        let (x2, y2) = appearance[1];
                        res[dim][x1][y1][target] = Some((x2, y2));
                        res[dim][x2][y2][target] = Some((x1, y1));
                    }
                }
            }
        }
        res
    }

    impl GameBoard {
        pub fn from_string(input: &str) -> Self {
            let mut grid = [[Cell::Blank(BlankCell::new_empty_cell()); 9]; 9];
            let mut row_occupied = [BitMap::new(); 9];
            let mut col_occupied = [BitMap::new(); 9];
            let mut box_occupied = [BitMap::new(); 9];
            for (index, c) in input.chars().enumerate() {
                let i = index / 9;
                let j = index % 9;
                if c.is_digit(10) {
                    let num = c.to_digit(10).unwrap() as usize - 1;
                    grid[i][j] = Cell::Printed(num);
                    row_occupied[num].insert(i);
                    col_occupied[num].insert(j);
                    box_occupied[num].insert(Coord::get_box_id(i, j));
                }
            }
            for (index, c) in input.chars().enumerate() {
                let i = index / 9;
                let j = index % 9;
                if !c.is_digit(10) {
                    let mut possible_candidates = BitMap::all();
                    for (x, y) in Coord::seeable_cells(i, j) {
                        if let Cell::Printed(num) = grid[x][y] {
                            possible_candidates.remove(num);
                        }
                    }
                    if let Cell::Blank(ref mut cell) = grid[i][j] {
                        cell.set_candidates(possible_candidates);
                    }
                }
            }
            GameBoard {
                grid,
                occupied: [row_occupied, col_occupied, box_occupied],
                hard_links: compute_hard_links(grid),
            }
        }

        pub fn from_array(arr: [u16; 81]) -> Self {
            let mut i = 0;
            let mut j = 0;
            let mut grid = [[Cell::Blank(BlankCell::new_empty_cell()); 9]; 9];
            let mut row_occupied = [BitMap::new(); 9];
            let mut col_occupied = [BitMap::new(); 9];
            let mut box_occupied = [BitMap::new(); 9];
            for raw in arr {
                let candidates = BitMap::from_raw(raw);
                if candidates.count() == 1 {
                    let num = candidates.trailing_zeros();

                    grid[i][j] = Cell::Printed(num);
                    row_occupied[num].insert(i);
                    col_occupied[num].insert(j);
                    box_occupied[num].insert(Coord::get_box_id(i, j));
                } else {
                    if let Cell::Blank(ref mut cell) = grid[i][j] {
                        cell.set_candidates(candidates);
                    }
                }
                j += 1;
                if j == 9 {
                    j = 0;
                    i += 1;
                }
            }
            GameBoard {
                grid,
                occupied: [row_occupied, col_occupied, box_occupied],
                hard_links: compute_hard_links(grid),
            }
        }
    }

    fn to_string(game_board: &GameBoard) -> String {
        let mut res = String::new();
        for i in 0..9 {
            for j in 0..9 {
                match game_board.get_cell(i, j) {
                    Cell::Printed(ans) => res.push_str(&(ans + 1).to_string()),
                    Cell::Blank(c) => res.push_str(&(c.get_answer() + 1).to_string()),
                }
            }
        }
        res
    }

    #[test]
    fn test_solver_1() {
        let mut game_board = GameBoard::from_string(
            "...8...6..58.19....23...4.87..........16.45..........28.6...29....97.18..7...2...",
        );
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        assert!(res.is_ok());
        assert_eq!(
            to_string(&game_board),
            "147823965658419723923567418794258631281634579365791842816345297532976184479182356"
        );
    }
    #[test]
    fn test_solver_2() {
        let mut game_board = GameBoard::from_string(
            ".....3......71......7.4.15371...2.4.5.2...6.1.8.9...25463.7.9......94......6.....",
        );
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        assert!(res.is_ok());
        assert_eq!(
            to_string(&game_board),
            "146853279325719864897246153719562348532487691684931725463175982278394516951628437"
        );
    }

    #[test]
    fn test_no_solution() {
        let mut game_board = GameBoard::from_string(
            "..4..3......71......7.4.15371...2.4.5.2...6.1.8.9...25463.7.9......94......6.....",
        );
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        if let Err(dlx_solver::dlx_solution::DLXSolution::NoSolution) = res {
            assert!(true);
        } else {
            assert!(false);
        }
    }
    #[test]

    fn test_multi_solution() {
        let mut game_board = GameBoard::from_string(
            "...8...6..58.19.....3...4.87..........16.45..........28.....29....97.18..7...2...",
        );
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        if let Err(dlx_solver::dlx_solution::DLXSolution::MultipleSolutions) = res {
            assert!(true);
        } else {
            assert!(false);
        }
    }
    #[test]
    fn test_easy_solvers() {
        let mut game_board = GameBoard::from_string(
            "..68532..2.36...1..........6.......2..59.47..3.......8..........2...63.7..47829..",
        );
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        assert!(res.is_ok());
        let solvers = easy::get_easy_solvers();
        while !game_board.finished() {
            for solver in &solvers {
                if let Some(solution) = solver.solve(&game_board) {
                    for action in solution.actions {
                        match &action {
                            Confirmation(confirmation_details) => {
                                let ConfirmationDetails { x, y, target } = confirmation_details;
                                assert_eq!(game_board.get_answer(*x, *y), *target);
                            }
                            Elimination(elimination_details) => {
                                let EliminationDetails { x, y, target } = elimination_details;
                                for i in (0..9).filter(|x| target.contains(*x)) {
                                    assert_ne!(game_board.get_answer(*x, *y), i);
                                }
                            }
                        }
                        game_board.execute(action);
                    }
                    break;
                }
            }
        }
    }
}
