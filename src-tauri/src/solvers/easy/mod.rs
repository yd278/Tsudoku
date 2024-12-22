use super::Solver;
mod naked_single;
mod hidden_single;
pub fn get_easy_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(super::easy::naked_single::NakedSingle),
        Box::new(super::easy::hidden_single::HiddenSingle),
    ]
}