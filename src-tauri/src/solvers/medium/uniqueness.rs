use std::{iter::Cycle, ops::Not};

use crate::{
    game_board::GameBoard,
    impl_with_id,
    solvers::{
        solution::{candidate, Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House, HouseType},
};

fn find_base_line(
    game_board: &GameBoard,
) -> impl Iterator<Item = (House, usize, usize, BitMap)> + '_ {
    (0..2)
        .flat_map(|dim| (0..9).map(move |house_index| HouseType::from_dim(dim).house(house_index)))
        .flat_map(move |house| {
            (0..9).filter_map(move |first| {
                let (fx, fy) = Coord::from_house_and_index(&house, first);
                game_board
                    .get_candidates(fx, fy)
                    .and_then(|candidates| (candidates.count() == 2).then_some((first, candidates)))
                    .and_then(|(first, bi_value)| {
                        (first + 1..9).find_map(|second| {
                            let (sx, sy) = Coord::from_house_and_index(&house, second);
                            game_board.get_candidates(sx, sy).and_then(|candidates| {
                                (candidates == bi_value).then_some((house, first, second, bi_value))
                            })
                        })
                    })
            })
        })
}

struct SemiPossibleUR {
    base_house: House,
    base_bi_value: BitMap,
    first_index: usize,
    second_index: usize,
    span_house: House,
    first_span_candidates: BitMap,
    second_span_candidates: BitMap,
}
/// This functions is for Uniqueness Test 2 and 3
/// The side with two cells only contains UR candidates is called Base House
/// and the side with two cells contains extra candidates is called Span House
///
/// This function returns an iterator to
fn semi_possible_ur(game_board: &GameBoard) -> impl Iterator<Item = SemiPossibleUR> + '_ {
    find_base_line(game_board).flat_map(move |(base_house, first, second, bi_value)| {
        let same_house_flag = first / 3 == second / 3;
        (0..9)
            .filter(move |&x| {
                x != base_house.get_index()
                    && (x / 3 == base_house.get_index() / 3) != same_house_flag
            })
            .filter_map(move |x| {
                let span_house = base_house.get_parallel(x);
                let (fx, fy) = Coord::from_house_and_index(&span_house, first);
                let (sx, sy) = Coord::from_house_and_index(&span_house, second);
                game_board
                    .get_candidates(fx, fy)
                    .and_then(|first_span_candidates| {
                        game_board
                            .get_candidates(sx, sy)
                            .and_then(|second_span_candidates| {
                                bi_value
                                    .iter_ones()
                                    .all(|candidate| game_board.could_have_been(fx, fy, candidate))
                                    .then(|| {
                                        bi_value
                                            .iter_ones()
                                            .all(|candidate| {
                                                game_board.could_have_been(sx, sy, candidate)
                                            })
                                            .then_some(SemiPossibleUR {
                                                base_house,
                                                base_bi_value: bi_value,
                                                first_index: first,
                                                second_index: second,
                                                span_house,
                                                first_span_candidates,
                                                second_span_candidates,
                                            })
                                    })
                            })
                            .flatten()
                    })
            })
    })
}

impl_with_id!(UniquenessTest1, UniquenessTest2, UniquenessTest3);
struct UniquenessTest1 {
    id: usize,
}

impl Solver for UniquenessTest1 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Coord::all_cells()
            .filter_map(|(x, y)| {
                game_board
                    .get_candidates(x, y)
                    .and_then(|candidates| (candidates.count() == 2).then_some((x, y, candidates)))
            })
            .find_map(|(px, py, bi_value)| {
                //closure: returns Option<Solution>
                (0..9)
                    .filter(|&qy| qy != py)
                    .filter_map(|qy| {
                        game_board.get_candidates(px, qy).and_then(|candidates| {
                            (candidates == bi_value).then_some((qy, py / 3 == qy / 3))
                        })
                    })
                    .flat_map(|(qy, same_box_flag)| {
                        (0..9)
                            .filter(|&rx| rx != px)
                            .filter(move |&rx| (rx / 3 == px / 3) != same_box_flag)
                            .filter_map(move |rx| {
                                bi_value
                                    .iter_ones()
                                    .all(|candidate| game_board.could_have_been(rx, qy, candidate))
                                    .then_some((rx, qy))
                            })
                    })
                    .map(|(rx, qy)| {
                        game_board.get_candidates(rx, qy).and_then(|candidates| {
                            (candidates.intersect(&bi_value) == bi_value).then_some(Solution {
                                actions: vec![Action::Elimination(EliminationDetails {
                                    x: rx,
                                    y: qy,
                                    target: bi_value,
                                })],
                                house_clues: vec![
                                    House::Row(px),
                                    House::Row(rx),
                                    House::Col(py),
                                    House::Col(qy),
                                ],
                                candidate_clues: vec![
                                    Candidate::new(px, py, bi_value),
                                    Candidate::new(px, qy, bi_value),
                                    Candidate::new(rx, py, bi_value),
                                    Candidate::new(rx, qy, bi_value.intersect(&candidates)),
                                ],
                                solver_id: self.id,
                            })
                        })
                    })
                    .find_map(|solution| solution)
            })
    }
}

