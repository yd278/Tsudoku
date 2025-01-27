use crate::{game_board::GameBoard, solvers::{solution::{Action, Candidate, EliminationDetails, Solution}, Solver}, utils::{BitMap, Coord, House}};

use super::{semi_possible_ur, SemiPossibleUR, UniquenessTest3};



impl Solver for UniquenessTest3 {
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
                let virtual_cell = first_diff.union(&second_diff);
                [
                    Some(span_house),
                    (first_index / 3 == second_index / 3).then_some(House::Box(
                        Coord::get_box_id_by_tuple(Coord::from_house_and_index(
                            &span_house,
                            first_index,
                        )),
                    )),
                ]
                .into_iter()
                .flatten()
                .find_map(|investigate_house| {
                    let mask: BitMap = (0..9)
                        .filter(|&x| {
                            let (cx, cy) = Coord::from_house_and_index(&investigate_house, x);
                            !game_board.not_filled(cx, cy)
                                || Coord::from_house_and_index(&span_house, first_index) == (cx, cy)
                                || Coord::from_house_and_index(&span_house, second_index)
                                    == (cx, cy)
                        })
                        .collect();

                    (virtual_cell.count() - 1..9 - mask.count())
                        .flat_map(|subset_size| BitMap::get_masked_combo(subset_size, mask)) // for some fixed combo with fixed size:
                        .find_map(|combo| {
                            // calculate sub candidates
                            let subset_candidates = combo
                                .iter_ones()
                                .filter_map(|x| {
                                    let (cx, cy) =
                                        Coord::from_house_and_index(&investigate_house, x);
                                    game_board.get_candidates(cx, cy)
                                })
                                .fold(BitMap::new(), |acc, candidates| acc.union(&candidates));
                            let subset_candidates = subset_candidates.union(&virtual_cell);

                            (subset_candidates.count() == combo.count() + 1)
                                .then(|| {
                                    mask.complement()
                                        .difference(&combo)
                                        .iter_ones()
                                        .map(|x| Coord::from_house_and_index(&investigate_house, x))
                                        .filter_map(|(cx, cy)| {
                                            game_board.get_candidates(cx, cy).and_then(
                                                |candidates| {
                                                    let eliminable_candidates =
                                                        candidates.intersect(&subset_candidates);
                                                    (eliminable_candidates.count() > 0).then_some(
                                                        Action::Elimination(EliminationDetails {
                                                            x: cx,
                                                            y: cy,
                                                            target: eliminable_candidates,
                                                        }),
                                                    )
                                                },
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .map(|actions| (actions, combo, subset_candidates))
                        })
                        .map(|(actions, combo, subset_candidates)| {
                            (!actions.is_empty()).then(|| {
                                let candidate_clues = vec![
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(&base_house, first_index),
                                        base_bi_value,
                                    ),
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(&base_house, second_index),
                                        base_bi_value,
                                    ),
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(
                                            &investigate_house,
                                            first_index,
                                        ),
                                        base_bi_value.intersect(&first_span_candidates),
                                    ),
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(
                                            &investigate_house,
                                            second_index,
                                        ),
                                        base_bi_value.intersect(&second_span_candidates),
                                    ),
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(
                                            &investigate_house,
                                            first_index,
                                        ),
                                        subset_candidates.intersect(&first_span_candidates),
                                    ),
                                    Candidate::from_coord(
                                        Coord::from_house_and_index(
                                            &investigate_house,
                                            second_index,
                                        ),
                                        subset_candidates.intersect(&second_span_candidates),
                                    ),
                                ]
                                .into_iter()
                                .chain(
                                    combo
                                        .iter_ones()
                                        .map(|x| Coord::from_house_and_index(&investigate_house, x))
                                        .filter_map(|(cx, cy)| {
                                            game_board.get_candidates(cx, cy).map(|candidates| {
                                                Candidate::new(
                                                    cx,
                                                    cy,
                                                    candidates.intersect(&subset_candidates),
                                                )
                                            })
                                        }),
                                )
                                .collect();
                                Solution {
                                    actions,
                                    house_clues: vec![
                                        base_house,
                                        span_house,
                                        base_house.get_perpendicular(first_index),
                                        base_house.get_perpendicular(second_index),
                                        investigate_house,
                                    ],
                                    candidate_clues,
                                    solver_id: self.id,
                                }
                            })
                        })
                })
                .flatten()
            },
        )
    }
}
