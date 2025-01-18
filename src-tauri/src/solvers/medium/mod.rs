use super::Solver;


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
mod finned;
pub fn get_medium_solvers() -> Vec<Box<dyn Solver>> {
    vec![
        Box::new(super::medium::fish::XWing),
        Box::new(super::medium::fish::Swordfish),
        Box::new(super::easy::naked_subset::NakedQuadruple),
        Box::new(super::medium::finned::FinnedXWing),
        Box::new(super::medium::fish::Jellyfish),
        Box::new(super::easy::hidden_subset::HiddenQuadruple),
        Box::new(super::medium::finned::FinnedSwordfish),
        Box::new(super::medium::finned::FinnedJellyfish),
    ]
}

