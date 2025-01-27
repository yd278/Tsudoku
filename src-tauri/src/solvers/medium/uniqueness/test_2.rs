use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::{semi_possible_ur, SemiPossibleUR, UniquenessTest2};

impl Solver for UniquenessTest2 {
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
                let first_diff = first_span_candidates.difference(&base_bi_value);
                let second_diff = second_span_candidates.difference(&base_bi_value);
                (first_diff.count() == 1 && first_diff == second_diff)
                    .then(|| {
                        let target = first_diff.trailing_zeros();
                        let (fx, fy) = Coord::from_house_and_index(&span_house, first_index);
                        let (sx, sy) = Coord::from_house_and_index(&span_house, second_index);
                        let eliminables: Vec<_> = Coord::pinched_by(fx, fy, sx, sy)
                            .filter_map(|(ex, ey)| {
                                game_board.contains_candidate(ex, ey, target).then_some(
                                    Action::Elimination(EliminationDetails {
                                        x: ex,
                                        y: ey,
                                        target: BitMap::from(target),
                                    }),
                                )
                            })
                            .collect();
                        (!eliminables.is_empty()).then_some(Solution {
                            actions: eliminables,
                            house_clues: vec![
                                base_house,
                                span_house,
                                base_house.get_perpendicular(first_index),
                                base_house.get_perpendicular(second_index),
                            ],
                            candidate_clues: vec![
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&base_house, first_index),
                                    base_bi_value,
                                ),
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&base_house, second_index),
                                    base_bi_value,
                                ),
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&span_house, first_index),
                                    base_bi_value.intersect(&first_span_candidates),
                                ),
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&span_house, second_index),
                                    base_bi_value.intersect(&second_span_candidates),
                                ),
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&span_house, first_index),
                                    BitMap::from(target),
                                ),
                                Candidate::from_coord(
                                    Coord::from_house_and_index(&span_house, second_index),
                                    BitMap::from(target),
                                ),
                            ],
                            solver_id: self.id,
                        })
                    })
                    .flatten()
            },
        )
    }
}
