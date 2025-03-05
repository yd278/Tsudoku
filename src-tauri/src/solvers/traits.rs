use crate::game_board::GameBoard;
use crate::solvers::solution::Solution;

use super::SolverIdentifier;

pub trait Solver {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution>;
    fn solver_id(&self) -> SolverIdentifier;
}
