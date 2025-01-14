use crate::solvers::Solver;
use crate::game_board::GameBoard;
use crate::solvers::solver_result::SolverResult;

pub struct HiddenPair;

impl Solver for HiddenPair{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}

pub struct HiddenTriple;

impl Solver for HiddenTriple{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}

pub struct HiddenQuadruple;

impl Solver for HiddenQuadruple{
    fn solve(&self, game_board: &GameBoard) -> Option<SolverResult> {
        todo!();
    }
}