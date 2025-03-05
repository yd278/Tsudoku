use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver, SolverIdentifier,
    },
    utils::{AllEqualValue, BitMap, Coord, HouseType},
};

fn get_coords_with_target_by_masks<'a>(
    game_board: &'a GameBoard,
    first: &'a BitMap,
    second: &'a BitMap,
    first_dim: &'a HouseType,
    target: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    first
        .iter_ones()
        .flat_map(move |first_index| {
            second.iter_ones().map(move |second_index| {
                Coord::from_house_and_index(&first_dim.house(first_index), second_index)
            })
        })
        .filter(move |&(x, y)| game_board.contains_candidate(x, y, target))
}

fn find_finned_fish(
    game_board: &GameBoard,
    base_dim: &HouseType,
    n: usize,
    solver_id: SolverIdentifier,
) -> Option<Solution> {
    for target in 0..9 {
        for base in BitMap::get_masked_combo(n, *game_board.house_occupied_by(base_dim, target)) {
            for cover in BitMap::get_masked_combo(
                n,
                *game_board.house_occupied_by(&base_dim.other(), target),
            ) {
                let cover_comp = cover.complement();
                let base_comp = base.complement();
                let body_clues: Vec<_> =
                    get_coords_with_target_by_masks(game_board, &base, &cover, base_dim, target)
                        .map(|(x, y)| Candidate {
                            x,
                            y,
                            candidates: BitMap::from(target),
                        })
                        .collect();

                if body_clues.is_empty() {
                    continue;
                }

                let fin_clues: Vec<_> = get_coords_with_target_by_masks(
                    game_board,
                    &base,
                    &cover_comp,
                    base_dim,
                    target,
                )
                .map(|(x, y)| Candidate {
                    x,
                    y,
                    candidates: BitMap::from(target),
                })
                .collect();

                if fin_clues.is_empty() {
                    continue;
                }

                let fin_box = fin_clues
                    .clone()
                    .into_iter()
                    .map(
                        |Candidate {
                             x,
                             y,
                             candidates: _,
                         }| Coord::get_box_id(x, y),
                    )
                    .all_equal_value();

                if let Some(fin_box) = fin_box {
                    let eliminable: Vec<_> = get_coords_with_target_by_masks(
                        game_board, &base_comp, &cover, base_dim, target,
                    )
                    .filter(|&(x, y)| Coord::get_box_id(x, y) == fin_box)
                    .map(|(x, y)| {
                        Action::Elimination(EliminationDetails {
                            x,
                            y,
                            target: BitMap::from(target),
                        })
                    })
                    .collect();
                    if !eliminable.is_empty() {
                        return Some(Solution {
                            actions: eliminable,
                            candidate_clues: [body_clues, fin_clues].concat(),
                            house_clues: base
                                .iter_ones()
                                .map(|base_index| base_dim.house(base_index))
                                .chain(
                                    cover
                                        .iter_ones()
                                        .map(|cover_index| base_dim.other().house(cover_index)),
                                )
                                .collect(),
                            solver_id,
                        });
                    }
                }
            }
        }
    }
    None
}

pub struct FinnedXWing;

impl Solver for FinnedXWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| find_finned_fish(game_board, &base_dim, 2, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::FinnedXWing
    }
}
pub struct FinnedSwordfish;

impl Solver for FinnedSwordfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| find_finned_fish(game_board, &base_dim, 3, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::FinnedSwordFish
    }
}

pub struct FinnedJellyfish;

impl Solver for FinnedJellyfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| find_finned_fish(game_board, &base_dim, 4, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::FinnedJellyfish
    }
}

#[cfg(test)]
mod finned_test {
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::utils::House::{Col, Row};
    use crate::{game_board::GameBoard, utils::House};
    fn test_function(
        solver: impl Solver,
        raws: [u16; 81],
        target: u16,
        exp_actions: Vec<(usize, usize)>,
        exp_house_clues: Vec<House>,
        exp_candidate_clues: Vec<(usize, usize)>,
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
            .into_iter()
            .map(|(a, b)| (a, b, target))
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
            .into_iter()
            .map(|(a, b)| (a, b, target))
            .collect();
        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }

