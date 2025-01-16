use crate::{
    game_board::GameBoard,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{BitMap, Coord, House},
};

fn find_eliminable(
    game_board: &GameBoard,
    base_sets: &BitMap,
    cover_sets: Vec<House>,
    target: usize,
) -> Vec<Action> {
    for house in cover_sets {
        let res: Vec<Action> = base_sets
            .iter_ones()
            .filter_map(|x| {
                let (x, y) = Coord::from_house_and_index(&house, x);
                game_board.get_candidates(x, y).and_then(|candidates| {
                    if candidates.contains(target) {
                        Some(Action::Elimination(EliminationDetails {
                            x,
                            y,
                            target: BitMap::from(target),
                        }))
                    } else {
                        None
                    }
                })
            })
            .collect();
        if !res.is_empty() {
            return res;
        }
    }
    Vec::new()
}

#[derive(Clone)]
enum Dimension {
    Row,
    Col,
}

impl Dimension {
    fn other(&self) -> Dimension {
        match self {
            Dimension::Row => Dimension::Col,
            Dimension::Col => Dimension::Row,
        }
    }
    fn house(&self, x: usize) -> House {
        match self {
            Dimension::Row => House::Row(x),
            Dimension::Col => House::Col(x),
        }
    }
}
fn cover_sets_from_appearance(appearance: BitMap, dim: Dimension) -> Vec<House> {
    appearance.iter_ones().map(|x| dim.house(x)).collect()
}

fn check_base_set_combo(
    game_board: &GameBoard,
    n: usize,
    base_dim: &Dimension,
    target: usize,
    combo: &BitMap,
) -> Option<Solution> {
    let mut appearance = BitMap::new();
    for base in combo.iter_ones() {
        for cover in (0..9).filter(|index| {
            let (x, y) = Coord::from_house_and_index(&base_dim.house(base), *index);
            game_board.contains_candidate(x, y, target)
        }) {
            appearance.insert(cover);
            if appearance.count() > n {
                return None;
            };
        }
    }
    if appearance.count() == n {
        let cover_sets: Vec<House> = appearance
            .iter_ones()
            .map(|x| base_dim.other().house(x))
            .collect();
        let elimination = find_eliminable(game_board, combo, cover_sets, target);
        if !elimination.is_empty() {
            return Some(Solution {
                actions: elimination,
                house_clues: combo
                    .iter_ones()
                    .map(|base_index| base_dim.house(base_index))
                    .chain(
                        appearance
                            .iter_ones()
                            .map(|cover_index| base_dim.other().house(cover_index)),
                    )
                    .collect(),
                candidate_clues: combo
                    .iter_ones()
                    .flat_map(|base_index| {
                        appearance.iter_ones().map({
                            let value = base_dim.clone();
                            move |cover_index| match value {
                                Dimension::Row => (base_index, cover_index),
                                Dimension::Col => (cover_index, base_index),
                            }
                        })
                    })
                    .filter(|(x, y)| game_board.contains_candidate(*x, *y, target))
                    .map(|(x, y)| Candidate {
                        x,
                        y,
                        candidates: BitMap::from(target),
                    })
                    .collect(),
            });
        } else {
            return None;
        }
    }
    None
}

fn check_fish_with_dim(game_board: &GameBoard, n: usize, base_dim: &Dimension) -> Option<Solution> {
    for target in 0..9 {
        for combo in BitMap::get_combo_with_mask(n, &game_board.row_occupied()[target]) {
            if let Some(solution) = check_base_set_combo(game_board, n, base_dim, target, &combo) {
                return Some(solution);
            }
        }
    }
    None
}

pub struct XWing;
impl Solver for XWing {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 2, &base_dim))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}

pub struct Swordfish;
impl Solver for Swordfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 3, &base_dim))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}

pub struct Jellyfish;
impl Solver for Jellyfish {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
            .into_iter()
            .find_map(|base_dim| check_fish_with_dim(game_board, 4, &base_dim))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}
// 枚举base-sets和target
//      如果target的出现的并集大小==n，对于这个并集构成的cover set
