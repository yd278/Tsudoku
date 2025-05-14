

use crate::{game_board::GameBoard, utils::BitMap};

#[derive(Copy,Clone)]
pub(super) struct Yoke{
    line_dim: usize,
    line_id: usize,
    box_id:usize,
    candidates: usize,
    indices_in_line: BitMap,
    indices_in_box: BitMap,
}

impl Yoke {
    pub(super) fn new(line_dim: usize, line_id: usize, box_id: usize, candidates: usize, indices_in_line: BitMap, indices_in_box: BitMap) -> Self {
        Self { line_dim, line_id, box_id, candidates, indices_in_line, indices_in_box }
    }
    pub(super) fn try_new(game_board: &GameBoard, line_dim: usize, line_id:usize,box_id:usize)-> Option<Self>{
        todo!()
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
}