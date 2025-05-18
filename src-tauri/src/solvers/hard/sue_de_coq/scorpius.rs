use crate::{game_board::als::Als, utils::BitMap};

use super::{Orion, Yoke};
#[derive(Clone, Copy)]
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
    fn yoke_compatible_check(yoke: Yoke, als: &Als) -> bool {
        yoke.indices_in_line().intersect(als.indices()).count() == 0
            && yoke.candidates().intersect(als.candidates()).count() >= 2
    }
    fn orion_conjugate_checck(yoke: Yoke, orion: Orion, als: &Als) -> bool {
        let line_remain = yoke.candidates().difference(orion.candidates());
        let box_remain = yoke.candidates().difference(als.candidates());
        line_remain.count() < als.indices().count() + yoke.indices_in_line().count()
            && box_remain.count() < orion.indices().count() + yoke.indices_in_box().count()
    }
    pub(super) fn try_new(yoke: Yoke, orion: Orion, als: &Als) -> Option<Self> {
        (Self::yoke_compatible_check(yoke, als) && Self::orion_conjugate_checck(yoke, orion, als))
            .then_some(Self::new(als.indices(), als.candidates()))
    }

    pub(super) fn indices(&self) -> BitMap {
        self.indices
    }

    pub(super) fn candidates(&self) -> BitMap {
        self.candidates
    }
}
