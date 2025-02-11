use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, HouseType},
};

use super::WWing;

#[derive(Clone, Copy)]
struct Bridge {
    target: usize,
    ux: usize,
    uy: usize,
    vx: usize,
    vy: usize,
}

impl WWing {
    fn iter_bridge(game_board: &GameBoard) -> impl Iterator<Item = Bridge> + '_ {
        Coord::all_cells()
            .flat_map(|(x, y)| (0..9).map(move |target| (x, y, target)))
            .flat_map(|(x, y, target)| {
                (0..3).map(move |dim| (x, y, target, HouseType::from_dim(dim)))
            })
            .filter_map(|(x, y, target, dim)| {
                game_board
                    .get_hard_link(x, y, target, dim)
                    .and_then(|(ox, oy)| {
                        (ox >= x && oy >= y).then_some(Bridge {
                            ux: x,
                            uy: y,
                            vx: ox,
                            vy: y,
                            target,
                        })
                    })
            })
    }

    fn iter_p(game_board: &GameBoard, bridge: Bridge) -> impl Iterator<Item = PincerP> + '_ {
        Coord::seeable_cells(bridge.ux, bridge.uy)
            .filter_map(|(px, py)| {
                game_board
                    .get_candidates(px, py)
                    .map(|candidates| (px, py, candidates))
            })
            .filter_map(move |(px, py, candidates)| {
                PincerP::try_from_bridge_p(bridge, px, py, candidates)
            })
    }

    fn iter_q(
        game_board: &GameBoard,
        pincer_p: PincerP,
    ) -> impl Iterator<Item = WWingPattern> + '_ {
        Coord::seeable_cells(pincer_p.bridge.vx, pincer_p.bridge.vx)
            .filter_map(|(qx, qy)| {
                game_board
                    .get_candidates(qx, qy)
                    .map(|candidates| (qx, qy, candidates))
            })
            .filter_map(move |(qx, qy, candidates)| {
                WWingPattern::try_from_p_q(pincer_p, qx, qy, candidates)
            })
    }
}
#[derive(Clone, Copy)]
struct PincerP {
    bridge: Bridge,
    px: usize,
    py: usize,
    other: usize,
}

impl PincerP {
    fn try_from_bridge_p(bridge: Bridge, px: usize, py: usize, candidates: BitMap) -> Option<Self> {
        (candidates.count() == 2 && candidates.contains(bridge.target)).then_some(Self {
            bridge,
            px,
            py,
            other: candidates
                .difference(BitMap::from(bridge.target))
                .trailing_zeros(),
        })
    }
}

struct WWingPattern {
    pincer_p: PincerP,
    qx: usize,
    qy: usize,
}

impl WWingPattern {
    fn try_from_p_q(pincer_p: PincerP, qx: usize, qy: usize, candidates: BitMap) -> Option<Self> {
        (candidates.count() == 2
            && candidates.contains(pincer_p.bridge.target)
            && candidates.contains(pincer_p.other))
        .then_some(Self { pincer_p, qx, qy })
    }

    fn get_actions(&self, game_board: &GameBoard) -> Vec<Action> {
        Coord::pinched_by(self.pincer_p.px, self.pincer_p.py, self.qx, self.qy)
            .filter_map(|(cx, cy)| {
                game_board.get_candidates(cx, cy).and_then(|candidates| {
                    candidates
                        .contains(self.pincer_p.other)
                        .then_some(Action::Elimination(EliminationDetails {
                            x: cx,
                            y: cy,
                            target: BitMap::from(self.pincer_p.other),
                        }))
                })
            })
            .collect()
    }
    fn try_get_solution(&self, game_board: &GameBoard, solver_id: usize) -> Option<Solution> {
        let actions = self.get_actions(game_board);
        (!actions.is_empty()).then_some(Solution {
            actions,
            house_clues: vec![],
            candidate_clues: vec![
                Candidate::new_single(
                    self.pincer_p.bridge.ux,
                    self.pincer_p.bridge.uy,
                    self.pincer_p.bridge.target,
                ),
                Candidate::new_single(
                    self.pincer_p.bridge.vx,
                    self.pincer_p.bridge.vy,
                    self.pincer_p.bridge.target,
                ),
                Candidate::new_single(
                    self.pincer_p.px,
                    self.pincer_p.py,
                    self.pincer_p.bridge.target,
                ),
                Candidate::new_single(self.pincer_p.px, self.pincer_p.py, self.pincer_p.other),
                Candidate::new_single(self.qx, self.qy, self.pincer_p.bridge.target),
                Candidate::new_single(self.qx, self.qy, self.pincer_p.other),
            ],
            solver_id,
        })
    }
}

impl Solver for WWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Self::iter_bridge(game_board)
            .flat_map(|bridge| Self::iter_p(game_board, bridge))
            .flat_map(|pincer_p| Self::iter_q(game_board, pincer_p))
            .find_map(|w_wing| w_wing.try_get_solution(game_board, self.id))
    }
}
