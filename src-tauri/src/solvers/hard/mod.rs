// Sue de Coq
// X-Chain
// XY-Chain
// Nice Loop

pub(super) use crate::solvers::medium::finned::{FinnedJellyfish, FinnedSwordfish};
use crate::solvers::solver_enum::SolverEnum;
mod sue_de_coq;
pub(super) use sue_de_coq::SueDeCoq;
#[rustfmt::skip]
pub fn get_hard_solvers() -> Vec<SolverEnum> {
    vec![
        SolverEnum::from(FinnedSwordfish),
        SolverEnum::from(FinnedJellyfish),
        SolverEnum::from(SueDeCoq)
    ]
}
