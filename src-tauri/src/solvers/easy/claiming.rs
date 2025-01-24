use crate::game_board::GameBoard;
use crate::solvers::solution::Action::Elimination;
use crate::solvers::solution::Solution;
use crate::solvers::solution::{Action, Candidate, EliminationDetails};
use crate::solvers::Solver;
use crate::utils::{AllEqualValue, BitMap, Coord, House};
use crate::impl_with_id;
pub struct Claiming {
    id: usize,
}
impl_with_id!(Claiming);

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
                                solver_id: self.id,
                            });
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod claiming_test {

    use super::*;
    #[test]
    fn test_claiming_sol() {
        let raws = [
            20, 32, 2, 20, 64, 1, 128, 256, 8, 256, 65, 129, 2, 140, 140, 68, 16, 32, 84, 8, 144,
            436, 404, 180, 70, 71, 69, 80, 67, 49, 149, 157, 140, 62, 38, 256, 8, 4, 256, 80, 32,
            2, 1, 128, 80, 128, 67, 49, 341, 285, 76, 126, 38, 84, 2, 256, 64, 37, 21, 52, 52, 8,
            128, 1, 16, 8, 228, 132, 256, 100, 100, 2, 32, 128, 4, 8, 2, 80, 256, 65, 17,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = Claiming::with_id(3);
        let Solution {
            actions,
            house_clues,
            candidate_clues,
            solver_id: _,
        } = solver.solve(&game_board).unwrap();

        assert_eq!(actions.len(), 1);
        assert_matches!(actions[0],Elimination(EliminationDetails{x:5,y:1,target})if target.get_raw()==64);

        assert_eq!(house_clues.len(), 2);
        assert_matches!(house_clues[0], House::Box(3));
        assert_matches!(house_clues[1], House::Row(3));

        assert_eq!(candidate_clues.len(), 2);
        assert_matches!(candidate_clues[0],Candidate{ x:3, y:0, candidates }if candidates.get_raw()==64);
        assert_matches!(candidate_clues[1],Candidate{ x:3, y:1, candidates }if candidates.get_raw()==64);
    }
    #[test]
    fn test_claiming_not_found() {
        let raws = [
            20, 32, 2, 20, 64, 1, 128, 256, 8, 256, 65, 129, 2, 140, 140, 68, 16, 32, 84, 8, 144,
            436, 404, 180, 70, 71, 69, 80, 67, 49, 149, 157, 140, 62, 38, 256, 8, 4, 256, 80, 32,
            2, 1, 128, 80, 128, 3, 49, 341, 285, 76, 126, 38, 84, 2, 256, 64, 37, 21, 52, 52, 8,
            128, 1, 16, 8, 228, 132, 256, 100, 100, 2, 32, 128, 4, 8, 2, 80, 256, 65, 17,
        ];
        let game_board = GameBoard::from_array(raws);
        let solver = Claiming::with_id(3);
        let solution = solver.solve(&game_board);
        assert!(solution.is_none());
    }
}
