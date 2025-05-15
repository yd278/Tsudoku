use crate::{
    game_board::GameBoard,
    solvers::{solution::Solution, Solver, SolverIdentifier},
    utils::HouseType,
};

pub struct SueDeCoq;
mod yoke;
use scorpius::Scorpius;
use yoke::Yoke;
mod orion;
use orion::Orion;
mod scorpius;
impl Solver for SueDeCoq {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        todo!()
    }

    fn solver_id(&self) -> crate::solvers::SolverIdentifier {
        SolverIdentifier::SueDeCoq
    }
}

impl SueDeCoq {
    fn iter_possible_yokes(game_board: &GameBoard) -> impl Iterator<Item = Yoke> + '_ {
        (0..2).flat_map(move |line_dim| {
            (0..9).flat_map(move |line_id| {
                (0..9)
                    .filter_map(move |box_id| Yoke::try_new(game_board, line_dim, line_id, box_id))
            })
        })
    }

    fn iter_valid_orion(
        game_board: &GameBoard,
        yoke: Yoke,
    ) -> impl Iterator<Item = (Orion, Yoke)> + '_ {
        game_board
            .get_als_by_house(HouseType::Box.as_dim(), yoke.box_id())
            .iter()
            .filter_map(move |als| Orion::try_new(yoke, als).zip(Some(yoke)))
    }

    fn iter_valid_scorpius(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
    ) -> impl Iterator<Item = (Yoke, Orion, Scorpius)> + '_ {
        game_board
            .get_als_by_house(yoke.line_dim(), yoke.line_id())
            .iter()
            .filter_map(move |als| {
                Scorpius::try_new(yoke, orion, als).map(|scorpius| (yoke, orion, scorpius))
            })
    }

    fn generate_solution(
        game_board: &GameBoard,
        yoke: Yoke,
        orion: Orion,
        scorpius: Scorpius,
    ) -> Option<Solution> {
        todo!()
    }
}
