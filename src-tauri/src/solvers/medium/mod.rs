use crate::{
    game_board::GameBoard,
    solvers::easy::{hidden_subset::HiddenQuadruple, naked_subset::NakedQuadruple},
    utils::{BitMap, Coord},
};

use super::Solver;

// remote pair

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
use wings::{WWing, XYWing, XYZWing};
mod color;
use color::Coloring;
#[rustfmt::skip]
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(XWing                       ), 
        Box::new(Swordfish                   ), 
        Box::new(Skyscraper                  ), 
        Box::new(TwoStringKite               ),
        Box::new(TurbotFish                  ),
        Box::new(EmptyRectangle              ),
        Box::new(BiValueUniversalGravePlusOne),
        Box::new(UniquenessTest1             ),
        Box::new(UniquenessTest2             ),
        Box::new(UniquenessTest3             ),
        Box::new(UniquenessTest4             ),
        Box::new(UniquenessTest5             ),
        Box::new(UniquenessTest6             ),
        Box::new(FinnedXWing                 ),
        Box::new(AvoidableRectangle1         ),
        Box::new(AvoidableRectangle2         ),
        Box::new(NakedQuadruple              ), 
        Box::new(HiddenQuadruple             ), 
        Box::new(Jellyfish                   ),
        Box::new(XYWing                      ),
        Box::new(XYZWing                     ),
        Box::new(WWing                       ),
        Box::new(Coloring                    ),
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
