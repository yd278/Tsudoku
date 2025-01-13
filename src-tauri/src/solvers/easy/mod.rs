use super::Solver;
mod naked_single;
mod hidden_single;
mod pointing;
mod claiming;
mod naked_subset;
mod hidden_subset;
pub fn get_easy_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(super::easy::naked_single::NakedSingle),
        Box::new(super::easy::hidden_single::HiddenSingle),
        Box::new(super::easy::pointing::Pointing),
        Box::new(super::easy::claiming::Claiming),
    ]
}