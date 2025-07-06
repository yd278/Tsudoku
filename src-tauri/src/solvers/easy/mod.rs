use crate::solvers::solver_enum::SolverEnum;

mod claiming;
mod hidden_single;
pub mod hidden_subset;
mod naked_single;
pub mod naked_subset;
mod pointing;
pub(super) use claiming::Claiming;
pub(super) use hidden_single::HiddenSingle;
pub(super) use hidden_subset::{HiddenPair, HiddenTriple};
pub(super) use naked_single::NakedSingle;
pub(super) use naked_subset::{NakedPair, NakedTriple};
pub(super) use pointing::Pointing;

#[rustfmt::skip]
pub fn get_easy_solvers() -> Vec<SolverEnum> {
    vec![
        SolverEnum::from(NakedSingle),
        SolverEnum::from(HiddenSingle),
        SolverEnum::from(Pointing    ),
        SolverEnum::from(Claiming    ),
        SolverEnum::from(NakedPair   ),
        SolverEnum::from(HiddenPair  ),
        SolverEnum::from(NakedTriple ),
        SolverEnum::from(HiddenTriple),
    ]
}
