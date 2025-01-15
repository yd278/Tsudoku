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
        Box::new(super::easy::naked_subset::NakedPair),
        Box::new(super::easy::hidden_subset::HiddenPair),
        Box::new(super::easy::naked_subset::NakedTriple),
        Box::new(super::easy::hidden_subset::HiddenTriple),
        Box::new(super::easy::naked_subset::NakedQuadruple),
        Box::new(super::easy::hidden_subset::HiddenQuadruple),
    ]
}
