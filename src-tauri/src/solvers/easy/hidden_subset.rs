use std::vec;

use crate::game_board::{Cell, GameBoard};
use crate::solvers::solution::{Action, Candidate, EliminationDetails, Solution};
use crate::solvers::Solver;
use crate::utils::House::{Box, Col, Row};
use crate::utils::{BitMap, Coord};

fn solve_hidden_subset(n: usize, game_board: &GameBoard, solver_id: usize) -> Option<Solution> {
    for i in 0..9 {
        for clue in [Box(i), Row(i), Col(i)] {
            let combos = BitMap::get_combinations(n);
            'combo: for combo in combos {
                //combo 是可能的candidates
                // 把所有包括combo的格子的加进来
                let mut action_cells = BitMap::new();
                for index in 0..9 {
                    let (x, y) = Coord::from_house_and_index(&clue, index);
                    match game_board.get_cell(x, y) {
                        Cell::Printed(num) => {
                            if combo.contains(*num) {
                                continue 'combo;
                            }
                        }
                        Cell::Blank(blank_cell) => {
                            if let Some(num) = blank_cell.get_pen_mark() {
                                if combo.contains(num) {
                                    continue 'combo;
                                }
                            } else {
                                let candidates = blank_cell.get_candidates();
                                if candidates.intersect(&combo).count() != 0 {
                                    action_cells.insert(index);
                                }
                            }
                        }
                    }
                }
                if action_cells.count() == n {
                    let mut eliminations: Vec<Action> = Vec::new();
                    let mut candidate_clues: Vec<Candidate> = Vec::new();
                    for i in (0..9).filter(|x| action_cells.contains(*x)) {
                        let (x, y) = Coord::from_house_and_index(&clue, i);
                        let mut include = BitMap::new();
                        let mut exclude = BitMap::new();
                        if let Some(cell_candidates) = game_board.get_candidates(x, y) {
                            for candidate in (0..9).filter(|x| cell_candidates.contains(*x)) {
                                if combo.contains(candidate) {
                                    include.insert(candidate);
                                } else {
                                    exclude.insert(candidate);
                                }
                            }
                        }
                        if exclude.count() != 0 {
                            eliminations.push(Action::Elimination(EliminationDetails {
                                x,
                                y,
                                target: exclude,
                            }));
                        }
                        candidate_clues.push(Candidate {
                            x,
                            y,
                            candidates: include,
                        });
                    }

                    if !eliminations.is_empty() {
                        return Some(Solution {
                            actions: eliminations,
                            house_clues: vec![clue],
                            candidate_clues,
                            solver_id,
                        });
                    }
                }
            }
        }
    }
    None
}
pub struct HiddenPair {
    id: usize,
}
impl HiddenPair {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for HiddenPair {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_hidden_subset(2, game_board, self.id)
    }
}

pub struct HiddenTriple {
    id: usize,
}

impl HiddenTriple {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for HiddenTriple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_hidden_subset(3, game_board, self.id)
    }
}

pub struct HiddenQuadruple {
    id: usize,
}
impl HiddenQuadruple {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for HiddenQuadruple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_hidden_subset(4, game_board, self.id)
    }
}

