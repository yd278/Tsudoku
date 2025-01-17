use crate::game_board::{Cell, GameBoard};
use crate::solvers::solution::{Action::Confirmation, Candidate, ConfirmationDetails, Solution};
use crate::solvers::Solver;
use crate::utils::{BitMap, Coord, House};

pub struct HiddenSingle;

impl Solver for HiddenSingle {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        let form_result =
            |x: usize, y: usize, target: usize, clue: House, index: usize| -> Solution {
                let actions = vec![Confirmation(ConfirmationDetails { x, y, target })];
                let house_clues = vec![clue];
                let candidate_clues = vec![Candidate {
                    x,
                    y,
                    candidates: BitMap::from(target),
                }];
                Solution {
                    actions,
                    house_clues,
                    candidate_clues,
                }
            };
        for target in 0..9 {
            let contains_target = |(x, y): &(usize, usize)| -> bool {
                let cell = game_board.get_cell(*x, *y);
                if let Cell::Blank(cell) = cell {
                    if cell.is_pen_mark() {
                        return false;
                    }
                    return cell.get_candidates().contains(target);
                }
                false
            };
            for i in 0..9 {
                let filtered = Coord::row(i).filter(contains_target).collect::<Vec<_>>();
                if filtered.len() == 1 {
                    let (x, y) = filtered[0];
                    return Some(form_result(x, y, target, House::Row(i), 0));
                }
                let filtered = Coord::col(i).filter(contains_target).collect::<Vec<_>>();
                if filtered.len() == 1 {
                    let (x, y) = filtered[0];
                    return Some(form_result(x, y, target, House::Col(i), 0));
                }
                let filtered = Coord::box_coords(i)
                    .filter(contains_target)
                    .collect::<Vec<_>>();
                if filtered.len() == 1 {
                    let (x, y) = filtered[0];
                    return Some(form_result(x, y, target, House::Box(i), 0));
                }
            }
        }
        None
    }

    fn solver_id(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod hidden_single_test {
    use super::*;

    #[test]
    fn naked_single_found_test() {
        let board = GameBoard::from_string(
            ".7.9..8633..78.294..9...1754...........637...........17.....4....1.49..7624..8.19",
        );
        let hidden_single_solver = HiddenSingle;
        let res = hidden_single_solver.solve(&board).unwrap();
        let actions = res.actions;
        assert_eq!(actions.len(), 1);
        let action = &actions[0];
        if let Confirmation(ConfirmationDetails { x, y, target }) = action {
            assert_eq!(*x, 0);
            assert_eq!(*y, 5);
            assert_eq!(*target, 3);
        } else {
            assert!(false);
        }
        let house_clues = res.house_clues;
        assert_eq!(house_clues.len(), 1);
        let house_clue = &house_clues[0];
        match *house_clue {
            House::Row(i) => assert_eq!(i, 0),
            House::Col(_) => panic!(),
            House::Box(_) => panic!(),
        }
        let candidate_clues = res.candidate_clues;
        assert_eq!(candidate_clues.len(), 1);
        let Candidate { x, y, candidates } = candidate_clues[0];
        assert_eq!(x, 0);
        assert_eq!(y, 5);
        assert_eq!(candidates.get_raw(), 8)
    }

    #[test]
    fn naked_single_no_solution_test() {
        let board = GameBoard::from_string(
            "95..62.8....51..........25416..7.5.2295...7.88.7.25.695.9..........57....8.39...5",
        );

        let hidden_single_solver = HiddenSingle;
        let res = hidden_single_solver.solve(&board);
        assert!(res.is_none());
    }
}
