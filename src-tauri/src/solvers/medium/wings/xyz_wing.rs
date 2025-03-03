use crate::{
    game_board::GameBoard,
    solvers::{
        medium::{iter_valid_bi_value, BiValueCell},
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord},
};

use super::XYZWing;
#[derive(Clone, Copy)]
struct Pincers {
    qx: usize,
    qy: usize,
    rx: usize,
    ry: usize,
    x: usize,
    y: usize,
    z: usize,
}

impl Pincers {
    fn try_get_candidates(qc: BitMap, rc: BitMap) -> Option<(usize, usize, usize)> {
        let intersect = qc.intersect(rc);
        (intersect.count() == 1).then(|| {
            (
                qc.difference(rc).trailing_zeros(),
                rc.difference(qc).trailing_zeros(),
                intersect.trailing_zeros(),
            )
        })
    }

    fn try_from_q_r(q: BiValueCell, r: BiValueCell) -> Option<Self> {
        Self::try_get_candidates(q.bi_value, r.bi_value).map(|(x, y, z)| Self {
            qx: q.x,
            qy: q.y,
            rx: r.x,
            ry: r.y,
            x,
            y,
            z,
        })
    }
}
struct XYZWingPattern {
    pincers: Pincers,
    px: usize,
    py: usize,
}
impl XYZWingPattern {
    fn try_from_pincers_p(
        pincers: Pincers,
        px: usize,
        py: usize,
        p_candidates: BitMap,
    ) -> Option<Self> {
        (p_candidates.count() == 3
            && p_candidates.contains(pincers.x)
            && p_candidates.contains(pincers.y)
            && p_candidates.contains(pincers.z))
        .then_some(Self { pincers, px, py })
    }

    fn get_actions(&self, game_board: &GameBoard) -> Vec<Action> {
        Coord::pinched_by(
            self.pincers.qx,
            self.pincers.qy,
            self.pincers.rx,
            self.pincers.ry,
        )
        .filter(|&(x, y)| Coord::sees(x, y, self.px, self.py))
        .filter_map(|(x, y)| {
            game_board.get_candidates(x, y).and_then(|candidates| {
                candidates
                    .contains(self.pincers.z)
                    .then_some(Action::Elimination(EliminationDetails {
                        x,
                        y,
                        target: BitMap::from(self.pincers.z),
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
                Candidate::new_single(self.px, self.py, self.pincers.x),
                Candidate::new_single(self.pincers.qx, self.pincers.qy, self.pincers.x),
                Candidate::new_single(self.px, self.py, self.pincers.y),
                Candidate::new_single(self.pincers.rx, self.pincers.ry, self.pincers.y),
                Candidate::new_single(self.px, self.py, self.pincers.z),
                Candidate::new_single(self.pincers.qx, self.pincers.qy, self.pincers.z),
                Candidate::new_single(self.pincers.rx, self.pincers.ry, self.pincers.z),
            ],
            solver_id,
        })
    }
}
impl XYZWing {
    fn check_position(qx: usize, qy: usize, rx: usize, ry: usize) -> bool {
        !Coord::sees(qx, qy, rx, ry) && (rx / 3 == qx / 3 || ry / 3 == qy / 3)
    }
    fn iter_pincers_from_q(
        game_board: &GameBoard,
        q: BiValueCell,
    ) -> impl Iterator<Item = Pincers> + '_ {
        Coord::all_cells()
            .filter(move |&(rx, ry)| Self::check_position(q.x, q.y, rx, ry))
            .filter_map(|(x, y)| {
                game_board
                    .get_candidates(x, y)
                    .and_then(|candidates| BiValueCell::try_new(x, y, candidates))
            })
            .filter_map(move |r| Pincers::try_from_q_r(q, r))
    }

    fn check_pattern(
        game_board: &GameBoard,
        pincers: Pincers,
        px: usize,
        py: usize,
    ) -> Option<XYZWingPattern> {
        game_board.get_candidates(px, py).and_then(|p_candidates| {
            XYZWingPattern::try_from_pincers_p(pincers, px, py, p_candidates)
        })
    }
    fn find_xyz_wing(game_board: &GameBoard, pincers: Pincers) -> Option<XYZWingPattern> {
        Coord::pinched_by(pincers.qx, pincers.qy, pincers.rx, pincers.ry)
            .find_map(|(px, py)| Self::check_pattern(game_board, pincers, px, py))
    }
}
impl Solver for XYZWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_valid_bi_value(game_board)
            .flat_map(|q| Self::iter_pincers_from_q(game_board, q))
            .filter_map(|pincers| Self::find_xyz_wing(game_board, pincers))
            .find_map(|xyz_wing| xyz_wing.try_get_solution(game_board, self.id))
    }
}
