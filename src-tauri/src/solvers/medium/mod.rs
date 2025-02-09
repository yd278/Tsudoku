use crate::{
    game_board::GameBoard,
    solvers::easy::{hidden_subset::HiddenQuadruple, naked_subset::NakedQuadruple},
    utils::{BitMap, Coord},
};

use super::Solver;

// remote pair

// w-wing
// xy-wing
// xyz-wing
// simple colors
//multi colors
mod finned;
mod fish;
mod single_digit_patterns;
mod uniqueness;
use single_digit_patterns::{EmptyRectangle, Skyscraper, TurbotFish, TwoStringKite};
use uniqueness::{
    AvoidableRectangle1, AvoidableRectangle2, BiValueUniversalGravePlusOne, UniquenessTest1,
    UniquenessTest2, UniquenessTest3, UniquenessTest4, UniquenessTest5, UniquenessTest6,
};
mod wings;
use finned::FinnedXWing;
use fish::{Jellyfish, Swordfish, XWing};
#[rustfmt::skip]
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(XWing                       ::with_id(8)), 
        Box::new(Swordfish                   ::with_id(9)), 
        Box::new(Skyscraper                  ::with_id(10)), 
        Box::new(TwoStringKite               ::with_id(11)),
        Box::new(TurbotFish                  ::with_id(12)),
        Box::new(EmptyRectangle              ::with_id(13)),
        Box::new(BiValueUniversalGravePlusOne::with_id(14)),
        Box::new(UniquenessTest1             ::with_id(15)),
        Box::new(UniquenessTest2             ::with_id(16)),
        Box::new(UniquenessTest3             ::with_id(17)),
        Box::new(UniquenessTest4             ::with_id(18)),
        Box::new(UniquenessTest5             ::with_id(19)),
        Box::new(UniquenessTest6             ::with_id(20)),
        Box::new(FinnedXWing                 ::with_id(21)),
        Box::new(AvoidableRectangle1         ::with_id(22)),
        Box::new(AvoidableRectangle2         ::with_id(23)),
        Box::new(NakedQuadruple              ::with_id(24)), 
        Box::new(HiddenQuadruple             ::with_id(25)), 
        Box::new(Jellyfish                   ::with_id(26)),
    ]
}
#[derive(Copy, Clone)]
struct BiValueCell {
    x: usize,
    y: usize,
    bi_value: BitMap,
}
impl BiValueCell {
    pub fn new(x: usize, y: usize, bi_value: BitMap) -> Self {
        Self { x, y, bi_value }
    }
    pub fn try_new(x: usize, y: usize, bi_value: BitMap) -> Option<Self> {
        (bi_value.count() == 2).then_some(Self::new(x, y, bi_value))
    }
}
/// Iter through the whole
fn iter_valid_bi_value(game_board: &GameBoard) -> impl Iterator<Item = BiValueCell> + '_ {
    Coord::all_cells().filter_map(|(px, py)| {
        game_board.get_candidates(px, py).and_then(|candidates| {
            (candidates.count() == 2).then_some(BiValueCell::new(px, py, candidates))
        })
    })
}
