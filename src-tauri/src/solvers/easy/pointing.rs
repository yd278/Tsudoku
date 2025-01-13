use crate::solvers::solver_result::candidate::Candidate;
use crate::solvers::solver_result::elimination::Elimination;
use crate::solvers::solver_result::{SolverActionResult, SolverResult};
use crate::solvers::Solver;

use crate::utils::{BitMap, Coord, House};
pub struct Pointing;

impl Solver for Pointing {
    fn solve(
        &self,
        game_board: &crate::game_board::GameBoard,
    ) -> Option<crate::solvers::solver_result::SolverResult> {
        for box_id in 0..9 {
            'target: for target in 0..9 {
                // for a fixed box and target
                let iter = Coord::box_coords(box_id);
                let mut row = BitMap::new();
                let mut col = BitMap::new();
                let mut candidate_clues = Vec::new();

                // iterate though all cells in the box
                for (i, j) in iter {
                    if game_board.contains_candidate(i, j, target) {
                        row.insert(i);
                        col.insert(j);
                        candidate_clues.push(Candidate {
                            x: i,
                            y: j,
                            candidates: BitMap::from(target),
                        });
                        if row.count() > 1 && col.count() > 1 {
                            // this target doesn't fit pointing pattern, try next one
                            continue 'target;
                        }
                    }
                }
                //this candidate doesn't appear in the box
                if row.count() == 0 {
                    continue;
                }

                // pointing pattern found
                let clue = if row.count() == 1 {
                    House::Row(row.trailing_zeros())
                } else {
                    House::Col(col.trailing_zeros())
                };
                let eliminations: Vec<SolverActionResult> = Coord::house(&clue)
                    .filter(|&(x, y)| Coord::get_box_id(x, y) != box_id)
                    .filter_map(|(x, y)| {
                        if game_board.contains_candidate(x, y, target) {
                            Some(SolverActionResult::Elimination(Elimination {
                                x,
                                y,
                                target: BitMap::from(target),
                            }))
                        } else {
                            None
                        }
                    })
                    .collect();
                if !eliminations.is_empty() {
                    return Some(SolverResult {
                        actions: eliminations,
                        house_clues: vec![House::Box(box_id), clue],
                        candidate_clues,
                    });
                }
            }
        }
        None
    }
}
