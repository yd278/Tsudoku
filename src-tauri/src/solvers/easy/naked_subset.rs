use crate::game_board::GameBoard;
use crate::solvers::solution::{Action, Candidate, EliminationDetails, Solution};
use crate::solvers::Solver;
use crate::utils::House::{Box, Col, Row};
use crate::utils::{BitMap, Coord};

fn solve_naked_subset(n: usize, game_board: &GameBoard) -> Option<Solution> {
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
                        candidates = candidates.union(&candi);
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
                            let target = candidates.intersect(&candi);
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
        solve_naked_subset(2, game_board)
    }
}

pub struct NakedTriple;

impl Solver for NakedTriple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_naked_subset(3, game_board)
    }
}

pub struct NakedQuadruple;

impl Solver for NakedQuadruple {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        solve_naked_subset(4, game_board)
    }
}
