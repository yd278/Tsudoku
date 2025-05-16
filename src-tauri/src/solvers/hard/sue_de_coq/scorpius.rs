use crate::{game_board::als::Als, utils::BitMap};

use super::{Orion, Yoke};

pub(super) struct Scorpius {
    indices: BitMap,
    candidates: BitMap,
}

impl Scorpius {
    pub(super) fn new(indices: BitMap, candidates: BitMap) -> Self {
        Self {
            indices,
            candidates,
        }
    }
    pub(super) fn try_new(yoke: Yoke, orion: Orion, als: &Als) -> Option<Self> {
        // TODO: check if als has no intersection with yoke in cell and has at least 2 mutual
        // candidates with yoke and
        //  1. pulling out orion's candidates leads to insufficient candidates in yoke union
        //     Scorpius
        //  2. pulling out Scorpius's candidates leads to insufficient candidates in yoke union
        //     Orion
        todo!()
    }
}

