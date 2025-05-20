use std::fmt;

use crate::utils::BitMap;
#[derive(Clone)]
pub struct Candidate {
    pub x: usize,
    pub y: usize,
    pub candidates: BitMap,
}
impl fmt::Debug for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.candidates == BitMap::NAN {
            write!(f, "SEP")
        } else {
            write!(
                f,
                "Candidates {:?} in Cell ({}, {})",
                self.candidates,
                self.x + 1,
                self.y + 1
            )
        }
    }
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
