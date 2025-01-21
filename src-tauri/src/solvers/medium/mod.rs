use crate::solvers::easy::{hidden_subset::HiddenQuadruple, naked_subset::NakedQuadruple};

use super::Solver;

// remote pair
// BUG+1
// Skyscraper
// 2-string kite
// turbot Fish
// Empty rectangle
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
use single_digit_patterns::Skyscraper;

use finned::FinnedXWing;
use fish::{Jellyfish, Swordfish, XWing};
#[rustfmt::skip]
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(XWing          ::with_id(8)),
        Box::new(Swordfish      ::with_id(9)),
        Box::new(NakedQuadruple ::with_id(10)),
        Box::new(Skyscraper     ::with_id(11)),
        Box::new(FinnedXWing    ::with_id(12)),
        Box::new(Jellyfish      ::with_id(13)),
        Box::new(HiddenQuadruple::with_id(14)),
    ]
}
