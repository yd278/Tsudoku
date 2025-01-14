use crate::solvers::Solver;
use crate::game_board::GameBoard;
use crate::solvers::solver_result::SolverResult;

pub struct NakedPair;

impl Solver for NakedPair{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}

pub struct NakedTriple;

impl Solver for NakedTriple{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}

pub struct NakedQuadruple;

impl Solver for NakedQuadruple{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}