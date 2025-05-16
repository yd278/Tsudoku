use crate::{
    game_board::GameBoard,
    utils::{BitMap, Coord, House},
};

#[derive(Copy, Clone)]
pub(super) struct Yoke {
    line_dim: usize,
    line_id: usize,
    box_id: usize,
    candidates: BitMap,
    indices_in_line: BitMap,
    indices_in_box: BitMap,
}

impl Yoke {
    pub(super) fn new(
        line_dim: usize,
        line_id: usize,
        box_id: usize,
        candidates: BitMap,
        indices_in_line: BitMap,
        indices_in_box: BitMap,
    ) -> Self {
        Self {
            line_dim,
            line_id,
            box_id,
            candidates,
            indices_in_line,
            indices_in_box,
        }
    }
    pub(super) fn try_new(
        game_board: &GameBoard,
        line_dim: usize,
        line_id: usize,
        box_id: usize,
    ) -> Option<Self> {
        let cells: Vec<(usize, usize)> =
            Coord::intersect(House::from_dim_id(line_dim, line_id), House::Box(box_id))
                .filter(|&(x, y)| game_board.not_filled(x, y))
                .collect();
        if cells.len() >= 2 {
            let mut candidates: BitMap = BitMap::new();
            let mut indices_in_line: BitMap = BitMap::new();
            let mut indices_in_box: BitMap = BitMap::new();
            for (x, y) in cells {
                candidates.insert_set(game_board.get_candidates(x, y)?);
                indices_in_line.insert(Coord::get_index_from_house(
                    &House::from_dim_id(line_dim, line_id),
                    x,
                    y,
                ));
                indices_in_box.insert(Coord::get_index_from_house(&House::Box(box_id), x, y));
            }
            Some(Self::new(
                line_dim,
                line_id,
                box_id,
                candidates,
                indices_in_line,
                indices_in_box,
            ))
        } else {
            None
        }
    }

    pub(super) fn box_id(&self) -> usize {
        self.box_id
    }

    pub(super) fn line_dim(&self) -> usize {
        self.line_dim
    }

    pub(super) fn line_id(&self) -> usize {
        self.line_id
    }

    pub(super) fn candidates(&self) -> BitMap {
        self.candidates
    }

    pub(super) fn indices_in_box(&self) -> BitMap {
        self.indices_in_box
    }
}

