use crate::{
    game_board::GameBoard,
    utils::{BitMap, Coord, House},
};

#[derive(Copy, Clone)]
struct CellInfo {
    candidate: BitMap,
    index_in_line: BitMap,
    index_in_box: BitMap,
}

impl CellInfo {
    fn from_cell(
        game_board: &GameBoard,
        x: usize,
        y: usize,
        line_dim: usize,
        line_id: usize,
        box_id: usize,
    ) -> Option<Self> {
        Some(Self {
            candidate: game_board.get_candidates(x, y)?,
            index_in_line: BitMap::from(Coord::get_index_from_house(
                &House::from_dim_id(line_dim, line_id),
                x,
                y,
            )?),
            index_in_box: BitMap::from(Coord::get_index_from_house(&House::Box(box_id), x, y)?),
        })
    }
}

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

    fn accumulate_cells(
        cells: &[CellInfo],
        line_dim: usize,
        line_id: usize,
        box_id: usize,
    ) -> Self {
        let mut candidates: BitMap = BitMap::new();
        let mut indices_in_line: BitMap = BitMap::new();
        let mut indices_in_box: BitMap = BitMap::new();
        for cell_info in cells {
            candidates.insert_set(cell_info.candidate);
            indices_in_line.insert_set(cell_info.index_in_line);
            indices_in_box.insert_set(cell_info.index_in_box);
        }
        Self::new(
            line_dim,
            line_id,
            box_id,
            candidates,
            indices_in_line,
            indices_in_box,
        )
    }
    pub(super) fn get_all_yokes(
        game_board: &GameBoard,
        line_dim: usize,
        line_id: usize,
        box_id: usize,
    ) -> impl Iterator<Item = Self> + '_ {
        let cells: Vec<CellInfo> =
            Coord::intersect(House::from_dim_id(line_dim, line_id), House::Box(box_id))
                .filter_map(|(x, y)| {
                    CellInfo::from_cell(game_board, x, y, line_dim, line_id, box_id)
                })
                .collect();
        match cells.len() {
            2 => vec![Self::accumulate_cells(&cells, line_dim, line_id, box_id)].into_iter(),
            3 => vec![
                Self::accumulate_cells(&cells[0..3], line_dim, line_id, box_id),
                Self::accumulate_cells(&cells[0..2], line_dim, line_id, box_id),
                Self::accumulate_cells(&cells[1..3], line_dim, line_id, box_id),
                Self::accumulate_cells(&[cells[0], cells[2]], line_dim, line_id, box_id),
            ]
            .into_iter(),
            _ => vec![].into_iter(),
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

    pub(super) fn indices_in_line(&self) -> BitMap {
        self.indices_in_line
    }
}
