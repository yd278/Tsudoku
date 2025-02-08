use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, ConfirmationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::BiValueUniversalGravePlusOne;

enum InvalidBUG {
    AppearanceLimitExceeded,
    MultipleOutlierCandidates,
    CandidatesLimitExceeded,
    MultipleTriValueCells,
    MismatchedOutliers,
}

#[derive(Default)]
struct HouseCandidateCounts {
    count: [usize; 9],
    outlier_flag: Option<usize>,
}
impl HouseCandidateCounts {
    pub fn add(&mut self, candidate: usize) -> Result<Option<usize>, InvalidBUG> {
        self.count[candidate] += 1;
        match self.count[candidate].cmp(&3) {
            std::cmp::Ordering::Greater => Err(InvalidBUG::AppearanceLimitExceeded),
            std::cmp::Ordering::Equal => {
                if self.outlier_flag.is_some() {
                    Err(InvalidBUG::MultipleOutlierCandidates)
                } else {
                    self.outlier_flag = Some(candidate);
                    Ok(self.outlier_flag)
                }
            }
            std::cmp::Ordering::Less => Ok(None),
        }
    }
}

#[derive(Default)]
struct GridCandidateCounts {
    row_counts: [HouseCandidateCounts; 9],
    col_counts: [HouseCandidateCounts; 9],
    box_counts: [HouseCandidateCounts; 9],
    outlier: Option<(usize, usize, usize)>,
    tri_value_flag: bool,
}

impl GridCandidateCounts {
    pub fn add_candidate(
        &mut self,
        x: usize,
        y: usize,
        candidate: usize,
    ) -> Result<Option<(usize, usize, usize)>, InvalidBUG> {
        match (
            self.row_counts[x].add(candidate)?,
            self.col_counts[y].add(candidate)?,
            self.box_counts[Coord::get_box_id(x, y)].add(candidate)?,
        ) {
            (Some(_), Some(_), Some(_)) => {
                if self.outlier.is_none() {
                    self.outlier = Some((x, y, candidate));
                    Ok(self.outlier)
                } else {
                    Err(InvalidBUG::MultipleOutlierCandidates)
                }
            }
            (None, None, None) => Ok(None),
            _ => Err(InvalidBUG::MismatchedOutliers),
        }
    }
    pub fn update(
        &mut self,
        x: usize,
        y: usize,
        candidates: BitMap,
    ) -> Result<Option<(usize, usize, usize)>, InvalidBUG> {
        let count = candidates.count();
        if count > 3 {
            return Err(InvalidBUG::CandidatesLimitExceeded);
        }
        if count == 3 {
            if self.tri_value_flag {
                return Err(InvalidBUG::MultipleTriValueCells);
            } else {
                self.tri_value_flag = true;
            }
        }
        for candidate in candidates.iter_ones() {
            self.add_candidate(x, y, candidate)?;
        }
        Ok(self.outlier)
    }
    pub fn get_outlier(&self) -> Option<(usize, usize, usize)> {
        self.outlier
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
                |mut acc, (x, y, candidates)| {
                    acc.update(x, y, candidates)?;
                    Ok::<GridCandidateCounts, InvalidBUG>(acc)
                },
            )
            .ok()?
            .get_outlier()
            .map(|(x, y, target)| Solution {
                actions: vec![Action::Confirmation(ConfirmationDetails { x, y, target })],
                house_clues: vec![],
                candidate_clues: vec![],
                solver_id: self.id,
            })
    }
}
