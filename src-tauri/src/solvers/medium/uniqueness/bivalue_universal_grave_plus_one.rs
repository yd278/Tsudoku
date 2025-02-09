use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, ConfirmationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::BiValueUniversalGravePlusOne;

#[derive(Default)]
struct HouseCandidateCounts {
    count: [usize; 9],
    special: Option<usize>,
}
impl HouseCandidateCounts {
    /// add an candidate to the house
    /// returns None if this is invalid BUG+1 pattern
    /// returns Some(true) if this target is special
    /// return  Some(false) if this target is regular(yet)
    pub fn add(&mut self, target: usize, expecting_special: bool) -> Option<bool> {
        self.count[target] += 1;
        match self.count[target].cmp(&3) {
            std::cmp::Ordering::Less => Some(false),
            std::cmp::Ordering::Equal => {
                if expecting_special && self.special.is_none() {
                    self.special = Some(target);
                    Some(true)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => None,
        }
    }
}

#[derive(Default)]
struct GridCandidateCounts {
    row_counts: [HouseCandidateCounts; 9],
    col_counts: [HouseCandidateCounts; 9],
    box_counts: [HouseCandidateCounts; 9],
    tri_value_cell: Option<(usize, usize)>,
    special: Option<usize>,
}

impl GridCandidateCounts {
    pub fn check_special(&mut self, special: usize) -> Option<()> {
        if self.special.is_none() {
            self.special = Some(special);
            Some(())
        } else if matches!(self.special, Some(special)) {
            Some(())
        } else {
            None
        }
    }
    /// update accumulator by given cell
    /// returns true if successfully updated
    /// returns false if the grid does not match BUG+1 pattern
    pub fn update(&mut self, x: usize, y: usize, candidates: BitMap) -> Option<()> {
        let count = candidates.count();
        if count > 3 {
            return None;
        }
        if count == 3 {
            if self.tri_value_cell.is_none() {
                self.tri_value_cell = Some((x, y));
            } else {
                return None;
            }
        }
        let box_id = Coord::get_box_id(x, y);
        let row_expecting = self.tri_value_cell.map(|(sx, _)| sx == x).unwrap_or(false);
        let col_expecting = self.tri_value_cell.map(|(_, sy)| sy == y).unwrap_or(false);
        let box_expecting = self
            .tri_value_cell
            .map(|(sx, sy)| box_id == Coord::get_box_id(sx, sy))
            .unwrap_or(false);
        for target in candidates.iter_ones() {
            if self.row_counts[x].add(target, row_expecting)? {
                self.check_special(target)?;
            }
            if self.col_counts[y].add(target, col_expecting)? {
                self.check_special(target)?;
            }
            if self.box_counts[y].add(target, box_expecting)? {
                self.check_special(target)?;
            }
        }
        Some(())
    }
    pub fn get_outlier(&self) -> Option<(usize, usize, usize)> {
        self.tri_value_cell
            .and_then(|(x, y)| self.special.map(|special| (x, y, special)))
    }
}

impl Solver for BiValueUniversalGravePlusOne {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Coord::all_cells()
            .filter_map(|(x, y)| {
                game_board
                    .get_candidates(x, y)
                    .map(|candidate| (x, y, candidate))
            })
            .try_fold(
                GridCandidateCounts::default(),
                |mut acc, (x, y, candidates)| acc.update(x, y, candidates).map(|()| acc),
            )
            .and_then(|counter| counter.get_outlier())
            .map(|(x, y, target)| Solution {
                actions: vec![Action::Confirmation(ConfirmationDetails { x, y, target })],
                house_clues: vec![],
                candidate_clues: vec![],
                solver_id: self.id,
            })
    }
}
