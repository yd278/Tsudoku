use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, ConfirmationDetails, Solution},
        Solver, SolverIdentifier,
    },
    utils::{BitMap, House, HouseType},
};

use super::{iter_valid_bi_value, valid_unique_rectangle_cell, BiValueCell, UniquenessTest6};
#[derive(Clone, Copy)]

struct BaseRow {
    x: usize,
    py: usize,
    qy: usize,
    bi_value: BitMap,
    q_clue: BitMap,
}
impl BaseRow {
    fn new(p: BiValueCell, qy: usize, q_clue: BitMap) -> Self {
        Self {
            x: p.x,
            py: p.y,
            qy,
            bi_value: p.bi_value,
            q_clue,
        }
    }
}
struct UR6 {
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
    bi_value: BitMap,
    q_clue: BitMap,
    r_clue: BitMap,
}

impl UR6 {
    pub fn from_row_r_s(row: BaseRow, sx: usize, r_clue: BitMap) -> Self {
        Self {
            px: row.x,
            py: row.py,
            sx,
            sy: row.qy,
            bi_value: row.bi_value,
            q_clue: row.q_clue,
            r_clue,
        }
    }

    pub fn get_house_clues(&self) -> Vec<House> {
        vec![
            House::Row(self.px),
            House::Row(self.sx),
            House::Col(self.py),
            House::Col(self.sy),
        ]
    }
    pub fn get_actions(&self, target: usize) -> Vec<Action> {
        vec![
            Action::Confirmation(ConfirmationDetails {
                x: self.px,
                y: self.py,
                target,
            }),
            Action::Confirmation(ConfirmationDetails {
                x: self.sx,
                y: self.sy,
                target,
            }),
        ]
    }
    pub fn get_candidate_clues(&self, target: usize, clue: usize) -> Vec<Candidate> {
        vec![
            Candidate::new_single(self.px, self.py, clue),
            Candidate::new_single(self.sx, self.sy, clue),
            Candidate::new(self.px, self.sy, self.q_clue),
            Candidate::new(self.sx, self.py, self.r_clue),
        ]
    }
    pub fn get_solution(
        &self,
        target: usize,
        clue: usize,
        solver_id: SolverIdentifier,
    ) -> Solution {
        Solution {
            actions: self.get_actions(target),
            house_clues: self.get_house_clues(),
            candidate_clues: self.get_candidate_clues(target, clue),
            solver_id,
        }
    }
}
impl UniquenessTest6 {
    fn iter_q(p: BiValueCell, game_board: &GameBoard) -> impl Iterator<Item = BaseRow> + '_ {
        (0..9).filter(move |&qy| qy != p.y).filter_map(move |qy| {
            valid_unique_rectangle_cell(game_board, p.x, qy, p.bi_value).and_then(
                |(q_clue, q_extra)| (q_extra.count() > 0).then_some(BaseRow::new(p, qy, q_clue)),
            )
        })
    }

    fn iter_r(row: BaseRow, game_board: &GameBoard) -> impl Iterator<Item = UR6> + '_ {
        (0..9)
            .filter(move |&rx| rx != row.x && (rx / 3 == row.x / 3) != (row.py / 3 == row.qy / 3))
            .filter_map(move |rx| {
                valid_unique_rectangle_cell(game_board, rx, row.py, row.bi_value)
                    .and_then(|(r_clue, r_extra)| {
                        (r_extra.count() > 0).then(|| {
                            game_board
                                .get_candidates(rx, row.qy)
                                .and_then(|candidates| {
                                    (candidates == row.bi_value)
                                        .then_some(UR6::from_row_r_s(row, rx, r_clue))
                                })
                        })
                    })
                    .flatten()
            })
    }
    fn get_solution(&self, ur: UR6, game_board: &GameBoard) -> Option<Solution> {
        ur.bi_value
            .iter_ones()
            .map(|target| {
                (
                    target,
                    ur.bi_value
                        .difference(BitMap::from(target))
                        .trailing_zeros(),
                )
            })
            .find_map(|(target, clue)| {
                game_board
                    .get_hard_link(ur.px, ur.py, target, HouseType::Row)
                    .and_then(|(_, ly)| {
                        (ly == ur.sy)
                            .then(|| {
                                game_board
                                    .get_hard_link(ur.px, ur.py, target, HouseType::Col)
                                    .and_then(|(lx, _)| {
                                        (lx == ur.sx)
                                            .then(|| {
                                                game_board
                                                    .get_hard_link(
                                                        ur.sx,
                                                        ur.sy,
                                                        target,
                                                        HouseType::Row,
                                                    )
                                                    .and_then(|(_, ly)| {
                                                        (ly == ur.py).then(|| {
                                                            game_board
                                                                .get_hard_link(
                                                                    ur.sx,
                                                                    ur.sy,
                                                                    target,
                                                                    HouseType::Col,
                                                                )
                                                                .and_then(|(lx, _)| {
                                                                    (lx == ur.px).then(|| {
                                                                        ur.get_solution(
                                                                            target,
                                                                            clue,
                                                                            self.solver_id(),
                                                                        )
                                                                    })
                                                                })
                                                        })
                                                    })
                                                    .flatten()
                                            })
                                            .flatten()
                                    })
                            })
                            .flatten()
                    })
            })
    }
}
impl Solver for UniquenessTest6 {
    fn solve(&self, game_board: &GameBoard) -> Option<crate::solvers::solution::Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|bi_value_cell| Self::iter_q(bi_value_cell, game_board))
            .flat_map(|row| Self::iter_r(row, game_board))
            .find_map(|ur| self.get_solution(ur, game_board))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::UniquenessTest6
    }
}
