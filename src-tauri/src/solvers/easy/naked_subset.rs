use crate::game_board::GameBoard;
use crate::solvers::solution::{Action, Candidate, EliminationDetails, Solution};
use crate::solvers::{Solver, SolverIdentifier};
use crate::utils::House::{Box, Col, Row};
use crate::utils::{BitMap, Coord};

fn solve_naked_subset(
    n: usize,
    game_board: &GameBoard,
    solver_id: SolverIdentifier,
) -> Option<Solution> {
    for i in 0..9 {
        for clue in [Box(i), Row(i), Col(i)] {
            let combos = BitMap::get_combinations(n);
            'combo: for combo in combos {
                let mut eliminations: Vec<Action> = Vec::new();
                let mut candidate_clues = Vec::new();
                let mut candidates = BitMap::new();
                let include = (0..9usize).filter(|x| combo.contains(*x));
                let exclude = (0..9usize).filter(|x| !combo.contains(*x));

                for index in include {
                    let (x, y) = Coord::from_house_and_index(&clue, index);
                    if let Some(candi) = game_board.get_candidates(x, y) {
                        candidates = candidates.union(candi);
                        candidate_clues.push(Candidate {
                            x,
                            y,
                            candidates: candi,
                        })
                    } else {
                        continue 'combo;
                    }
                }
                if candidates.count() == n {
                    for index in exclude {
                        let (x, y) = Coord::from_house_and_index(&clue, index);
                        if let Some(candi) = game_board.get_candidates(x, y) {
                            let target = candidates.intersect(candi);
                            if target.count() > 0 {
                                eliminations.push(Action::Elimination(EliminationDetails {
                                    x,
                                    y,
                                    target,
                                }));
                            }
                        }
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

pub struct NakedPair;
impl Solver for NakedPair {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_naked_subset(2, game_board, self.solver_id())
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::NakedPair
    }
}

pub struct NakedTriple;
impl Solver for NakedTriple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_naked_subset(3, game_board, self.solver_id())
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::NakedTriple
    }
}

pub struct NakedQuadruple;
impl Solver for NakedQuadruple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_naked_subset(4, game_board, self.solver_id())
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::NakedQuadruple
    }
}

