use crate::{game_board::als::Als, utils::BitMap};

use super::Yoke;
#[derive(Clone, Copy)]
pub(super) struct Orion {
    candidates: BitMap,
    indices: BitMap,
}

impl Orion {
    pub(super) fn new(candidates: BitMap, indices: BitMap) -> Self {
        Self {
            candidates,
            indices,
        }
    }

    pub(super) fn try_new(yoke: Yoke, als: &Als) -> Option<Self> {
        ((als.indices().intersect(yoke.indices_in_box()).count() == 0)
            && (als.candidates().intersect(yoke.candidates()).count() >= 2))
            .then_some(Self::new(als.candidates(), als.indices()))
    }
}

