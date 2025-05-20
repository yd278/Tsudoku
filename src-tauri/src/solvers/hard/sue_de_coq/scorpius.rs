use std::ops::Not;

use crate::{
    game_board::als::Als,
    utils::{BitMap, Coord, House},
};

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
    fn orion_no_intersect_check(yoke: Yoke, orion: Orion, als: &Als) -> bool {
        orion
            .indices()
            .iter_ones()
            .map(|index| Coord::from_house_and_index(&House::Box(yoke.box_id()), index))
            .filter_map(|(x, y)| {
                Coord::get_index_from_house(
                    &House::from_dim_id(yoke.line_dim(), yoke.line_id()),
                    x,
                    y,
                )
            })
            .any(|line_index| als.indices().contains(line_index))
            .not()
    }
    fn orion_conjugate_check(yoke: Yoke, orion: Orion, als: &Als) -> bool {
        let line_remain = yoke
            .candidates()
            .difference(orion.candidates())
            .union(als.candidates());
        let box_remain = yoke
            .candidates()
            .difference(als.candidates())
            .union(orion.candidates());
        line_remain.count() < als.indices().count() + yoke.indices_in_line().count()
            && box_remain.count() < orion.indices().count() + yoke.indices_in_box().count()
    }
    pub(super) fn try_new(yoke: Yoke, orion: Orion, als: &Als) -> Option<Self> {
        (Self::yoke_compatible_check(yoke, als)
            && Self::orion_no_intersect_check(yoke, orion, als)
            && Self::orion_conjugate_check(yoke, orion, als))
        .then_some(Self::new(als.indices(), als.candidates()))
    }

    pub(super) fn indices(&self) -> BitMap {
        self.indices
    }

    pub(super) fn candidates(&self) -> BitMap {
        self.candidates
    }
}
