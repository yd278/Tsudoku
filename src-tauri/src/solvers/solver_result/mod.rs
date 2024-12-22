pub mod confirmation;
pub mod elimination;
pub mod candidate;
pub mod house;
pub enum SolverActionResult {
    Confirmation(confirmation::Confirmation),
    Elimination(elimination::Elimination),
}
pub struct SolverResult{
    pub actions: Vec<SolverActionResult>,
    pub house_clues: Vec<house::House>,
    pub candidate_clues: Vec<candidate::Candidate>,
}