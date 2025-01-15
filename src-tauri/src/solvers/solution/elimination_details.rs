use crate::utils::BitMap;

#[derive(Debug)]
pub struct EliminationDetails {
    pub x: usize,
    pub y: usize,
    pub target: BitMap,
}
