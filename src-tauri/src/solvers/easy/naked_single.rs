
use crate::game_board::{ Cell, GameBoard};
use crate::solvers::solver_result::candidate::Candidate;
use crate::solvers::solver_result::confirmation::Confirmation;
use crate::solvers::solver_result::{SolverActionResult, SolverResult};
use crate::solvers::traits::Solver;
pub struct NakedSingle;

impl Solver for NakedSingle {
    fn solve(&self, game_board: &GameBoard) -> Option<crate::solvers::solver_result::SolverResult> {
        for row in 0..9 {
            for col in 0..9 {
                if let Cell::Blank(blank_cell) = game_board.get_cell(row, col) {
                    if blank_cell.get_pen_mark().is_none() {
                        let candidates = blank_cell.get_candidates();
                        if candidates.count() == 1 {
                            return Some(SolverResult {
                                actions: vec![SolverActionResult::Confirmation(Confirmation {
                                    x: row,
                                    y: col,
                                    target: candidates.trailing_zeros(),
                                })],
                                house_clues: vec![],
                                candidate_clues: vec![
                                    Candidate{
                                        x:row,
                                        y:col,
                                        candidates: *candidates,
                                    }
                                ],
                            });
                        }
                    }
                }
            }
        }

        None
    }
}
