use crate::utils::BitMap;
#[derive(Debug, Clone)]
pub struct Candidate {
    pub x: usize,
    pub y: usize,
    pub candidates: BitMap,
}
impl Candidate {
    pub fn new_single(x: usize, y: usize, target: usize) -> Self {
        Self {
            x,
            y,
            candidates: BitMap::from(target),
        }
    }

    pub fn new(x:usize, y:usize, candidates: BitMap) -> Self{
        Self{
            x,y,candidates
        }
    }
}
