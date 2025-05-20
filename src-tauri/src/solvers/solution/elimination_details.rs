use std::fmt;

use crate::utils::BitMap;

pub struct EliminationDetails {
    pub x: usize,
    pub y: usize,
    pub target: BitMap,
}
impl fmt::Debug for EliminationDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} are eliminated in cell ({}, {})",
            self.target,
            self.x + 1,
            self.y + 1
        )
    }
}
