use crate::game_board::GameBoard;
use crate::solvers::solution::Action::Elimination;
use crate::solvers::solution::Solution;
use crate::solvers::solution::{Action, Candidate, EliminationDetails};
use crate::solvers::Solver;
use crate::utils::{AllEqualValue, BitMap, Coord, House};

pub struct Claiming;

impl Solver for Claiming {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        for i in 0..9 {
            for target in 0..9 {
                for line in [House::Row(i), House::Col(i)] {
                    let mut candidate_clues = Vec::new();
                    if let Some(box_id) = Coord::house(&line)
                        .filter_map(|(x, y)| {
                            if game_board.contains_candidate(x, y, target) {
                                candidate_clues.push(Candidate {
                                    x,
                                    y,
                                    candidates: BitMap::from(target),
                                });
                                Some(Coord::get_box_id(x, y))
                            } else {
                                None
                            }
                        })
                        .all_equal_value()
                    {
                        // potential claiming found
                        let eliminations: Vec<Action> = Coord::box_coords(box_id)
                            .filter(|&(x, y)| !Coord::is_in_house(x, y, &line))
                            .filter_map(|(x, y)| {
                                if game_board.contains_candidate(x, y, target) {
                                    Some(Elimination(EliminationDetails {
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
                            return Some(Solution {
                                actions: eliminations,
                                house_clues: vec![House::Box(box_id), line],
                                candidate_clues,
                            });
                        }
                    }
                }
            }
        }
        None
    }
}
