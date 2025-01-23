use crate::solvers::easy::{hidden_subset::HiddenQuadruple, naked_subset::NakedQuadruple};

use super::Solver;

// remote pair
// BUG+1
// w-wing
// xy-wing
// xyz-wing
// uniqueness test 1~6
// hidden rectangle
//avoidable rectangle 1/2
// simple colors
//multi colors
mod finned;
mod fish;
mod single_digit_patterns;
use single_digit_patterns::{EmptyRectangle, Skyscraper, TurbotFish, TwoStringKite};

use finned::FinnedXWing;
use fish::{Jellyfish, Swordfish, XWing};
#[rustfmt::skip]
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(XWing          ::with_id(8)), 
        Box::new(Swordfish      ::with_id(9)), 
        Box::new(Skyscraper     ::with_id(10)), 
        Box::new(TwoStringKite  ::with_id(11)),
        Box::new(TurbotFish     ::with_id(12)),
        Box::new(EmptyRectangle ::with_id(13)),
        Box::new(FinnedXWing    ::with_id(14)),
        Box::new(NakedQuadruple ::with_id(15)), 
        Box::new(HiddenQuadruple::with_id(16)), 
        Box::new(Jellyfish      ::with_id(17)),
    ]
}
