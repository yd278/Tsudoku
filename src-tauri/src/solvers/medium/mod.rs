use super::Solver;

// X-wing
// swordfish
// Naked/Hidden Quadruples
// Jellyfish
// remote pair
// BUG+1
// Skyscraper
// 2-string kite
// turbot Fish
// Empty rectangle
// w-wing
// xy-wing
// xyz-wing
// uniqueness test 1~6
// hidden rectangle
//avoidable rectangle 1/2
//finned/ sashimi x-wing
// simple colors
//multi colors
mod fish;
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(super::easy::naked_subset::NakedQuadruple),
        Box::new(super::easy::hidden_subset::HiddenQuadruple),
    ]
}
