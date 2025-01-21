use crate::utils::BitMap;
#[derive(Debug, Clone)]
pub struct Candidate {
    pub x: usize,
    pub y: usize,
    pub candidates: BitMap,
}
