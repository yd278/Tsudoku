use crate::{
    game_board::GameBoard,
    impl_with_id,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
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

impl_with_id!(UniquenessTest1, UniquenessTest2);
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
        find_base_line(game_board)
            .flat_map(|(base_house, first, second, bi_value)| {
                let same_house_flag = first / 3 == second / 3;
                (0..9)
                    .filter(move |&x| {
                        x != base_house.get_index()
                            && (x / 3 == base_house.get_index() / 3) != same_house_flag
                    })
                    .map(move |x| {
                        (
                            base_house,
                            base_house.get_parallel(x),
                            first,
                            second,
                            bi_value,
                        )
                    })
            })
            .find_map(|(base_house, span_house, first, second, bi_value)| {
                let (px, py) = Coord::from_house_and_index(&span_house, first);
                game_board
                    .get_candidates(px, py)
                    .and_then(|first_candidates| {
                        let target_set = first_candidates.difference(&bi_value);
                        (target_set.count() == 1)
                            .then(|| {
                                let target = target_set.trailing_zeros();
                                bi_value
                                    .iter_ones()
                                    .all(|candidate| game_board.could_have_been(px, py, candidate))
                                    .then(|| {
                                        let (qx, qy) =
                                            Coord::from_house_and_index(&span_house, second);
                                        game_board.get_candidates(qx, qy).and_then(
                                            |second_candidates| {
                                                (bi_value.iter_ones().all(|candidate| {
                                                    game_board.could_have_been(qx, qy, candidate)
                                                }) && second_candidates.difference(&bi_value)
                                                    == target_set)
                                                    .then(|| {
                                                        let eliminable: Vec<_> =
                                                            Coord::pinched_by(px, py, qx, qy)
                                                                .filter(|&(ex, ey)| {
                                                                    game_board.contains_candidate(
                                                                        ex, ey, target,
                                                                    )
                                                                })
                                                                .collect();

                                                        (!eliminable.is_empty()).then_some(
                                                            Solution {
                                                                actions: eliminable
                                                                    .into_iter()
                                                                    .map(|(x, y)| {
                                                                        Action::Elimination(
                                                                            EliminationDetails {
                                                                                x,
                                                                                y,
                                                                                target:
                                                                                    BitMap::from(
                                                                                        target,
                                                                                    ),
                                                                            },
                                                                        )
                                                                    })
                                                                    .collect(),
                                                                house_clues: vec![
                                                                    base_house,
                                                                    span_house,
                                                                    base_house
                                                                        .get_perpendicular(first),
                                                                    base_house
                                                                        .get_perpendicular(second),
                                                                ],
                                                                candidate_clues: vec![
                                                                    Candidate::from_coord(
                                                                        Coord::from_house_and_index(
                                                                            &base_house,
                                                                            first,
                                                                        ),
                                                                        bi_value,
                                                                    ),
                                                                    Candidate::from_coord(
                                                                        Coord::from_house_and_index(
                                                                            &base_house,
                                                                            second,
                                                                        ),
                                                                        bi_value,
                                                                    ),
                                                                    Candidate::from_coord(
                                                                        Coord::from_house_and_index(
                                                                            &span_house,
                                                                            first,
                                                                        ),
                                                                        bi_value.intersect(
                                                                            &first_candidates,
                                                                        ),
                                                                    ),
                                                                    Candidate::from_coord(
                                                                        Coord::from_house_and_index(
                                                                            &span_house,
                                                                            second,
                                                                        ),
                                                                        bi_value.intersect(
                                                                            &second_candidates,
                                                                        ),
                                                                    ),
                                                                ],
                                                                solver_id: self.id,
                                                            },
                                                        )
                                                    })
                                                    .flatten()
                                            },
                                        )
                                    })
                                    .flatten()
                            })
                            .flatten()
                    })
            })
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
}