#[cfg(test)]
mod naked_subset_test {
    use super::*;
    use crate::{solvers::solution::Action::Elimination, utils::House};
    #[test]
    fn test_naked_pair_sol() {
        let raws = [
            1, 64, 2, 276, 388, 156, 276, 264, 32, 8, 4, 16, 322, 322, 32, 322, 128, 1, 128, 256,
            32, 87, 71, 93, 86, 10, 70, 320, 9, 269, 69, 16, 2, 324, 32, 128, 66, 3, 128, 32, 69,
            256, 8, 17, 84, 32, 16, 261, 128, 8, 69, 326, 259, 70, 18, 137, 72, 87, 199, 213, 32,
            18, 256, 4, 32, 257, 8, 259, 17, 128, 64, 18, 274, 130, 320, 338, 32, 208, 1, 4, 8,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedPair;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        assert_eq!(actions.len(), 3);
        assert_matches!(actions[0],Elimination(EliminationDetails{x:6,y:3,target})if target.get_raw()==18);
        assert_matches!(actions[1],Elimination(EliminationDetails{x:6,y:4,target})if target.get_raw()==2);
        assert_matches!(actions[2],Elimination(EliminationDetails{x:6,y:5,target})if target.get_raw()==16);

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Row(6));

        assert_eq!(candidate_clues.len(), 2);
        assert_matches!(candidate_clues[0],Candidate{ x:6, y:0, candidates }if candidates.get_raw()==18);
        assert_matches!(candidate_clues[1],Candidate{ x:6, y:7, candidates }if candidates.get_raw()==18);
    }
    #[test]
    fn test_naked_pair_not_found() {
        let raws = [
            1, 64, 2, 276, 388, 156, 276, 264, 32, 8, 4, 16, 322, 322, 32, 322, 128, 1, 128, 256,
            32, 87, 71, 93, 86, 10, 70, 320, 9, 269, 69, 16, 2, 324, 32, 128, 66, 3, 128, 32, 69,
            256, 8, 17, 84, 32, 16, 261, 128, 8, 69, 326, 259, 70, 18, 137, 72, 69, 197, 197, 32,
            18, 256, 4, 32, 257, 8, 259, 17, 128, 64, 18, 274, 130, 320, 338, 32, 208, 1, 4, 8,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedPair;
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }
    #[test]
    fn test_naked_triple_sol() {
        let raws = [
            8, 1, 2, 100, 96, 128, 292, 324, 16, 16, 36, 64, 256, 8, 1, 36, 2, 128, 132, 256, 164,
            102, 114, 116, 44, 76, 1, 5, 44, 44, 77, 65, 2, 128, 16, 256, 7, 14, 16, 13, 128, 256,
            64, 5, 32, 64, 128, 256, 16, 33, 36, 13, 13, 2, 32, 70, 132, 67, 339, 80, 257, 385, 8,
            256, 16, 1, 128, 4, 8, 2, 32, 64, 130, 74, 136, 99, 355, 96, 16, 385, 4,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedTriple;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 3;
        let action_std = [(2, 4, 96), (6, 4, 65), (8, 4, 97)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Col(4));

        let clues_len = 3;
        let clues_std = [(0, 4, 96), (3, 4, 65), (5, 4, 33)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn test_naked_triple_not_found() {
        let raws = [
            8, 1, 2, 100, 96, 128, 292, 324, 16, 16, 36, 64, 256, 8, 1, 36, 2, 128, 132, 256, 164,
            102, 18, 116, 44, 76, 1, 5, 44, 44, 77, 65, 2, 128, 16, 256, 7, 14, 16, 13, 128, 256,
            64, 5, 32, 64, 128, 256, 16, 33, 36, 13, 13, 2, 32, 70, 132, 67, 274, 80, 257, 385, 8,
            256, 16, 1, 128, 4, 8, 2, 32, 64, 130, 74, 136, 99, 258, 96, 16, 385, 4,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedTriple;
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }

    #[test]
    fn test_naked_quadruple_sol() {
        let raws = [
            4, 1, 16, 448, 2, 96, 8, 352, 224, 232, 224, 232, 452, 192, 116, 1, 370, 230, 256, 2,
            224, 1, 8, 116, 48, 112, 228, 192, 4, 1, 32, 16, 8, 130, 66, 256, 2, 8, 160, 68, 256,
            68, 160, 1, 16, 16, 352, 352, 2, 1, 128, 4, 8, 96, 200, 208, 202, 200, 32, 256, 18, 4,
            1, 201, 208, 4, 200, 192, 3, 256, 50, 34, 33, 288, 290, 16, 4, 3, 64, 128, 8,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedQuadruple;
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        let action_len = 4;
        let action_std = [(1, 3, 192), (1, 5, 96), (1, 7, 96), (1, 8, 224)];

        assert_eq!(actions.len(), action_len);
        for i in 0..action_len {
            let (x, y, raw) = action_std[i];
            let action = &actions[i];
            assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
        }

        assert_eq!(house_clues.len(), 1);
        assert_matches!(house_clues[0], House::Row(1));

        let clues_len = 4;
        let clues_std = [(1, 0, 232), (1, 1, 224), (1, 2, 232), (1, 4, 192)];

        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn test_naked_quadruple_not_found() {
        let raws = [
            4, 1, 16, 448, 2, 96, 8, 352, 224, 232, 224, 232, 260, 192, 20, 1, 274, 6, 256, 2, 224,
            1, 8, 116, 48, 112, 228, 192, 4, 1, 32, 16, 8, 130, 66, 256, 2, 8, 160, 68, 256, 68,
            160, 1, 16, 16, 352, 352, 2, 1, 128, 4, 8, 96, 200, 208, 202, 200, 32, 256, 18, 4, 1,
            201, 208, 4, 200, 192, 3, 256, 50, 34, 33, 288, 290, 16, 4, 3, 64, 128, 8,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = NakedQuadruple;
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }
}
