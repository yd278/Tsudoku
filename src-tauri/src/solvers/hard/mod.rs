// Sue de Coq
// X-Chain
// XY-Chain
// Nice Loop

use super::Solver;
use crate::solvers::medium::finned::{FinnedJellyfish, FinnedSwordfish};

#[rustfmt::skip]
pub fn get_hard_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(FinnedSwordfish),
        Box::new(FinnedJellyfish),
    ]
}
