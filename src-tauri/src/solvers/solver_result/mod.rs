pub mod confirmation;
pub mod elimination;
pub mod candidate;
use crate::utils::House;
pub enum SolverActionResult {
    Confirmation(confirmation::Confirmation),
    Elimination(elimination::Elimination),
}
pub struct SolverResult{
    pub actions: Vec<SolverActionResult>,
    pub house_clues: Vec<House>,
    pub candidate_clues: Vec<candidate::Candidate>,
}