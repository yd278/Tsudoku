pub mod easy;
pub mod medium;
pub mod solution;
pub mod traits;

pub use traits::Solver; // 重新导出 Solver trait
mod solver_identifier;
pub use solver_identifier::SolverIdentifier;
