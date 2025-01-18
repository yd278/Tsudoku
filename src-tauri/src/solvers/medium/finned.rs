use crate::{
    game_board::GameBoard,
    solvers::{solution::{Action, Candidate, EliminationDetails, Solution}, Solver},
    utils::{AllEqualValue, BitMap, Coord, Dimension},
};

fn get_coords_with_target_by_masks<'a>(
    game_board: &'a GameBoard,
    first: &'a BitMap,
    second: &'a BitMap,
    first_dim: &'a Dimension,
    target: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    first
        .iter_ones()
        .flat_map(move |first_index| {
            second
                .iter_ones()
                .map(move |second_index| match &first_dim {
                    Dimension::Row => (first_index, second_index),
                    Dimension::Col => (second_index, first_index),
                })
        })
        .filter(move |&(x, y)| game_board.contains_candidate(x, y, target))
}

fn find_finned_fish(game_board: &GameBoard, base_dim: &Dimension, n: usize) -> Option<Solution> {
    for target in 0..9 {
        for base in BitMap::get_combo_with_mask(n, game_board.occupied_by(base_dim, target)) {
            for cover in
                BitMap::get_combo_with_mask(n, game_board.occupied_by(&base_dim.other(), target))
            {
                let cover_comp = cover.complement();
                let base_comp = base.complement();
                let body_clues: Vec<_> =
                    get_coords_with_target_by_masks(game_board, &base, &cover, base_dim, target)
                        .map(|(x, y)| Candidate {
                            x,
                            y,
                            candidates: BitMap::from(target),
                        })
                        .collect();

                if body_clues.is_empty() {
                    continue;
                }

                let fin_clues: Vec<_> = get_coords_with_target_by_masks(
                    game_board,
                    &base,
                    &cover_comp,
                    base_dim,
                    target,
                )
                .map(|(x, y)| Candidate {
                    x,
                    y,
                    candidates: BitMap::from(target),
                })
                .collect();

                if fin_clues.is_empty() {
                    continue;
                }

                let fin_box = fin_clues
                    .clone()
                    .into_iter()
                    .map(
                        |Candidate {
                             x,
                             y,
                             candidates: _,
                         }| Coord::get_box_id(x, y),
                    )
                    .all_equal_value();

                if let Some(fin_box) = fin_box {
                    let eliminable: Vec<_> = get_coords_with_target_by_masks(
                        game_board, &base_comp, &cover, base_dim, target,
                    )
                    .filter(|&(x, y)| Coord::get_box_id(x, y) == fin_box)
                    .map(|(x, y)| {
                        Action::Elimination(EliminationDetails {
                            x,
                            y,
                            target: BitMap::from(target),
                        })
                    })
                    .collect();
                    if !eliminable.is_empty() {
                        return Some(Solution {
                            actions: eliminable,
                            candidate_clues: [body_clues, fin_clues].concat(),
                            house_clues: base
                                .iter_ones()
                                .map(|base_index| base_dim.house(base_index))
                                .chain(
                                    cover
                                        .iter_ones()
                                        .map(|cover_index| base_dim.other().house(cover_index)),
                                )
                                .collect(),
                        });
                    }
                }
            }
        }
    }
    None
}


pub struct FinnedXWing;
impl Solver for FinnedXWing{
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
        .into_iter()
        .find_map(|base_dim| find_finned_fish(game_board,  &base_dim,2))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}
pub struct FinnedSwordfish;
impl Solver for FinnedSwordfish{
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
        .into_iter()
        .find_map(|base_dim| find_finned_fish(game_board,  &base_dim,3))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}

pub struct FinnedJellyfish;
impl Solver for FinnedJellyfish{
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        [Dimension::Row, Dimension::Col]
        .into_iter()
        .find_map(|base_dim| find_finned_fish(game_board,  &base_dim,4))
    }

    fn solver_id(&self) -> usize {
        todo!()
    }
}