pub mod candidate;
pub mod confirmation_details;
pub mod elimination_details;
use std::fmt;

use crate::utils::House;
pub use candidate::Candidate;
pub use confirmation_details::ConfirmationDetails;
pub use elimination_details::EliminationDetails;

use super::SolverIdentifier;
pub enum Action {
    Confirmation(ConfirmationDetails),
    Elimination(EliminationDetails),
}
impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmation(cd) => write!(f, "{cd:?}"),
            Self::Elimination(ed) => write!(f, "{ed:?}"),
        }
    }
}
pub struct Solution {
    pub actions: Vec<Action>,
    pub house_clues: Vec<House>,
    pub candidate_clues: Vec<Candidate>,
    pub solver_id: SolverIdentifier,
}
