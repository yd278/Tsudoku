use crate::solvers::solution::Action::{self, Confirmation, Elimination};
use crate::solvers::solution::{ConfirmationDetails, EliminationDetails};
use crate::utils::{BitMap, Dimension};
use crate::utils::Coord;
pub mod blank_cell;
pub mod dlx_solver;
use blank_cell::BlankCell;

#[derive(Clone, Copy)]
pub enum Cell {
    Printed(usize),
    Blank(BlankCell),
}

pub struct GameBoard {
    grid: [[Cell; 9]; 9],
    row_occupied: [BitMap; 9], // row_occupied[i] .contains(j) : row-j is occupied by number i
    col_occupied: [BitMap; 9],
    box_occupied: [BitMap; 9],
}

impl GameBoard {
    // delete target in a cell's candidate list
    // and mark it as user deleted if user_deleted_flag is true
    fn delete_candidate(&mut self, x: usize, y: usize, target: usize, user_deleted_flag: bool) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
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
    pub fn get_answer(&self, x: usize, y: usize) -> usize {
        match &self.grid[x][y] {
            Cell::Printed(num) => *num,
            Cell::Blank(blank_cell) => blank_cell.get_answer(),
        }
    }

    // check if this cell collides with the target
    fn check_cell_collision(&self, x: usize, y: usize, target: usize) -> Option<(usize, usize)> {
        match &self.grid[x][y] {
            Cell::Printed(ans) if target == *ans => Some((x, y)),
            Cell::Blank(blank_cell) if blank_cell.check_collision(target) => Some((x, y)),
            _ => None,
        }
    }

    // take an cell at (x,y) and return the vector of coordinates that collide with the target
    fn check_clue_collision(&self, x: usize, y: usize, target: usize) -> Vec<(usize, usize)> {
        Coord::seeable_cells(x, y)
            .filter_map(|(xi, yi)| self.check_cell_collision(xi, yi, target))
            .collect()
    }

    // return a unmutable reference to the cell at (x,y)
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[x][y]
    }

    //return the cell specified by x and y contains candidate target
    pub fn contains_candidate(&self, x: usize, y: usize, target: usize) -> bool {
        match &self.grid[x][y] {
            Cell::Blank(cell) if !cell.is_pen_mark() => cell.contains_candidate(target),
            _ => false,
        }
    }
    // check if the given target shouldn't be deleted
    // the caller should ensure that the cell is a blank cell
    pub fn check_pencil_mark_deletion_error(&self, x: usize, y: usize, target: usize) -> bool {
        matches!(self.grid[x][y], Cell::Blank(ref cell) if target == cell.get_answer())
    }

    // check if the given target shouldn't be added
    // the caller should ensure that the cell is a blank cell with no pen mark
    pub fn check_pencil_mark_addition_collision(
        &self,
        x: usize,
        y: usize,
        target: usize,
    ) -> Vec<(usize, usize)> {
        self.check_clue_collision(x, y, target)
    }

    // erase an pencil mark in given cell by user
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is in the candidate set
    pub fn erase_pencil_mark(&mut self, x: usize, y: usize, target: usize) {
        self.delete_candidate(x, y, target, true);
    }

    // add an pencil mark in given cell by user
    // and remove the user deleted flag
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is not in the candidate set
    pub fn add_pencil_mark(&mut self, x: usize, y: usize, target: usize) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            cell.modify(|candidates, user_deleted| {
                if !candidates.contains(target) {
                    candidates.insert(target);
                    user_deleted.remove(target);
                }
            });
        }
    }

    // check if the given target shouldn't be set as pen mark
    // the caller should ensure that the cell is a blank with no pen mark
    pub fn check_pen_mark_addition_error(
        &self,
        x: usize,
        y: usize,
        target: usize,
    ) -> Option<Vec<(usize, usize)>> {
        match &self.grid[x][y] {
            Cell::Blank(cell) if target != cell.get_answer() => {
                Some(self.check_clue_collision(x, y, target))
            }
            _ => None,
        }
    }

    // set a cell as pen mark by user
    pub fn add_pen_mark(&mut self, x: usize, y: usize, target: usize) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            if cell.is_pen_mark() {
                return;
            }
            cell.set_pen_mark(target);
            self.row_occupied[target].insert(x);
            self.col_occupied[target].insert(y);
            self.box_occupied[target].insert(Coord::get_box_id(x, y));

            Coord::seeable_cells(x, y)
                .for_each(|(xi, yi)| self.delete_candidate(xi, yi, target, false));
        }
    }

    // erase a pen mark in given cell by user
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
                    if Coord::row(x)
                        .filter(|&(x, y)| self.is_clue(x, y, target))
                        .count()
                        == 0
                    {
                        self.row_occupied[target].remove(x);
                    }
                    if Coord::col(y)
                        .filter(|&(x, y)| self.is_clue(x, y, target))
                        .count()
                        == 0
                    {
                        self.col_occupied[target].remove(y);
                    }
                    let box_id = Coord::get_box_id(x, y);
                    if Coord::box_coords(box_id)
                        .filter(|&(x, y)| self.is_clue(x, y, target))
                        .count()
                        == 0
                    {
                        self.box_occupied[target].remove(box_id);
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
                    } else if self.check_clue_collision(xi, yi, target).is_empty() {
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
    }

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

    fn execute(&mut self, action: Action) {
        match action {
            Confirmation(ConfirmationDetails { x, y, target }) => {
                self.add_pen_mark(x, y, target);
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

    pub fn box_occupied(&self) -> &[BitMap; 9] {
        &self.box_occupied
    }

    pub fn col_occupied(&self) -> &[BitMap; 9] {
        &self.col_occupied
    }
    pub fn row_occupied(&self) -> &[BitMap; 9] {
        &self.row_occupied
    }

    pub fn occupied_by(&self, dim: &Dimension, target: usize) -> &BitMap{
        match dim {
            Dimension::Row => &self.row_occupied[target],
            Dimension::Col => &self.col_occupied[target],
        }
    }
}

#[cfg(test)]

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
            row_occupied,
            col_occupied,
            box_occupied,
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
            row_occupied,
            col_occupied,
            box_occupied,
        }
    }
}

#[cfg(test)]
pub mod game_board_test {

    use crate::solvers::easy;

    use super::*;

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
