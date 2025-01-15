use crate::game_board::GameBoard;
use crate::solvers::solution::Solution;
use crate::solvers::Solver;

pub struct HiddenPair;

impl Solver for HiddenPair {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!();
    }
}

pub struct HiddenTriple;

impl Solver for HiddenTriple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!();
    }
}

pub struct HiddenQuadruple;

impl Solver for HiddenQuadruple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!();
    }
}
