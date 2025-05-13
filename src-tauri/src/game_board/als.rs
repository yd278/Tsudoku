use crate::utils::{BitMap, Coord, House};

use super::GameBoard;


pub struct Als{
    house_type: usize,
    house_id: usize,
    indices: BitMap,
    candidates: BitMap,
}

impl Als {
    
    pub fn new(house_type: usize, house_id: usize, indices: BitMap, candidates: BitMap) -> Self {
        Self { house_type, house_id, indices, candidates }
    }
    
    pub fn try_new(game_board: &GameBoard, als_indices: BitMap, house_type:usize, house_id: usize) -> Option<Self>{
        let mut candidates = BitMap::new();
        for cell_id in als_indices.iter_ones(){
            let (x,y) = Coord::from_house_and_index(&House::from_dim_id(house_type, house_id), cell_id);
            candidates.insert_set(game_board.get_candidates(x, y)?);
        }
        if candidates.count() == als_indices.count()+1{
            Some(Self::new(house_type,house_id,als_indices,candidates))
        }else{
            None
        }
    }
    
    pub fn candidates(&self) -> BitMap {
        self.candidates
    }
    
    pub fn indices(&self) -> BitMap {
        self.indices
    }
    
}