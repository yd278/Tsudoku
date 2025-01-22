use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, HouseType},
};

fn check_turbot(
    game_board: &GameBoard,
    soft_dim: usize,
    hard1: usize,
    hard2: usize,
    solver_id: usize,
) -> Option<Solution> {
    for target in 0..9 {
        for soft_house_index in 0..9 {
            let appearance: Vec<_> =
                Coord::house(&HouseType::from_index(soft_dim).house(soft_house_index))
                    .filter(|&(x, y)| game_board.contains_candidate(x, y, target))
                    .collect();
            for i in 0..appearance.len() {
                let (x1, y1) = appearance[i];
                if let Some((p1, q1)) =
                    game_board.get_hard_link(x1, y1, target, HouseType::from_index(hard1))
                {
                    for (j, &(x2, y2)) in appearance.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        if let Some((p2, q2)) =
                            game_board.get_hard_link(x2, y2, target, HouseType::from_index(hard2))
                        {
                            let actions: Vec<_> = Coord::all_cells()
                                .filter(|&(u, v)| Coord::sees(p1, q1, u, v))
                                .filter(|&(u, v)| Coord::sees(p2, q2, u, v))
                                .filter(|&(u, v)| (u != x1 || v != y1) && (u != x2 || v != y2))
                                .filter(|&(u,v)| game_board.contains_candidate(u, v, target))
                                .map(|(u, v)| {
                                    Action::Elimination(EliminationDetails {
                                        x: u,
                                        y: v,
                                        target: BitMap::from(target),
                                    })
                                })
                                .collect();

                            if !actions.is_empty() {
                                return Some(Solution {
                                    actions,
                                    house_clues: vec![
                                        HouseType::from_index(soft_dim).house(soft_house_index),
                                        HouseType::from_index(hard1)
                                            .house(Coord::components_proj(x1, y1, hard1)),
                                        HouseType::from_index(hard2)
                                            .house(Coord::components_proj(x2, y2, hard2)),
                                    ],
                                    candidate_clues: vec![
                                        Candidate::new_single(x1, y1, target),
                                        Candidate::new_single(x2, y2, target),
                                        Candidate::new_single(p1, q1, target),
                                        Candidate::new_single(p2, q2, target),
                                    ],
                                    solver_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    //   skyscraper: weak in 0/1, pincers = weak.other
    //   2-string kite: week in 2, pincers in 0&1
    //   turbot fish:  weak in 0/1, pincers in 2 & weak.other
    //
    None
}

pub struct EmptyRectangle {
    id: usize,
}
impl EmptyRectangle {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
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
        (0..2).find_map(|x| check_turbot(game_board, x, 1 - x, 1 - x, self.id))
    }
}

pub struct TwoStringKite {
    id: usize,
}
impl TwoStringKite {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for TwoStringKite {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        check_turbot(game_board, 2, 0, 1, self.id)
    }
}

pub struct TurbotFish {
    id: usize,
}
impl TurbotFish {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for TurbotFish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        (0..2).find_map(|x| check_turbot(game_board, x, 2, 1 - x, self.id))
    }
}
#[cfg(test)]
mod single_digit_patterns_test {
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::utils::House::{Box, Col, Row};
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
    fn test_skyscraper() {
        test_function(
            Skyscraper::with_id(3),
            [
                1, 8, 292, 96, 100, 2, 292, 128, 16, 262, 128, 308, 24, 36, 1, 64, 42, 302, 6, 102,
                116, 24, 256, 128, 39, 43, 46, 18, 50, 1, 4, 8, 256, 50, 64, 128, 128, 272, 8, 98,
                98, 96, 272, 4, 1, 64, 294, 292, 128, 1, 16, 8, 34, 290, 268, 324, 452, 99, 16,
                100, 167, 43, 46, 28, 84, 2, 97, 224, 100, 165, 256, 44, 32, 1, 132, 256, 130, 8,
                134, 16, 64,
            ],
            256,
            vec![(5, 2)],
            vec![Col(6), Row(0), Row(4)],
            vec![(0, 6), (4, 6), (0, 2), (4, 1)],
        );
    }

    #[test]
    fn test_two_string_kite() {
        test_function(
            TwoStringKite::with_id(3),
            [
                8, 34, 64, 161, 4, 131, 16, 33, 256, 304, 128, 1, 304, 304, 64, 2, 4, 8, 272, 292,
                262, 8, 307, 257, 128, 33, 64, 129, 260, 276, 401, 409, 397, 64, 2, 32, 129, 64,
                276, 2, 401, 32, 257, 8, 132, 2, 8, 32, 64, 385, 389, 257, 16, 132, 64, 257, 8, 4,
                257, 16, 32, 128, 2, 288, 16, 258, 416, 426, 392, 4, 64, 1, 4, 35, 128, 33, 64, 3,
                8, 256, 16,
            ],
            32,
            vec![(1, 3)],
            vec![Box(6), Row(8), Col(0)],
            vec![(8, 1), (7, 0), (8, 3), (1, 0)],
        );
    }
    #[test]
    fn test_turbot_fish() {
        test_function(
            TurbotFish::with_id(3),
            [
                273, 8, 273, 128, 290, 274, 354, 96, 4, 128, 2, 32, 4, 64, 264, 16, 1, 264, 272, 4,
                64, 1, 290, 282, 290, 128, 298, 80, 208, 4, 66, 8, 32, 1, 256, 210, 8, 112, 145,
                322, 386, 387, 226, 4, 242, 97, 256, 2, 16, 4, 129, 224, 8, 224, 354, 1, 384, 290,
                386, 4, 8, 16, 480, 354, 224, 8, 290, 16, 386, 4, 96, 1, 4, 176, 400, 8, 1, 64,
                416, 2, 416,
            ],
            128,
            vec![(4, 5)],
            vec![Col(1), Box(3), Row(7)],
            vec![(3, 1), (7, 1), (4, 2), (7, 5)],
        );
    }
}
