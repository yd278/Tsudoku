use crate::utils::BitMap;
#[derive(Debug)]
pub struct Candidate {
    pub x: usize,
    pub y: usize,
    pub candidates: BitMap,
}
