use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::{semi_possible_ur, SemiPossibleUR, UniquenessTest4};

impl Solver for UniquenessTest4 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        semi_possible_ur(game_board).find_map(
            |SemiPossibleUR {
                 base_house,
                 base_bi_value,
                 first_index,
                 second_index,
                 span_house,
                 first_span_candidates,
                 second_span_candidates,
             }| {
                let (px, py) = Coord::from_house_and_index(&span_house, first_index);
                let (qx, qy) = Coord::from_house_and_index(&span_house, second_index);

                base_bi_value
                    .iter_ones()
                    .find_map(|competitor| {
                        game_board
                            .get_hard_link(px, py, competitor, span_house.get_type())
                            .and_then(|(ox, oy)| {
                                (ox == qx && oy == qy).then(|| {
                                    let target = base_bi_value
                                        .difference(&BitMap::from(competitor))
                                        .trailing_zeros();
                                    let actions: Vec<_> = [(px, py), (qx, qy)]
                                        .into_iter()
                                        .filter_map(|(x, y)| {
                                            game_board.contains_candidate(x, y, target).then_some(
                                                Action::Elimination(EliminationDetails {
                                                    x,
                                                    y,
                                                    target: BitMap::from(target),
                                                }),
                                            )
                                        })
                                        .collect();
                                    (!actions.is_empty()).then_some(Solution {
                                        actions,
                                        house_clues: vec![
                                            base_house,
                                            span_house,
                                            base_house.get_perpendicular(first_index),
                                            base_house.get_perpendicular(second_index),
                                        ],
                                        candidate_clues: vec![
                                            Candidate::from_coord(
                                                Coord::from_house_and_index(
                                                    &base_house,
                                                    first_index,
                                                ),
                                                base_bi_value,
                                            ),
                                            Candidate::from_coord(
                                                Coord::from_house_and_index(
                                                    &base_house,
                                                    second_index,
                                                ),
                                                base_bi_value,
                                            ),
                                            Candidate::from_coord(
                                                Coord::from_house_and_index(
                                                    &span_house,
                                                    first_index,
                                                ),
                                                BitMap::from(competitor),
                                            ),
                                            Candidate::from_coord(
                                                Coord::from_house_and_index(
                                                    &span_house,
                                                    second_index,
                                                ),
                                                BitMap::from(competitor),
                                            ),
                                        ],
                                        solver_id: self.id,
                                    })
                                })
                            })
                    })
                    .flatten()
            },
        )
    }
}
