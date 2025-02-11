use crate::{
    game_board::GameBoard,
    solvers::{
        medium::{iter_valid_bi_value, BiValueCell},
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::XYWing;

#[derive(Clone, Copy)]
struct XPincer {
    px: usize,
    py: usize,
    qx: usize,
    qy: usize,
    x: usize,
    y: usize,
    z: usize,
}
impl XPincer {
    fn try_from_p_q(p: BiValueCell, q: BiValueCell) -> Option<Self> {
        let x_map = p.bi_value.intersect(q.bi_value);
        (x_map.count() == 1).then(|| Self {
            px: p.x,
            py: p.y,
            qx: q.x,
            qy: q.y,
            x: x_map.trailing_zeros(),
            y: p.bi_value.difference(q.bi_value).trailing_zeros(),
            z: q.bi_value.difference(p.bi_value).trailing_zeros(),
        })
    }
}
struct XYPincers {
    px: usize,
    py: usize,
    qx: usize,
    qy: usize,
    rx: usize,
    ry: usize,
    x: usize,
    y: usize,
    z: usize,
}

impl XYPincers {
    fn try_from_p_q_r(pq: XPincer, r: BiValueCell) -> Option<Self> {
        (r.bi_value.contains(pq.y)
            && r.bi_value.contains(pq.z)
            && !Coord::sees(pq.qx, pq.qy, r.x, r.y)
            && (r.x != pq.qx || r.y != pq.qy))
            .then_some(Self {
                px: pq.px,
                py: pq.py,
                qx: pq.qx,
                qy: pq.qy,
                rx: r.x,
                ry: r.y,
                x: pq.x,
                y: pq.y,
                z: pq.z,
            })
    }

    fn get_actions(&self, game_board: &GameBoard) -> Vec<Action> {
        Coord::pinched_by(self.qx, self.qy, self.rx, self.ry)
            .filter(|&(cx, cy)| !Coord::same(cx, cy, self.px, self.py))
            .filter_map(|(cx, cy)| {
                game_board.get_candidates(cx, cy).and_then(|candidates| {
                    candidates
                        .contains(self.z)
                        .then_some(Action::Elimination(EliminationDetails {
                            x: cx,
                            y: cy,
                            target: BitMap::from(self.z),
                        }))
                })
            })
            .collect()
    }
    fn try_get_solution(&self, game_board: &GameBoard, solver_id: usize) -> Option<Solution> {
        let actions = self.get_actions(game_board);
        (!actions.is_empty()).then(|| Solution {
            actions,
            house_clues: vec![],
            candidate_clues: vec![
                Candidate::new_single(self.px, self.py, self.x),
                Candidate::new_single(self.qx, self.qy, self.x),
                Candidate::new_single(self.px, self.py, self.y),
                Candidate::new_single(self.rx, self.ry, self.y),
                Candidate::new_single(self.qx, self.qy, self.z),
                Candidate::new_single(self.rx, self.ry, self.z),
            ],
            solver_id,
        })
    }
}

impl XYWing {
    fn iter_x_pincer(game_board: &GameBoard, p: BiValueCell) -> impl Iterator<Item = XPincer> + '_ {
        Coord::seeable_cells(p.x, p.y)
            .filter_map(|(qx, qy)| {
                game_board
                    .get_candidates(qx, qy)
                    .map(|candidates| (qx, qy, candidates))
            })
            .filter_map(|(x, y, bi_value)| BiValueCell::try_new(x, y, bi_value))
            .filter_map(move |q| XPincer::try_from_p_q(p, q))
    }
    fn iter_xy_pincer(game_board: &GameBoard, pq: XPincer) -> impl Iterator<Item = XYPincers> + '_ {
        Coord::seeable_cells(pq.px, pq.py)
            .filter_map(|(rx, ry)| {
                game_board
                    .get_candidates(rx, ry)
                    .map(|candidates| (rx, ry, candidates))
            })
            .filter_map(|(x, y, bi_value)| BiValueCell::try_new(x, y, bi_value))
            .filter_map(move |r| XYPincers::try_from_p_q_r(pq, r))
    }
}
impl Solver for XYWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|p| Self::iter_x_pincer(game_board, p))
            .flat_map(|pq| Self::iter_xy_pincer(game_board, pq))
            .find_map(|xy_wing| xy_wing.try_get_solution(game_board, self.id))
    }
}
