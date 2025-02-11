use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House, HouseType},
};

use super::{iter_pen_cell, AvoidableRectangle2, PenCell};

impl PenCell {
    pub fn get_house_id(&self, dim: HouseType) -> usize {
        Coord::components_proj(self.x, self.y, dim.as_dim())
    }
    pub fn get_cell_id(&self, dim: HouseType) -> usize {
        Coord::components_proj(self.x, self.y, dim.other().as_dim())
    }

    pub fn get_base_house(&self, base_dim: usize) -> House {
        House::from_dim_id(base_dim, self.get_house_id(HouseType::from_dim(base_dim)))
    }
}
#[derive(Copy, Clone)]
struct BaseHouse {
    house: House,
    pi: usize,
    qi: usize,
    pv: usize,
    qv: usize,
}

impl BaseHouse {
    pub fn new(p: PenCell, dim: HouseType, qi: usize, qv: usize) -> Self {
        Self {
            house: dim.house(p.get_house_id(dim)),
            pi: p.get_cell_id(dim),
            qi,
            pv: p.target,
            qv,
        }
    }
}

struct AR2 {
    base_house: House,
    span_house: House,
    pi: usize,
    si: usize,
    pv: usize,
    qv: usize,
    clue: usize,
}

impl AR2 {
    pub fn new(base: BaseHouse, span_house: House, clue: usize) -> Self {
        Self {
            base_house: base.house,
            span_house,
            pi: base.pi,
            si: base.qi,
            pv: base.pv,
            qv: base.qv,
            clue,
        }
    }
    pub fn get_actions(&self, game_board: &GameBoard) -> Vec<Action> {
        let (rx, ry) = Coord::from_house_and_index(&self.span_house, self.pi);
        let (sx, sy) = Coord::from_house_and_index(&self.span_house, self.si);
        Coord::pinched_by(rx, ry, sx, sy)
            .filter_map(|(cx, cy)| {
                game_board
                    .contains_candidate(cx, cy, self.clue)
                    .then_some(Action::Elimination(EliminationDetails {
                        x: cx,
                        y: cy,
                        target: BitMap::from(self.clue),
                    }))
            })
            .collect()
    }
    pub fn get_house_clues(&self) -> Vec<House> {
        vec![
            self.base_house,
            self.span_house,
            self.base_house.get_perpendicular(self.pi),
            self.base_house.get_perpendicular(self.si),
        ]
    }

    pub fn get_candidate_clues(&self) -> Vec<Candidate> {
        vec![
            Candidate::from_coord_single(
                Coord::from_house_and_index(&self.span_house, self.pi),
                self.qv,
            ),
            Candidate::from_coord_single(
                Coord::from_house_and_index(&self.span_house, self.si),
                self.pv,
            ),
            Candidate::from_coord_single(
                Coord::from_house_and_index(&self.span_house, self.pi),
                self.clue,
            ),
            Candidate::from_coord_single(
                Coord::from_house_and_index(&self.span_house, self.si),
                self.clue,
            ),
        ]
    }

    pub fn try_get_solution(&self, game_board: &GameBoard, solver_id: usize) -> Option<Solution> {
        let actions = self.get_actions(game_board);
        (!actions.is_empty()).then(|| Solution {
            actions,
            house_clues: self.get_house_clues(),
            candidate_clues: self.get_candidate_clues(),
            solver_id,
        })
    }
}
impl AvoidableRectangle2 {
    fn iter_q(game_board: &GameBoard, p: PenCell) -> impl Iterator<Item = BaseHouse> + '_ {
        (0..2).flat_map(move |dim| {
            let base_house = p.get_base_house(dim);
            let pi = p.get_cell_id(HouseType::from_dim(dim));
            (0..9).filter(move |&qi| qi != pi).filter_map(move |qi| {
                let (qx, qy) = base_house.ith_cell(qi);
                game_board
                    .get_pen_mark(qx, qy)
                    .map(|qv| BaseHouse::new(p, HouseType::from_dim(dim), qi, qv))
            })
        })
    }

    fn try_extra_candidate(
        game_board: &GameBoard,
        x: usize,
        y: usize,
        target: usize,
    ) -> Option<usize> {
        game_board.get_candidates(x, y).and_then(|candidates| {
            (candidates.count() == 2 && candidates.contains(target))
                .then(|| candidates.difference(BitMap::from(target)).trailing_zeros())
        })
    }

    fn iter_ar(game_board: &GameBoard, house: BaseHouse) -> impl Iterator<Item = AR2> + '_ {
        (0..9)
            .filter(move |&span| {
                let base = house.house.get_index();
                span != base && (span / 3 == base / 3) != (house.pi / 3 == house.qi / 3)
            })
            .filter_map(move |span| {
                let span_house = house.house.get_parallel(span);
                let (rx, ry) = span_house.ith_cell(house.pi);
                let (sx, sy) = span_house.ith_cell(house.qi);
                Self::try_extra_candidate(game_board, rx, ry, house.qv)
                    .and_then(|r_clue| {
                        Self::try_extra_candidate(game_board, sx, sy, house.pv).map(|s_clue| {
                            (s_clue == r_clue).then_some(AR2::new(house, span_house, r_clue))
                        })
                    })
                    .flatten()
            })
    }
}
impl Solver for AvoidableRectangle2 {
    fn solve(
        &self,
        game_board: &crate::game_board::GameBoard,
    ) -> Option<crate::solvers::solution::Solution> {
        iter_pen_cell(game_board)
            .flat_map(|p| Self::iter_q(game_board, p))
            .flat_map(|house| Self::iter_ar(game_board, house))
            .find_map(|ar| ar.try_get_solution(game_board, self.id))
    }
}
