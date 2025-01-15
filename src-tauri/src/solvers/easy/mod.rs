use super::Solver;
mod claiming;
mod hidden_single;
mod hidden_subset;
mod naked_single;
mod naked_subset;
mod pointing;
pub fn get_easy_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(super::easy::naked_single::NakedSingle),
        Box::new(super::easy::hidden_single::HiddenSingle),
        Box::new(super::easy::pointing::Pointing),
        Box::new(super::easy::claiming::Claiming),
    ]
}
