use crate::game_board::GameBoard;
use crate::solvers::solver_result::SolverResult;

pub trait Solver {
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult>;
}