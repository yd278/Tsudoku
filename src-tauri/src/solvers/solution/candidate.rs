use crate::utils::BitMap;
#[derive(Debug, Clone)]
pub struct Candidate {
    pub x: usize,
    pub y: usize,
    pub candidates: BitMap,
}

impl Candidate {
    pub const SEPARATOR: Self = Self {
        x: 0,
        y: 0,
        candidates: BitMap::NAN,
    };

    pub fn new_single(x: usize, y: usize, target: usize) -> Self {
        Self {
            x,
            y,
            candidates: BitMap::from(target),
        }
    }

    pub fn new(x: usize, y: usize, candidates: BitMap) -> Self {
        Self { x, y, candidates }
    }

    pub fn from_coord((x, y): (usize, usize), candidates: BitMap) -> Self {
        Self { x, y, candidates }
    }
    pub fn from_coord_single((x, y): (usize, usize), target: usize) -> Self {
        Self {
            x,
            y,
            candidates: BitMap::from(target),
        }
    }
}