#[cfg(test)]
mod hidden_subset_test {
    use super::*;
    use crate::{solvers::solution::Action::Elimination, utils::House};
    #[test]
    fn test_hidden_pair_sol() {
        let raws = [
            1, 64, 32, 256, 2, 136, 28, 20, 144, 144, 8, 256, 209, 4, 193, 96, 34, 226, 148, 134,
            150, 32, 144, 200, 72, 1, 256, 2, 260, 1, 28, 56, 288, 116, 128, 112, 388, 32, 132, 23,
            64, 259, 20, 8, 18, 72, 16, 72, 134, 160, 162, 256, 38, 1, 472, 386, 218, 202, 168, 4,
            1, 48, 56, 92, 6, 94, 74, 1, 98, 128, 256, 56, 32, 1, 136, 136, 256, 16, 2, 64, 4,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenPair::with_id(2);
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 1;
        let action_std = [(2, 5, 128)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Row(2));

        let clues_len = 2;
        let clues_std = [(2, 5, 72), (2, 6, 72)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn test_hidden_pair_not_found() {
        let raws = [
            16, 1, 2, 32, 72, 192, 136, 4, 256, 44, 64, 36, 137, 256, 3, 170, 16, 129, 40, 256,
            128, 4, 11, 16, 64, 34, 33, 1, 52, 8, 144, 34, 132, 256, 98, 80, 256, 2, 52, 81, 65,
            69, 48, 128, 8, 128, 48, 64, 264, 40, 258, 50, 1, 4, 100, 52, 256, 2, 132, 8, 1, 96,
            144, 2, 128, 33, 321, 16, 321, 4, 8, 96, 68, 8, 17, 65, 132, 32, 144, 256, 2,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenPair::with_id(2);
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }
    #[test]
    fn test_hidden_triple_sol() {
        let raws = [
            194, 256, 82, 130, 146, 32, 1, 4, 8, 4, 25, 19, 64, 27, 17, 256, 32, 128, 129, 9, 32,
            4, 137, 256, 82, 82, 18, 67, 128, 87, 257, 68, 8, 50, 275, 307, 32, 17, 256, 130, 130,
            17, 4, 8, 64, 8, 68, 19, 32, 273, 68, 18, 128, 275, 65, 100, 69, 272, 96, 128, 8, 338,
            306, 16, 96, 128, 8, 257, 2, 96, 257, 4, 256, 2, 8, 17, 100, 68, 128, 81, 49,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenTriple::with_id(2);
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 2;
        let action_std = [(6, 7, 64), (6, 8, 32)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Row(6));

        let clues_len = 3;
        let clues_std = [(6, 3, 272), (6, 7, 274), (6, 8, 274)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn test_hidden_triple_not_found() {
        let raws = [
            16, 1, 2, 32, 72, 192, 136, 4, 256, 44, 64, 36, 137, 256, 3, 170, 16, 129, 40, 256,
            128, 4, 11, 16, 64, 34, 33, 1, 52, 8, 144, 34, 132, 256, 98, 80, 256, 2, 52, 81, 65,
            69, 48, 128, 8, 128, 48, 64, 264, 40, 258, 50, 1, 4, 100, 52, 256, 2, 132, 8, 1, 96,
            144, 2, 128, 33, 321, 16, 321, 4, 8, 96, 68, 8, 17, 65, 132, 32, 144, 256, 2,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenTriple::with_id(1);
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }

    #[test]
    fn test_hidden_quadruple_sol() {
        let raws = [
            16, 1, 2, 32, 72, 192, 136, 4, 256, 44, 64, 36, 137, 256, 3, 170, 16, 129, 40, 256,
            128, 4, 11, 16, 64, 34, 33, 1, 52, 8, 144, 34, 132, 256, 98, 80, 256, 2, 52, 81, 65,
            69, 48, 128, 8, 128, 48, 64, 280, 40, 258, 50, 1, 4, 100, 52, 256, 2, 132, 8, 1, 96,
            144, 2, 128, 33, 321, 16, 321, 4, 8, 96, 68, 8, 17, 65, 132, 32, 144, 256, 2,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenQuadruple::with_id(1);
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 1;
        let action_std = [(5, 3, 16)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Box(4));

        let clues_len = 4;
        let clues_std = [(3, 4, 34), (5, 3, 264), (5, 4, 40), (5, 5, 258)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn test_hidden_quadruple_not_found() {
        let raws = [
            16, 1, 2, 32, 72, 192, 136, 4, 256, 44, 64, 36, 137, 256, 3, 170, 16, 129, 40, 256,
            128, 4, 11, 16, 64, 34, 33, 1, 52, 8, 144, 34, 132, 256, 98, 80, 256, 2, 52, 81, 65,
            69, 48, 128, 8, 128, 48, 64, 264, 40, 258, 50, 1, 4, 100, 52, 256, 2, 132, 8, 1, 96,
            144, 2, 128, 33, 321, 16, 321, 4, 8, 96, 68, 8, 17, 65, 132, 32, 144, 256, 2,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = HiddenQuadruple::with_id(1);
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }
}
