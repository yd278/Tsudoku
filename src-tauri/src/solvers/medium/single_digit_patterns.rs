use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House, HouseType},
};
pub struct Skyscraper {
    id: usize,
}
impl Skyscraper {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for Skyscraper {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!()
    }
}
