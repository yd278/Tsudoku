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
        Box::new(NakedSingle ::with_id(0)),
        Box::new(HiddenSingle::with_id(1)),
        Box::new(Pointing    ::with_id(2)),
        Box::new(Claiming    ::with_id(3)),
        Box::new(NakedPair   ::with_id(4)),
        Box::new(HiddenPair  ::with_id(5)),
        Box::new(NakedTriple ::with_id(6)),
        Box::new(HiddenTriple::with_id(7)),
    ]
}
