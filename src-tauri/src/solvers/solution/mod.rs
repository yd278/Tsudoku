pub mod candidate;
pub mod confirmation_details;
pub mod elimination_details;
use crate::utils::House;
pub use candidate::Candidate;
pub use confirmation_details::ConfirmationDetails;
pub use elimination_details::EliminationDetails;
#[derive(Debug)]
pub enum Action {
    Confirmation(ConfirmationDetails),
    Elimination(EliminationDetails),
}
pub struct Solution {
    pub actions: Vec<Action>,
    pub house_clues: Vec<House>,
    pub candidate_clues: Vec<Candidate>,
}
