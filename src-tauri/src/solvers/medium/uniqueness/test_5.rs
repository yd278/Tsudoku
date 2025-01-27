use crate::{
    game_board::GameBoard,
    solvers::{solution::Solution, Solver},
    utils::{BitMap, Coord},
};

use super::UniquenessTest5;

impl UniquenessTest5 {
    fn valid_unique_rectangle_cell(
        game_board: &GameBoard,
        x: usize,
        y: usize,
        bi_value: BitMap,
    ) -> Option<(BitMap, BitMap)> {
        bi_value
            .iter_ones()
            .all(|candidate| game_board.could_have_been(x, y, candidate))
            .then_some(game_board.get_candidates(x, y).map(|candidate| {
                (
                    candidate.intersect(&bi_value),
                    candidate.difference(&bi_value),
                )
            }))
            .flatten()
    }

    fn valid_bi_value(game_board: &GameBoard) -> impl Iterator<Item = (usize, usize, BitMap)> + '_ {
        Coord::all_cells().filter_map(|(px, py)| {
            game_board
                .get_candidates(px, py)
                .and_then(|candidates| (candidates.count() == 2).then_some((px, py, candidates)))
        })
    }
    ///returns qy, the clue candidates and the extra candidate in cell (px,qy)
    fn iter_valid_base_row(
        game_board: &GameBoard,
        px: usize,
        py: usize,
        bi_value: BitMap,
    ) -> impl Iterator<Item = (usize, BitMap, BitMap)> + '_ {
        (0..9).filter(move |&qy| qy != py).filter_map(move |qy| {
            Self::valid_unique_rectangle_cell(game_board, px, qy, bi_value)
                .map(|(clue_candidates, extra_candidates)| {
                    (extra_candidates.count() == 1).then_some((
                        qy,
                        clue_candidates,
                        extra_candidates,
                    ))
                })
                .flatten()
        })
    }
    fn iter_valid_base_col(
        game_board: &GameBoard,
        px: usize,
        py: usize,
        bi_value: BitMap,
        extra_candidate: BitMap,
    ) -> impl Iterator<Item = (usize, BitMap)> + '_ {
        (0..9).filter(move |&rx| rx != px).filter_map(move |rx| {
            Self::valid_unique_rectangle_cell(game_board, rx, py, bi_value)
                .map(|(clue_candidates, extra_candidates_r)| {
                    (extra_candidate == extra_candidates_r).then_some((rx, clue_candidates))
                })
                .flatten()
        })
    }
}
impl Solver for UniquenessTest5 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Self::valid_bi_value(game_board)
            .flat_map(|(px, py, bi_value)| {
                Self::iter_valid_base_row(game_board, px, py, bi_value).map(
                    move |(qy, clue_candidates_q, extra_candidate)| {
                        (px, py, qy, bi_value, clue_candidates_q, extra_candidate)
                    },
                )
            })
            .flat_map(
                |(px, py, qy, bi_value, clue_candidates_q, extra_candidate)| {
                    Self::iter_valid_base_col(game_board, px, py, bi_value, extra_candidate).map(
                        move |(rx, clue_candidates_r)| {
                            (
                                px,
                                py,
                                rx,
                                qy,
                                bi_value,
                                clue_candidates_q,
                                clue_candidates_r,
                                extra_candidate,
                            )
                        },
                    )
                },
            )
            .count();
        todo!()
    }
}
