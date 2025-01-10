use crate::utils::BitMap;
use crate::utils::Coord;
pub mod blank_cell;
pub mod dlx_solver;
use blank_cell::BlankCell;

#[derive(Clone, Copy)]
pub enum Cell {
    Printed(u8),
    Blank(BlankCell),
}

pub struct GameBoard {
    grid: [[Cell; 9]; 9],
}

impl GameBoard {
    // delete target in a cell's candidate list
    // and mark it as user deleted if user_deleted_flag is true
    fn delete_candidate(&mut self, x: usize, y: usize, target: u8, user_deleted_flag: bool) {
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

    // check if this cell collides with the target
    fn check_cell_collision(&self, x: usize, y: usize, target: u8) -> Option<(usize, usize)> {
        match &self.grid[x][y] {
            Cell::Printed(ans) if target == *ans => Some((x, y)),
            Cell::Blank(blank_cell) if blank_cell.check_collision(target) => Some((x, y)),
            _ => None,
        }
    }

    // take an cell at (x,y) and return the vector of coordinates that collide with the target
    // the caller should ensure that the cell is a blank cell
    fn check_clue_collision(&self, x: usize, y: usize, target: u8) -> Vec<(usize, usize)> {
        Coord::seeable_cells(x, y)
            .filter_map(|(xi, yi)| self.check_cell_collision(xi, yi, target))
            .collect()
    }

    // return a unmutable reference to the cell at (x,y)
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[x][y]
    }

    // check if the given target shouldn't be deleted
    // the caller should ensure that the cell is a blank cell
    pub fn check_pencil_mark_deletion_error(&self, x: usize, y: usize, target: u8) -> bool {
        matches!(self.grid[x][y], Cell::Blank(ref cell) if target == cell.get_answer())
    }

    // check if the given target shouldn't be added
    // the caller should ensure that the cell is a blank cell with no pen mark
    pub fn check_pencil_mark_addition_collision(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Vec<(usize, usize)> {
        self.check_clue_collision(x, y, target)
    }

    // erase an pencil mark in given cell by user
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is in the candidate set
    pub fn erase_pencil_mark(&mut self, x: usize, y: usize, target: u8) {
        self.delete_candidate(x, y, target, true);
    }

    // add an pencil mark in given cell by user
    // and remove the user deleted flag
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is not in the candidate set
    pub fn add_pencil_mark(&mut self, x: usize, y: usize, target: u8) {
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
        target: u8,
    ) -> Option<Vec<(usize, usize)>> {
        match &self.grid[x][y] {
            Cell::Blank(cell) if target != cell.get_answer() => {
                Some(self.check_clue_collision(x, y, target))
            }
            _ => None,
        }
    }

    // set a cell as pen mark by user
    // the caller should ensure that the cell is a blank cell with no pen mark
    pub fn add_pen_mark(&mut self, x: usize, y: usize, target: u8) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            cell.set_pen_mark(target);

            Coord::seeable_cells(x, y)
                .for_each(|(xi, yi)| self.delete_candidate(xi, yi, target, false));
        }
    }

    // erase a pen mark in given cell by user
    // the caller should ensure that the cell is a blank cell with a pen mark
    pub fn erase_pen_mark(&mut self, x: usize, y: usize) {
        let mut possible_candidates = BitMap::all();
        let mut to_put_back = Vec::new();

        let target = {
            if let Cell::Blank(ref mut cell) = self.grid[x][y] {
                if let Some(target) = cell.get_pen_mark() {
                    cell.erase_pen_mark();
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
}


#[cfg(test)]
mod game_board_test {


    use super::*;

    fn from_string(input: &str) -> GameBoard {
        let mut grid = [[Cell::Blank(BlankCell::new_empty_cell()); 9]; 9];
        for (index, c) in input.chars().enumerate() {
            let i = index / 9;
            let j = index % 9;
            if c.is_digit(10) {
                grid[i][j] = Cell::Printed(c.to_digit(10).unwrap() as u8 - 1);
            }
        }
        GameBoard { grid }

    }
    fn to_string(game_board: &GameBoard) -> String {
        let mut res = String::new();
        for i in 0..9 {
            for j in 0..9 {
                match game_board.get_cell(i, j) {
                    Cell::Printed(ans) => res.push_str(&(ans+1).to_string()),
                    Cell::Blank(c) => res.push_str(&(c.get_answer()+1).to_string()),
                }
            }
        }
        res
    }

    #[test]
    fn test_solver_1() {
        let mut game_board = from_string("...8...6..58.19....23...4.87..........16.45..........28.6...29....97.18..7...2...");
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        assert!(res.is_ok());
        assert_eq!(to_string(&game_board), "147823965658419723923567418794258631281634579365791842816345297532976184479182356");
    }
    #[test]
    fn test_solver_2(){
        let mut game_board = from_string(".....3......71......7.4.15371...2.4.5.2...6.1.8.9...25463.7.9......94......6.....");
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        assert!(res.is_ok());
        assert_eq!(to_string(&game_board), "146853279325719864897246153719562348532487691684931725463175982278394516951628437");
   
    }

    #[test]
    fn test_no_solution(){
        let mut game_board = from_string("..4..3......71......7.4.15371...2.4.5.2...6.1.8.9...25463.7.9......94......6.....");
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        if let Err(dlx_solver::dlx_solution::DLXSolution::NoSolution) = res {
            assert!(true);
        } else {
            assert!(false);
        }
        
    }
    #[test]

    fn test_multi_solution(){
        let mut game_board = from_string("...8...6..58.19.....3...4.87..........16.45..........28.....29....97.18..7...2...");
        let res = dlx_solver::DLXSolver::solve_sudoku(&mut game_board);
        if let Err(dlx_solver::dlx_solution::DLXSolution::MultipleSolutions) = res {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    
}
