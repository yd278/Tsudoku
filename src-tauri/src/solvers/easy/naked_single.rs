use crate::game_board::{Cell, GameBoard};
use crate::solvers::solution::*;
use crate::solvers::traits::Solver;
pub struct NakedSingle;

impl Solver for NakedSingle {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        for row in 0..9 {
            for col in 0..9 {
                if let Cell::Blank(blank_cell) = game_board.get_cell(row, col) {
                    if blank_cell.get_pen_mark().is_none() {
                        let candidates = blank_cell.get_candidates();
                        if candidates.count() == 1 {
                            return Some(Solution {
                                actions: vec![Action::Confirmation(ConfirmationDetails {
                                    x: row,
                                    y: col,
                                    target: candidates.trailing_zeros(),
                                })],
                                house_clues: vec![],
                                candidate_clues: vec![Candidate {
                                    x: row,
                                    y: col,
                                    candidates: *candidates,
                                }],
                            });
                        }
                    }
                }
            }
        }

        None
    }
    
    fn solver_id(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod naked_single_test {
    use super::*;

    #[test]
    fn naked_single_found_test() {
        let board = GameBoard::from_string(
            "..24...5...92..7.334..8.....3.1....495.....378....3.1.....7..616.5..23...9...84..",
        );
        let naked_single_solver = NakedSingle;
        let res = naked_single_solver.solve(&board).unwrap();
        let actions = res.actions;
        assert_eq!(actions.len(), 1);
        let action = &actions[0];
        if let Action::Confirmation(ConfirmationDetails { x, y, target }) = action {
            assert_eq!(*x, 7);
            assert_eq!(*y, 3);
            assert_eq!(*target, 8);
        } else {
            assert!(false);
        }
        let house_clues = res.house_clues;
        assert!(house_clues.is_empty());
        let candidate_clues = res.candidate_clues;
        assert_eq!(candidate_clues.len(), 1);
        let Candidate { x, y, candidates } = candidate_clues[0];
        assert_eq!(x, 7);
        assert_eq!(y, 3);
        assert_eq!(candidates.get_raw(), 256)
    }

    #[test]
    fn naked_single_no_solution_test() {
        let board = GameBoard::from_string(
            ".7.9..8633..78.294..9...1754...........637...........17.....4....1.49..7624..8.19",
        );

        let naked_single_solver = NakedSingle;
        let res = naked_single_solver.solve(&board);
        assert!(res.is_none());
    }
}
