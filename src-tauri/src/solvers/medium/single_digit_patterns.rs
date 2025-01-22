use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, HouseType},
};

fn check_turbot(
    game_board: &GameBoard,
    soft_dim: usize,
    hard1: usize,
    hard2: usize,
    solver_id: usize,
) -> Option<Solution> {
    for target in 0..9 {
        for soft_house_index in 0..9 {
            let appearance: Vec<_> =
                Coord::house(&HouseType::from_index(soft_dim).house(soft_house_index))
                    .filter(|&(x, y)| game_board.contains_candidate(x, y, target))
                    .collect();
            for i in 0..appearance.len() {
                let (x1, y1) = appearance[i];
                if let Some((p1, q1)) =
                    game_board.get_hard_link(x1, y1, target, HouseType::from_index(hard1))
                {
                    for (j, &(x2, y2)) in appearance.iter().enumerate() {
                        if j == i {
                            continue;
                        }
                        if let Some((p2, q2)) =
                            game_board.get_hard_link(x2, y2, target, HouseType::from_index(hard2))
                        {
                            let actions: Vec<_> = Coord::all_cells()
                                .filter(|&(u, v)| Coord::sees(p1, q1, u, v))
                                .filter(|&(u, v)| Coord::sees(p2, q2, u, v))
                                .filter(|&(u, v)| (u != x1 || v != y1) && (u != x2 || v != y2))
                                .map(|(u, v)| {
                                    Action::Elimination(EliminationDetails {
                                        x: u,
                                        y: v,
                                        target: BitMap::from(target),
                                    })
                                })
                                .collect();

                            if !actions.is_empty() {
                                return Some(Solution {
                                    actions,
                                    house_clues: vec![
                                        HouseType::from_index(soft_dim).house(soft_house_index),
                                        HouseType::from_index(hard1)
                                            .house(Coord::components_proj(x1, y1, hard1)),
                                        HouseType::from_index(hard2)
                                            .house(Coord::components_proj(x2, y2, hard2)),
                                    ],
                                    candidate_clues: vec![
                                        Candidate::new_single(x1, y1, target),
                                        Candidate::new_single(x2, y2, target),
                                        Candidate::new_single(p1, q1, target),
                                        Candidate::new_single(p2, q2, target),
                                    ],
                                    solver_id,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    //   skyscraper: weak in 0/1, pincers = weak.other
    //   2-string kite: week in 2, pincers in 0&1
    //   turbot fish:  weak in 0/1, pincers in 2 & weak.other
    //
    None
}

pub struct EmptyRectangle {
    id: usize,
}
impl EmptyRectangle {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
pub struct Skyscraper {
    id: usize,
}
impl Skyscraper {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}

impl Solver for Skyscraper {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        (0..1).find_map(|x| check_turbot(game_board, x, 1 - x, 1 - x, self.id))
    }
}

pub struct TwoStringKite {
    id: usize,
}
impl TwoStringKite {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for TwoStringKite {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        check_turbot(game_board, 2, 0, 1, self.id)
    }
}

pub struct TurbotFish {
    id: usize,
}
impl TurbotFish {
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
}
impl Solver for TurbotFish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        (0..1).find_map(|x| check_turbot(game_board, x, 2, 1 - x, self.id))
    }
}