    #[test]
    fn test_f2() {
        test_function(
            FinnedXWing,
            [
                64, 273, 257, 128, 280, 32, 28, 276, 2, 384, 32, 8, 4, 336, 2, 208, 1, 192, 4, 400,
                2, 1, 344, 272, 216, 336, 32, 129, 140, 196, 74, 33, 132, 256, 102, 16, 257, 2,
                324, 72, 48, 20, 101, 128, 76, 16, 140, 32, 74, 257, 388, 69, 70, 76, 2, 133, 133,
                16, 132, 8, 96, 96, 256, 32, 64, 16, 256, 132, 1, 2, 8, 132, 8, 388, 388, 32, 2,
                64, 148, 20, 1,
            ],
            128,
            vec![(6, 1)],
            vec![Row(2), Row(8), Col(1), Col(6)],
            vec![(2, 1), (2, 6), (8, 1), (8, 6), (8, 2)],
        );
    }
    #[test]
    fn test_f3() {
        test_function(
            FinnedSwordfish,
            [
                2, 325, 276, 8, 337, 32, 17, 193, 208, 336, 128, 32, 337, 4, 337, 27, 73, 82, 80,
                89, 24, 211, 211, 211, 32, 256, 4, 336, 32, 2, 337, 81, 4, 128, 65, 8, 128, 84, 20,
                83, 8, 83, 3, 32, 256, 8, 320, 1, 32, 450, 450, 4, 16, 66, 1, 2, 64, 4, 400, 400,
                280, 136, 32, 4, 264, 280, 401, 32, 401, 64, 2, 144, 32, 272, 128, 66, 66, 8, 272,
                4, 1,
            ],
            1,
            vec![(0, 6)],
            vec![Row(1), Row(4), Row(7), Col(3), Col(5), Col(6)],
            vec![
                (1, 3),
                (1, 5),
                (1, 7),
                (4, 3),
                (4, 5),
                (4, 7),
                (7, 3),
                (7, 5),
                (1, 8),
            ],
        );
    }
    #[test]
    fn test_f4() {
        test_function(
            FinnedJellyfish,
            [
                328, 17, 328, 72, 17, 128, 4, 2, 32, 128, 2, 17, 32, 256, 4, 64, 8, 17, 72, 32, 4,
                2, 17, 72, 145, 256, 144, 265, 64, 128, 16, 38, 34, 9, 36, 265, 2, 21, 273, 69, 8,
                96, 129, 100, 453, 41, 5, 41, 69, 128, 256, 2, 16, 77, 100, 128, 96, 12, 38, 16,
                256, 1, 78, 37, 8, 2, 256, 64, 33, 48, 128, 20, 16, 256, 97, 128, 38, 43, 40, 68,
                78,
            ],
            1,
            vec![(4, 8), (5, 8)],
            vec![
                Row(1),
                Row(3),
                Row(7),
                Row(8),
                Col(0),
                Col(2),
                Col(5),
                Col(8),
            ],
            vec![
                (1, 2),
                (1, 8),
                (3, 0),
                (3, 8),
                (7, 0),
                (7, 5),
                (8, 2),
                (8, 5),
                (3, 6),
            ],
        );
    }

    #[test]
    fn test_s2() {
        test_function(
            FinnedXWing,
            [
                1, 32, 260, 64, 18, 128, 274, 20, 8, 336, 2, 8, 48, 49, 4, 336, 128, 289, 128, 80,
                68, 256, 59, 41, 82, 85, 55, 66, 320, 128, 8, 324, 16, 1, 32, 262, 90, 280, 32,
                129, 453, 65, 474, 92, 278, 88, 4, 1, 2, 480, 96, 472, 88, 272, 4, 65, 2, 48, 112,
                256, 24, 25, 128, 328, 128, 320, 4, 89, 73, 32, 2, 17, 32, 9, 16, 129, 136, 2, 4,
                256, 64,
            ],
            16,
            vec![(2, 8)],
            vec![Row(0), Row(7), Col(4), Col(8)],
            vec![(0, 4), (7, 4), (7, 8), (0, 6), (0, 7)],
        );
    }

    #[test]
    fn test_s3() {
        test_function(
            FinnedSwordfish,
            [
                34, 42, 256, 64, 128, 36, 16, 5, 9, 128, 16, 64, 265, 257, 12, 32, 268, 2, 40, 4,
                1, 312, 304, 2, 128, 328, 320, 96, 96, 2, 128, 8, 1, 4, 272, 272, 5, 256, 12, 48,
                112, 112, 2, 128, 9, 9, 128, 16, 2, 4, 256, 64, 9, 32, 336, 72, 40, 4, 368, 120, 1,
                2, 128, 90, 74, 128, 25, 81, 88, 256, 32, 4, 260, 1, 36, 288, 2, 128, 8, 80, 80,
            ],
            8,
            vec![(7, 0)],
            vec![Col(1), Col(2), Col(8), Row(0), Row(4), Row(7)],
            vec![(0, 1), (7, 1), (4, 2), (0, 8), (4, 8), (6, 1), (6, 2)],
        );
    }
    #[test]
    fn test_s4() {
        test_function(
            FinnedJellyfish,
            [
                14, 32, 256, 68, 70, 1, 128, 72, 16, 1, 136, 64, 136, 16, 256, 32, 2, 4, 16, 132,
                14, 196, 10, 32, 73, 256, 65, 12, 64, 44, 44, 256, 128, 16, 1, 2, 14, 1, 128, 16,
                76, 6, 256, 32, 72, 256, 16, 34, 104, 1, 74, 72, 4, 128, 64, 256, 12, 2, 44, 16, 9,
                128, 41, 32, 2, 16, 1, 128, 72, 4, 72, 256, 128, 12, 1, 256, 96, 68, 2, 16, 104,
            ],
            8,
            vec![(6, 6)],
            vec![
                Row(1),
                Row(5),
                Row(7),
                Row(8),
                Col(1),
                Col(3),
                Col(5),
                Col(6),
            ],
            vec![
                (1, 1),
                (1, 3),
                (5, 3),
                (5, 5),
                (5, 6),
                (6, 5),
                (7, 1),
                (7, 7),
                (8, 8),
            ],
        );
    }
}
