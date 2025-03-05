use crate::{
    game_board::GameBoard,
    solvers::{
        medium::uniqueness::iter_valid_bi_value,
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver, SolverIdentifier,
    },
    utils::{BitMap, House},
};

use super::{valid_unique_rectangle_cell, BiValueCell, UniquenessTest1};
#[derive(Clone, Copy)]
struct BaseRow {
    x: usize,
    py: usize,
    qy: usize,
    bi_value: BitMap,
}
impl BaseRow {
    pub fn from_p_and_q(p: BiValueCell, qy: usize) -> Self {
        Self {
            x: p.x,
            py: p.y,
            qy,
            bi_value: p.bi_value,
        }
    }
}

#[derive(Clone, Copy)]
struct UR1 {
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
    bi_value: BitMap,
    clue_candidates: BitMap,
    extra_candidates: BitMap,
}
impl UR1 {
    pub fn from_row_r_s(
        row: BaseRow,
        sx: usize,
        clue_candidates: BitMap,
        extra_candidates: BitMap,
    ) -> Self {
        Self {
            px: row.x,
            py: row.py,
            sx,
            sy: row.qy,
            bi_value: row.bi_value,
            clue_candidates,
            extra_candidates,
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
    pub fn get_candi_clues(&self) -> Vec<Candidate> {
        vec![
            Candidate::new(self.px, self.py, self.bi_value),
            Candidate::new(self.px, self.sy, self.bi_value),
            Candidate::new(self.sx, self.py, self.bi_value),
        ]
    }
    pub fn get_solution(&self, solver_id: SolverIdentifier) -> Solution {
        Solution {
            actions: vec![Action::Elimination(EliminationDetails {
                x: self.sx,
                y: self.sy,
                target: self.clue_candidates,
            })],
            house_clues: self.get_house_clues(),
            candidate_clues: self.get_candi_clues(),
            solver_id,
        }
    }
}

impl UniquenessTest1 {
    ///returns qy for a valid bi-value cell at (px, qy)
    fn iter_valid_base_row(
        game_board: &GameBoard,
        p: BiValueCell,
    ) -> impl Iterator<Item = BaseRow> + '_ {
        (0..9).filter(move |&qy| qy != p.y).filter_map(move |qy| {
            game_board.get_candidates(p.x, qy).and_then(|candidate| {
                (candidate == p.bi_value).then_some(BaseRow::from_p_and_q(p, qy))
            })
        })
    }

    fn iter_valid_rectangle(
        game_board: &GameBoard,
        base_row: BaseRow,
    ) -> impl Iterator<Item = UR1> + '_ {
        (0..9)
            .filter(move |&rx| {
                rx != base_row.x
                    && (rx / 3 == base_row.x / 3) != (base_row.py / 3 == base_row.qy / 3)
            })
            .filter_map(move |rx| {
                game_board
                    .get_candidates(rx, base_row.py)
                    .and_then(|candidate| {
                        (candidate == base_row.bi_value).then(|| {
                            valid_unique_rectangle_cell(
                                game_board,
                                rx,
                                base_row.qy,
                                base_row.bi_value,
                            )
                            .map(
                                |(clue_candidates, extra_candidates)| {
                                    UR1::from_row_r_s(
                                        base_row,
                                        rx,
                                        clue_candidates,
                                        extra_candidates,
                                    )
                                },
                            )
                        })
                    })
            })
            .flatten()
    }
}
impl Solver for UniquenessTest1 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|p| Self::iter_valid_base_row(game_board, p))
            .flat_map(|base_row| Self::iter_valid_rectangle(game_board, base_row))
            .map(|ur| ur.get_solution(self.solver_id()))
            .next()
    }

    fn solver_id(&self) -> SolverIdentifier {
        SolverIdentifier::UniquenessTest1
    }
}
