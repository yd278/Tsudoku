use super::Solver;
mod claiming;
mod hidden_single;
pub mod hidden_subset;
mod naked_single;
pub mod naked_subset;
mod pointing;
use claiming::Claiming;
use hidden_single::HiddenSingle;
use hidden_subset::{HiddenPair, HiddenTriple};
use naked_single::NakedSingle;
use naked_subset::{NakedPair, NakedTriple};
use pointing::Pointing;

#[rustfmt::skip]
pub fn get_easy_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(NakedSingle ),
        Box::new(HiddenSingle),
        Box::new(Pointing    ),
        Box::new(Claiming    ),
        Box::new(NakedPair   ),
        Box::new(HiddenPair  ),
        Box::new(NakedTriple ),
        Box::new(HiddenTriple),
    ]
}
