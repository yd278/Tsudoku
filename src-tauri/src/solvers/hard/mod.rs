// Sue de Coq
// X-Chain
// XY-Chain
// Nice Loop

use super::Solver;
use crate::solvers::medium::finned::{FinnedJellyfish, FinnedSwordfish};
mod sue_de_coq;
use sue_de_coq::SueDeCoq;
#[rustfmt::skip]
pub fn get_hard_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(FinnedSwordfish),
        Box::new(FinnedJellyfish),
        Box::new(SueDeCoq)
    ]
}