struct UniquenessTest2 {
    id: usize,
}
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
                            ],
                            solver_id: self.id,
                        })
                    })
                    .flatten()
            },
        )
    }
}
struct UniquenessTest3 {
    id: usize,
}

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
                let mask: BitMap = (0..9)
                    .filter(|&x| {
                        let (cx, cy) = Coord::from_house_and_index(&span_house, x);
                        !game_board.not_filled(cx, cy)
                    })
                    .collect();

                (virtual_cell.count() - 1..7)
                    .flat_map(|subset_size| BitMap::get_masked_combo(subset_size, mask))
                    .find_map(|combo| {
                        let subset_candidates = combo
                            .iter_ones()
                            .filter_map(|x| {
                                let (cx, cy) = Coord::from_house_and_index(&span_house, x);
                                game_board.get_candidates(cx, cy)
                            })
                            .fold(BitMap::new(), |acc, candidates| acc.union(&candidates));
                        let subset_candidates = subset_candidates.union(&virtual_cell);
                        (subset_candidates.count() == combo.count() + 1).then(|| {
                            combo
                                .iter_zeros()
                                .filter(|&x| x != first_index && x != second_index)
                                .map(|x| Coord::from_house_and_index(&span_house, x))
                                .filter_map(|(cx, cy)| {
                                    game_board.get_candidates(cx, cy).and_then(|candidates| {
                                        let eliminable_candidates =
                                            candidates.intersect(&subset_candidates);
                                        (eliminable_candidates.count() > 0).then_some(
                                            Action::Elimination(EliminationDetails {
                                                x: cx,
                                                y: cy,
                                                target: eliminable_candidates,
                                            }),
                                        )
                                    })
                                })
                                .collect::<Vec<_>>()
                        })
                        .and_then(|actions|Some((actions,combo,subset_candidates)))
                        
                    })
                    .and_then(|(actions,combo,subset_candidates)| {
                        let candidate_clues =  vec![
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
                                second_span_candidates.intersect(&second_span_candidates),
                            ),
                            Candidate::from_coord(
                                Coord::from_house_and_index(&span_house, second_index),
                                subset_candidates.intersect(&first_span_candidates),
                            ),
                            Candidate::from_coord(
                                Coord::from_house_and_index(&span_house, second_index),
                                subset_candidates.intersect(&second_span_candidates),
                            ),

                        ].into_iter().chain(
                            combo.iter_ones().map(|x|
                                Coord::from_house_and_index(&span_house, x)
                            ).filter_map(|(cx,cy)|{
                                game_board.get_candidates(cx, cy).and_then(|candidates|{
                                    Some(Candidate::new(cx,cy,candidates.intersect(&subset_candidates)))
                                })
                            })
                
                        )
                       .collect();
                        Some(Solution {
                            actions,
                            house_clues: vec![
                                base_house,
                                span_house,
                                base_house.get_perpendicular(first_index),
                                base_house.get_perpendicular(second_index),
                            ],
                            candidate_clues,
                            solver_id: self.id,
                        })
                    })
            },
        )
    }
}
#[cfg(test)]
mod uniqueness_test {
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::utils::House::{Col, Row};
    use crate::{game_board::GameBoard, utils::House};
    fn test_function(
        solver: impl Solver,
        raws: [u16; 81],
        exp_actions: Vec<(usize, usize)>,
        exp_action_targets: Vec<u16>,
        exp_house_clues: Vec<House>,
        exp_candidate_clues: Vec<(usize, usize)>,
        exp_candidate_masks: Vec<u16>,
    ) {
        // raws
        let game_board = GameBoard::from_array(raws);
        // solver type
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        // action data
        let action_len = exp_actions.len();
        let action_std: Vec<_> = exp_actions
            .iter()
            .enumerate()
            .map(|(i, (a, b))| (a, b, exp_action_targets[i]))
            .collect();

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];

            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }
        // // if confirmation
        // assert_eq!(actions.len(), action_len);
        // for i in 0..action_len {
        //     let (x, y, raw) = action_std[i];
        //     let action = &actions[i];
        //     assert_matches!(action, confirmation(ConfirmationDetails{x,y,target:raw});
        // }

        // house_clue data
        let house_clues_len = exp_house_clues.len();

        assert_eq!(house_clues.len(), house_clues_len);
        for i in 0..house_clues_len {
            assert_eq!(house_clues[i], exp_house_clues[i]);
        }

        // candidate_clue data
        let clues_len = exp_candidate_clues.len();
        let clues_std: Vec<_> = exp_candidate_clues
            .iter()
            .enumerate()
            .map(|(i, (a, b))| (a, b, exp_candidate_masks[i]))
            .collect();
        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn uniqueness_test_1() {
        test_function(
            UniquenessTest1::with_id(1),
            [
                64, 16, 256, 136, 4, 1, 2, 32, 136, 129, 8, 32, 16, 192, 2, 256, 65, 4, 4, 130,
                131, 392, 32, 328, 16, 65, 136, 16, 480, 129, 169, 448, 4, 41, 384, 2, 393, 482,
                135, 169, 448, 104, 41, 388, 16, 393, 416, 133, 2, 16, 40, 41, 388, 64, 2, 4, 64,
                288, 8, 288, 128, 16, 1, 32, 1, 16, 4, 2, 128, 64, 8, 256, 384, 384, 8, 64, 1, 16,
                4, 2, 32,
            ],
            vec![(2, 3)],
            vec![136],
            vec![Row(0), Row(2), Col(8), Col(3)],
            vec![(0, 8), (0, 3), (2, 8), (2, 3)],
            vec![136, 136, 136, 136],
        );
    }

    #[test]
    fn uniqueness_test_2() {
        test_function(
            UniquenessTest2::with_id(1),
            [
                128, 4, 256, 8, 82, 67, 114, 3, 96, 16, 2, 8, 32, 256, 65, 128, 65, 4, 64, 1, 32,
                18, 128, 4, 24, 256, 10, 1, 16, 66, 128, 74, 98, 40, 4, 256, 42, 256, 4, 80, 88,
                98, 1, 74, 128, 42, 72, 128, 1, 4, 256, 98, 16, 104, 10, 32, 66, 4, 1, 128, 256,
                72, 16, 256, 72, 1, 66, 32, 16, 4, 128, 74, 4, 128, 16, 256, 66, 8, 66, 32, 1,
            ],
            vec![(0, 4), (0, 6)],
            vec![2, 2],
            vec![Row(1), Row(0), Col(5), Col(7)],
            vec![(1, 5), (1, 7), (0, 5), (0, 7)],
            vec![65, 65, 65, 1],
        );
    }

    #[test]
    fn uniqueness_test_3() {
        test_function(
            UniquenessTest3::with_id(1),
            [128,9,16,256,96,9,4,96,2,4,265,64,19,40,43,24,288,128,32,264,2,212,204,140,24,320,1,64,2,4,8,1,256,32,128,16,16,32,8,132,2,132,64,1,256,1,128,256,32,16,64,2,4,8,2,80,160,193,232,169,256,24,4,8,84,160,198,256,166,1,18,96,256,68,1,70,108,16,128,10,96],
            vec![(2,4),],
            vec![72],
            vec![Row(4), Row(2), Col(3), Col(5)],
            vec![(4,3), (4,5), (2,3), (2,5), (2,3), (2,5), (2,1), (2,6), (2,7),],
            vec![132, 132, 132, 132, 80, 8, 264, 24, 320],
        );
    }
}
