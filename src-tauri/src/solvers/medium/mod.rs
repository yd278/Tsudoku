use crate::{
    game_board::GameBoard,
    solvers::solver_enum::SolverEnum,
    utils::{BitMap, Coord},
};

pub mod finned;
mod fish;
mod single_digit_patterns;
mod uniqueness;
pub(super) use single_digit_patterns::{EmptyRectangle, Skyscraper, TurbotFish, TwoStringKite};
pub(super) use uniqueness::{
    AvoidableRectangle1, AvoidableRectangle2, BiValueUniversalGravePlusOne, UniquenessTest1,
    UniquenessTest2, UniquenessTest3, UniquenessTest4, UniquenessTest5, UniquenessTest6,
};
mod wings;
pub(super) use super::easy::hidden_subset::HiddenQuadruple;
pub(super) use super::easy::naked_subset::NakedQuadruple;
pub(super) use finned::FinnedXWing;
pub(super) use fish::{Jellyfish, Swordfish, XWing};
pub(super) use wings::{WWing, XYWing, XYZWing};
mod color;
pub(super) use color::Coloring;
#[rustfmt::skip]
pub fn get_medium_solvers() -> Vec<SolverEnum> {
    vec![
        SolverEnum::from(XWing                       ), 
        SolverEnum::from(Swordfish                   ), 
        SolverEnum::from(Skyscraper                  ), 
        SolverEnum::from(TwoStringKite               ),
        SolverEnum::from(TurbotFish                  ),
        SolverEnum::from(EmptyRectangle              ),
        SolverEnum::from(BiValueUniversalGravePlusOne),
        SolverEnum::from(UniquenessTest1             ),
        SolverEnum::from(UniquenessTest2             ),
        SolverEnum::from(UniquenessTest3             ),
        SolverEnum::from(UniquenessTest4             ),
        SolverEnum::from(UniquenessTest5             ),
        SolverEnum::from(UniquenessTest6             ),
        SolverEnum::from(FinnedXWing                 ),
        SolverEnum::from(AvoidableRectangle1         ),
        SolverEnum::from(AvoidableRectangle2         ),
        SolverEnum::from(NakedQuadruple              ), 
        SolverEnum::from(HiddenQuadruple             ), 
        SolverEnum::from(Jellyfish                   ),
        SolverEnum::from(XYWing                      ),
        SolverEnum::from(XYZWing                     ),
        SolverEnum::from(WWing                       ),
        SolverEnum::from(Coloring                    ),
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
