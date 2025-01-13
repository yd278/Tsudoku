use crate::game_board::{Cell, GameBoard};
use crate::solvers::solver_result::candidate;
use crate::solvers::solver_result::confirmation::Confirmation;
use crate::solvers::solver_result::house::House;
use crate::solvers::solver_result::SolverActionResult;
use crate::solvers::solver_result::SolverResult;
use crate::solvers::traits::Solver;
use crate::utils::{BitMap, Coord};

pub struct HiddenSingle;

impl Solver for HiddenSingle {
    fn solve(&self, game_board: &GameBoard) -> Option<crate::solvers::solver_result::SolverResult> {
        let form_result =
            |x: usize, y: usize, target: usize, clue: House, index: usize| -> SolverResult {
                let actions = vec![SolverActionResult::Confirmation(Confirmation {
                    x,
                    y,
                    target,
                })];
                let house_clues = vec![clue];
                let candidate_clues = vec![candidate::Candidate {
                    x,
                    y,
                    candidates: BitMap::from(target),
                }];
                SolverResult {
                    actions,
                    house_clues,
                    candidate_clues,
                }
            };
        for target in 0..9 {
            let contains_target = |(x, y): &(usize, usize)| -> bool {
                let cell = game_board.get_cell(*x, *y);
                if let Cell::Blank(cell) = cell {
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
}
