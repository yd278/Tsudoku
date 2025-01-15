use crate::game_board::GameBoard;
use crate::solvers::solution::candidate::Candidate;
use crate::solvers::solution::elimination_details::EliminationDetails;
use crate::solvers::solution::{Action, Solution};
use crate::solvers::Solver;

use crate::utils::{BitMap, Coord, House};
pub struct Pointing;

impl Solver for Pointing {
    fn solve(&self, game_board: &GameBoard) -> Option<crate::solvers::solution::Solution> {
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
                let eliminations: Vec<Action> = Coord::house(&clue)
                    .filter(|&(x, y)| Coord::get_box_id(x, y) != box_id)
                    .filter_map(|(x, y)| {
                        if game_board.contains_candidate(x, y, target) {
                            Some(Action::Elimination(EliminationDetails {
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
                        house_clues: vec![House::Box(box_id), clue],
                        candidate_clues,
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod pointing_test {

    use super::*;

    #[test]
    fn pointing_found_test() {
        let board = GameBoard::from_string(
            "95..62.8....51..........25416..7.5.2295...7.88.7.25.695.9..........57....8.39...5",
        );
        let pointing_solver = Pointing;
        let res = pointing_solver.solve(&board).unwrap();
        let actions = res.actions;
        let house_clues = res.house_clues;
        let candidate_clues = res.candidate_clues;
        assert_eq!(actions.len(), 1);
        if let Action::Elimination(EliminationDetails { x, y, target }) = &actions[0] {
            assert_eq!(*x, 0);
            assert_eq!(*y, 2);
            assert_eq!(target.get_raw(), 1);
        } else {
            panic!();
        }
        assert_eq!(house_clues.len(), 2);
        if let House::Box(x) = &house_clues[0] {
            assert_eq!(*x, 2);
        } else {
            panic!()
        }
        if let House::Row(x) = &house_clues[1] {
            assert_eq!(*x, 0);
        } else {
            panic!()
        }
        assert_eq!(candidate_clues.len(), 2);
        let Candidate { x, y, candidates } = &candidate_clues[0];
        assert_eq!(*x, 0);
        assert_eq!(*y, 6);
        assert_eq!(candidates.get_raw(), 1);

        let Candidate { x, y, candidates } = &candidate_clues[1];
        assert_eq!(*x, 0);
        assert_eq!(*y, 8);
        assert_eq!(candidates.get_raw(), 1);
    }

    #[test]
    fn pointing_no_solution_test() {
        let board = GameBoard::from_string(
            "5.47......26.5.....8..912..3.....8..2...3...4..8.....7..132..6.....1.49......93.1",
        );

        let pointing_solver = Pointing;
        let res = pointing_solver.solve(&board);
        assert!(res.is_none());
    }
}
