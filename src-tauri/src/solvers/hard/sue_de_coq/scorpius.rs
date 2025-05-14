use crate::{game_board::als::Als, utils::BitMap};

use super::{Orion, Yoke};

pub(super) struct Scorpius{
    indices: BitMap,
    candidates: BitMap,
}

impl Scorpius {
    pub(super) fn new(indices: BitMap, candidates: BitMap) -> Self {
        Self { indices, candidates }
    }
    pub(super) fn try_new(yoke: Yoke, orion: Orion, als : &Als) -> Option<Self>{
        todo!()
    }
}