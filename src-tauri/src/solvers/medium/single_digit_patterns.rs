use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House,HouseType},
};
pub struct Skyscraper;
impl Solver for Skyscraper {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!()
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}