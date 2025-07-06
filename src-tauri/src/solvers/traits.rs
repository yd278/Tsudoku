use super::SolverIdentifier;
use super::easy::*;
use super::hard::*;
use super::medium::*;
use super::solver_enum::SolverEnum;
use crate::game_board::GameBoard;
use crate::solvers::solution::Solution;

use enum_dispatch::enum_dispatch;
#[enum_dispatch]
pub trait Solver {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution>;
    fn solver_id(&self) -> SolverIdentifier;
}
