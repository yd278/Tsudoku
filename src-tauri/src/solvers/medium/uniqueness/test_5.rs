use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House},
};

use super::{iter_valid_bi_value, valid_unique_rectangle_cell, BiValueCell, UniquenessTest5};

#[derive(Clone, Copy)]
struct BaseRow {
    x: usize,
    py: usize,
    qy: usize,
    bi_value: BitMap,
    clue_candidates: BitMap,
    extra_candidate: BitMap,
}
impl BaseRow {
    pub fn from_p_and_q(
        p: BiValueCell,
        qy: usize,
        clue_candidates: BitMap,
        extra_candidate: BitMap,
    ) -> Self {
        Self {
            x: p.x,
            py: p.y,
            qy,
            bi_value: p.bi_value,
            clue_candidates,
            extra_candidate,
        }
    }
}

#[derive(Clone, Copy)]
struct UR5 {
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
    bi_value: BitMap,
    clue_candidates_q: BitMap,
    clue_candidates_r: BitMap,
    clue_candidates_s: BitMap,
    extra_candidate: BitMap,
    third_pincer: bool,
}
impl UR5 {
    pub fn from_row_r_s(
        row: BaseRow,
        sx: usize,
        clue_candidates_r: BitMap,
        clue_candidates_s: BitMap,
        third_pincer: bool,
    ) -> Self {
        Self {
            px: row.x,
            py: row.py,
            sx,
            sy: row.qy,
            bi_value: row.bi_value,
            clue_candidates_q: row.clue_candidates,
            clue_candidates_r,
            clue_candidates_s,
            extra_candidate: row.extra_candidate,
            third_pincer,
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
        if self.third_pincer {
            vec![
                Candidate::new(self.px, self.py, self.bi_value),
                Candidate::new(self.px, self.sy, self.clue_candidates_q),
                Candidate::new(self.sx, self.py, self.clue_candidates_r),
                Candidate::new(self.sx, self.sy, self.clue_candidates_s),
                Candidate::new(self.px, self.sy, self.extra_candidate),
                Candidate::new(self.sx, self.py, self.extra_candidate),
                Candidate::new(self.sx, self.sy, self.extra_candidate),
            ]
        } else {
            vec![
                Candidate::new(self.px, self.py, self.bi_value),
                Candidate::new(self.px, self.sy, self.clue_candidates_q),
                Candidate::new(self.sx, self.py, self.clue_candidates_r),
                Candidate::new(self.sx, self.sy, self.clue_candidates_s),
                Candidate::new(self.px, self.sy, self.extra_candidate),
                Candidate::new(self.sx, self.py, self.extra_candidate),
            ]
        }
    }
}

impl UniquenessTest5 {
    ///returns Iterator BaseRow
    fn iter_valid_base_row(
        game_board: &GameBoard,
        bi_value_cell: BiValueCell,
    ) -> impl Iterator<Item = BaseRow> + '_ {
        (0..9)
            .filter(move |&qy| qy != bi_value_cell.y)
            .filter_map(move |qy| {
                valid_unique_rectangle_cell(game_board, bi_value_cell.x, qy, bi_value_cell.bi_value)
                    .and_then(|(clue_candidates, extra_candidates)| {
                        (extra_candidates.count() == 1).then_some(BaseRow::from_p_and_q(
                            bi_value_cell,
                            qy,
                            clue_candidates,
                            extra_candidates,
                        ))
                    })
            })
    }

    fn iter_valid_rectangle(
        game_board: &GameBoard,
        base_row: BaseRow,
    ) -> impl Iterator<Item = UR5> + '_ {
        (0..9)
            .filter(move |&rx| rx != base_row.x)
            .filter_map(move |rx| {
                valid_unique_rectangle_cell(game_board, rx, base_row.py, base_row.bi_value).map(
                    |(clue_candidates_r, extra_candidates_r)| {
                        (extra_candidates_r == base_row.extra_candidate).then(|| {
                            valid_unique_rectangle_cell(
                                game_board,
                                rx,
                                base_row.qy,
                                base_row.bi_value,
                            )
                            .map(
                                |(clue_candidates_s, extra_candidates_s)| {
                                    (extra_candidates_r.union(&extra_candidates_s)
                                        == extra_candidates_r)
                                        .then_some(UR5::from_row_r_s(
                                            base_row,
                                            rx,
                                            clue_candidates_r,
                                            clue_candidates_s,
                                            extra_candidates_s.count() == 1,
                                        ))
                                },
                            )
                        })
                    },
                )
            })
            .flatten()
            .flatten()
            .flatten()
    }
    fn get_eliminables(game_board: &GameBoard, ur: UR5) -> Vec<Action> {
        let rx = ur.sx;
        let ry = ur.py;
        let qx = ur.px;
        let qy = ur.sy;
        let target = ur.extra_candidate.trailing_zeros();
        Coord::pinched_by(rx, ry, qx, qy)
            .filter(|&(cx, cy)| !ur.third_pincer || Coord::sees(cx, cy, ur.sx, ur.sy))
            .filter_map(|(cx, cy)| {
                game_board
                    .contains_candidate(cx, cy, target)
                    .then_some(Action::Elimination(EliminationDetails {
                        x: cx,
                        y: cy,
                        target: ur.extra_candidate,
                    }))
            })
            .collect()
    }
}
impl Solver for UniquenessTest5 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|bi_value_cell| Self::iter_valid_base_row(game_board, bi_value_cell))
            .flat_map(|base_row| Self::iter_valid_rectangle(game_board, base_row))
            .find_map(|ur| {
                let actions = Self::get_eliminables(game_board, ur);
                (!actions.is_empty()).then_some(Solution {
                    actions,
                    house_clues: ur.get_house_clues(),
                    candidate_clues: ur.get_candi_clues(),
                    solver_id: self.id,
                })
            })
    }
}
