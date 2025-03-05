use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver, SolverIdentifier,
    },
    utils::{BitMap, House, HouseType},
};

use super::{iter_valid_bi_value, valid_unique_rectangle_cell, BiValueCell, HiddenRectangle};
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
struct HR {
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
    bi_value: BitMap,
    q_clue: BitMap,
    r_clue: BitMap,
    s_clue: BitMap,
}

impl HR {
    pub fn from_row_r_s(row: BaseRow, sx: usize, r_clue: BitMap, s_clue: BitMap) -> Self {
        Self {
            px: row.x,
            py: row.py,
            sx,
            sy: row.qy,
            bi_value: row.bi_value,
            q_clue: row.q_clue,
            r_clue,
            s_clue,
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
    pub fn get_actions(&self, clue: usize) -> Vec<Action> {
        vec![Action::Elimination(EliminationDetails {
            x: self.sx,
            y: self.sy,
            target: BitMap::from(clue),
        })]
    }
    pub fn get_candidate_clues(&self, target: usize, clue: usize) -> Vec<Candidate> {
        vec![
            Candidate::new(self.px, self.py, self.bi_value),
            Candidate::new(self.px, self.sy, self.q_clue),
            Candidate::new(self.sx, self.py, self.r_clue),
            Candidate::new_single(self.sx, self.sy, target),
        ]
    }
    pub fn get_solution(
        &self,
        target: usize,
        clue: usize,
        solver_id: SolverIdentifier,
    ) -> Solution {
        Solution {
            actions: self.get_actions(clue),
            house_clues: self.get_house_clues(),
            candidate_clues: self.get_candidate_clues(target, clue),
            solver_id,
        }
    }
}
impl HiddenRectangle {
    fn iter_q(p: BiValueCell, game_board: &GameBoard) -> impl Iterator<Item = BaseRow> + '_ {
        (0..9).filter(move |&qy| qy != p.y).filter_map(move |qy| {
            valid_unique_rectangle_cell(game_board, p.x, qy, p.bi_value)
                .map(|(q_clue, _)| BaseRow::new(p, qy, q_clue))
        })
    }

    fn iter_r(row: BaseRow, game_board: &GameBoard) -> impl Iterator<Item = HR> + '_ {
        (0..9)
            .filter(move |&rx| rx != row.x && (rx / 3 == row.x / 3) != (row.py / 3 == row.qy / 3))
            .filter_map(move |rx| {
                valid_unique_rectangle_cell(game_board, rx, row.py, row.bi_value).and_then(
                    |(r_clue, _)| {
                        game_board
                            .get_candidates(rx, row.qy)
                            .and_then(|candidates| {
                                valid_unique_rectangle_cell(game_board, rx, row.qy, row.bi_value)
                                    .map(|(s_clue, _)| HR::from_row_r_s(row, rx, r_clue, s_clue))
                            })
                    },
                )
            })
    }

    fn get_solution(&self, hr: HR, game_board: &GameBoard) -> Option<Solution> {
        hr.bi_value
            .iter_ones()
            .map(|target| {
                (
                    target,
                    hr.bi_value
                        .difference(BitMap::from(target))
                        .trailing_zeros(),
                )
            })
            .find_map(|(target, clue)| {
                game_board
                    .get_hard_link(hr.sx, hr.sy, target, HouseType::Row)
                    .and_then(|(_, ly)| {
                        (ly == hr.py)
                            .then(|| {
                                game_board
                                    .get_hard_link(hr.sx, hr.sy, target, HouseType::Col)
                                    .and_then(|(lx, _)| {
                                        (lx == hr.px).then(|| {
                                            hr.get_solution(target, clue, self.solver_id())
                                        })
                                    })
                            })
                            .flatten()
                    })
            })
    }
}
impl Solver for HiddenRectangle {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|bi_value_cell| Self::iter_q(bi_value_cell, game_board))
            .flat_map(|row| Self::iter_r(row, game_board))
            .find_map(|ur| self.get_solution(ur, game_board))
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::HiddenRectangle
    }
}
