use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver, SolverIdentifier,
    },
    utils::{BitMap, Coord, House, HouseType},
};

fn find_eliminable(
    game_board: &GameBoard,
    base_sets: &BitMap,
    cover_sets: Vec<House>,
    target: usize,
) -> Vec<Action> {
    cover_sets
        .into_iter()
        .flat_map(|house| {
            base_sets.iter_zeros().filter_map(move |x| {
                let (x, y) = Coord::from_house_and_index(&house, x);
                game_board.get_candidates(x, y).and_then(|candidates| {
                    if candidates.contains(target) {
                        Some(Action::Elimination(EliminationDetails {
                            x,
                            y,
                            target: BitMap::from(target),
                        }))
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}

fn cover_sets_from_appearance(appearance: BitMap, dim: HouseType) -> Vec<House> {
    appearance.iter_ones().map(|x| dim.house(x)).collect()
}

fn check_base_set_combo(
    game_board: &GameBoard,
    n: usize,
    base_dim: &HouseType,
    target: usize,
    combo: &BitMap,
    solver_id: SolverIdentifier,
) -> Option<Solution> {
    let mut appearance = BitMap::new();
    for base in combo.iter_ones() {
        for cover in (0..9).filter(|index| {
            let (x, y) = Coord::from_house_and_index(&base_dim.house(base), *index);
            game_board.contains_candidate(x, y, target)
        }) {
            appearance.insert(cover);
            if appearance.count() > n {
                return None;
            };
        }
    }
    if appearance.count() == n {
        let cover_sets: Vec<House> = appearance
            .iter_ones()
            .map(|x| base_dim.other().house(x))
            .collect();
        let elimination = find_eliminable(game_board, combo, cover_sets, target);
        if !elimination.is_empty() {
            return Some(Solution {
                actions: elimination,
                house_clues: combo
                    .iter_ones()
                    .map(|base_index| base_dim.house(base_index))
                    .chain(
                        appearance
                            .iter_ones()
                            .map(|cover_index| base_dim.other().house(cover_index)),
                    )
                    .collect(),
                candidate_clues: combo
                    .iter_ones()
                    .flat_map(|base_index| {
                        appearance.iter_ones().map({
                            move |cover_index| {
                                Coord::from_house_and_index(
                                    &base_dim.house(base_index),
                                    cover_index,
                                )
                            }
                        })
                    })
                    .filter(|(x, y)| game_board.contains_candidate(*x, *y, target))
                    .map(|(x, y)| Candidate {
                        x,
                        y,
                        candidates: BitMap::from(target),
                    })
                    .collect(),
                solver_id,
            });
        } else {
            return None;
        }
    }
    None
}

fn check_fish_with_dim(
    game_board: &GameBoard,
    n: usize,
    base_dim: &HouseType,
    solver_id: SolverIdentifier,
) -> Option<Solution> {
    for target in 0..9 {
        let mask = game_board.occupied()[base_dim.as_dim()][target];
        for combo in BitMap::get_masked_combo(n, mask) {
            if let Some(solution) =
                check_base_set_combo(game_board, n, base_dim, target, &combo, solver_id)
            {
                return Some(solution);
            }
        }
    }
    None
}

pub struct XWing;

impl Solver for XWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 2, &base_dim, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::XWing
    }
}

pub struct Swordfish;

impl Solver for Swordfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 3, &base_dim, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::Swordfish
    }
}

pub struct Jellyfish;

impl Solver for Jellyfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [HouseType::Row, HouseType::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 4, &base_dim, self.solver_id()))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::Jellyfish
    }
}

#[cfg(test)]
mod fish_test {
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::utils::House::{Col, Row};
    #[test]
    fn x_wing_test() {
        let raws = [
            288, 1, 8, 80, 356, 372, 384, 2, 388, 2, 324, 356, 128, 292, 292, 8, 1, 16, 16, 128,
            260, 9, 2, 9, 64, 32, 260, 64, 260, 261, 32, 16, 128, 5, 8, 2, 8, 32, 17, 4, 65, 2,
            145, 256, 193, 128, 20, 2, 256, 8, 65, 21, 80, 32, 289, 336, 128, 81, 357, 373, 2, 80,
            8, 4, 2, 368, 89, 353, 377, 273, 128, 321, 257, 8, 336, 2, 128, 337, 32, 4, 321,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = XWing;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 3;
        let action_std = [(5, 6, 16), (6, 3, 16), (6, 5, 16)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }
        let house_clues_len = 4;
        let house_clues_std = [Col(1), Col(7), Row(5), Row(6)];

        assert_eq!(house_clues.len(), house_clues_len);
        for i in 0..house_clues_len {
            assert_eq!(house_clues[i], house_clues_std[i]);
        }

        let clues_len = 4;
        let clues_std = [(5, 1, 16), (6, 1, 16), (5, 7, 16), (6, 7, 16)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn swordfish_test() {
        let raws = [
            12, 16, 256, 2, 196, 196, 1, 32, 136, 1, 64, 12, 20, 148, 32, 256, 136, 2, 2, 32, 128,
            257, 257, 8, 16, 4, 64, 128, 256, 64, 5, 32, 16, 2, 9, 12, 32, 2, 5, 64, 8, 256, 128,
            16, 5, 12, 9, 16, 128, 7, 3, 32, 64, 256, 320, 4, 9, 32, 131, 131, 72, 384, 16, 336,
            129, 2, 24, 276, 132, 72, 257, 32, 272, 136, 32, 280, 336, 65, 4, 2, 129,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = Swordfish;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 1;
        let action_std = [(1, 4, 4)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        // house_clue data
        let house_clues_len = 6;
        let house_clues_std = [Row(0), Row(5), Row(7), Col(0), Col(4), Col(5)];

        assert_eq!(house_clues.len(), house_clues_len);
        for i in 0..house_clues_len {
            assert_eq!(house_clues[i], house_clues_std[i]);
        }

        let clues_len = 7;
        let clues_std = [
            (0, 0, 4),
            (0, 4, 4),
            (0, 5, 4),
            (5, 0, 4),
            (5, 4, 4),
            (7, 4, 4),
            (7, 5, 4),
        ];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn jellyfish_test() {
        let raws = [
            16, 32, 129, 256, 133, 8, 64, 132, 2, 256, 192, 2, 36, 16, 192, 1, 8, 36, 5, 68, 8, 97,
            193, 2, 16, 384, 288, 32, 8, 64, 2, 385, 132, 260, 16, 5, 130, 134, 384, 5, 8, 16, 32,
            64, 261, 5, 16, 257, 68, 320, 32, 8, 2, 128, 130, 386, 32, 16, 68, 68, 258, 1, 8, 64,
            1, 4, 8, 2, 256, 128, 32, 16, 8, 258, 16, 128, 32, 1, 262, 260, 64,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = Jellyfish;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 1;
        let action_std = [(3, 8, 4)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        // house_clue data
        let house_clues_len = 8;
        let house_clues_std = [
            Row(1),
            Row(2),
            Row(4),
            Row(5),
            Col(0),
            Col(1),
            Col(3),
            Col(8),
        ];

        assert_eq!(house_clues.len(), house_clues_len);
        for i in 0..house_clues_len {
            assert_eq!(house_clues[i], house_clues_std[i]);
        }

        let clues_len = 9;
        let clues_std = [
            (1, 3, 4),
            (1, 8, 4),
            (2, 0, 4),
            (2, 1, 4),
            (4, 1, 4),
            (4, 3, 4),
            (4, 8, 4),
            (5, 0, 4),
            (5, 3, 4),
        ];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
}
