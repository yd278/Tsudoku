
use crate::solvers::solution::Action::Confirmation;
use crate::solvers::solution::Action::Elimination;
use crate::solvers::solution::{Candidate, ConfirmationDetails, EliminationDetails, Solution};
use crate::solvers::Solver;
use crate::{game_board::GameBoard, utils::House};
pub fn test_function_e(
    solver: impl Solver,
    raws: [u16; 81],
    exp_actions: Vec<(usize, usize)>,
    exp_action_targets: Vec<u16>,
    exp_house_clues: Vec<House>,
    exp_candidate_clues: Vec<(usize, usize)>,
    exp_candidate_masks: Vec<u16>,
) {
    // raws
    let game_board = GameBoard::from_array(raws);
    // solver type
    let Solution {
        actions,
        house_clues,
        candidate_clues,
        solver_id: _,
    } = solver.solve(&game_board).unwrap();

    // action data
    let action_len = exp_actions.len();
    let action_std: Vec<_> = exp_actions
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, exp_action_targets[i]))
        .collect();

    assert_eq!(actions.len(), action_len);
    for i in 0..action_len {
        let (x, y, raw) = action_std[i];
        let action = &actions[i];

        assert_matches!(action, Elimination(EliminationDetails{x,y,target})if target.get_raw()==raw);
    }
    // // if confirmation
    // assert_eq!(actions.len(), action_len);
    // for i in 0..action_len {
    //     let (x, y, raw) = action_std[i];
    //     let action = &actions[i];
    //     assert_matches!(action, confirmation(ConfirmationDetails{x,y,target:raw});
    // }

    // house_clue data
    let house_clues_len = exp_house_clues.len();

    assert_eq!(house_clues.len(), house_clues_len);
    for i in 0..house_clues_len {
        assert_eq!(house_clues[i], exp_house_clues[i]);
    }

    // candidate_clue data
    let clues_len = exp_candidate_clues.len();
    let clues_std: Vec<_> = exp_candidate_clues
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, exp_candidate_masks[i]))
        .collect();
    assert_eq!(candidate_clues.len(), clues_len);
    for i in 0..clues_len {
        let (x, y, raw) = clues_std[i];
        let clue = &candidate_clues[i];
        assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
    }
}

pub fn test_function_c(
    solver: impl Solver,
    raws: [u16; 81],
    exp_actions: Vec<(usize, usize)>,
    exp_action_targets: Vec<u16>,
    exp_house_clues: Vec<House>,
    exp_candidate_clues: Vec<(usize, usize)>,
    exp_candidate_masks: Vec<u16>,
) {
    // raws
    let game_board = GameBoard::from_array(raws);
    // solver type
    let Solution {
        actions,
        house_clues,
        candidate_clues,
        solver_id: _,
    } = solver.solve(&game_board).unwrap();

    // action data
    let action_len = exp_actions.len();
    let action_std: Vec<_> = exp_actions
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, exp_action_targets[i]))
        .collect();

    assert_eq!(actions.len(), action_len);
    for i in 0..action_len {
        let (x, y, raw) = action_std[i];
        let action = &actions[i];
        assert_matches!(
            action,
            Confirmation(ConfirmationDetails { x, y, target: raw })
        );
    }

    // house_clue data
    let house_clues_len = exp_house_clues.len();

    assert_eq!(house_clues.len(), house_clues_len);
    for i in 0..house_clues_len {
        assert_eq!(house_clues[i], exp_house_clues[i]);
    }

    // candidate_clue data
    let clues_len = exp_candidate_clues.len();
    let clues_std: Vec<_> = exp_candidate_clues
        .iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, exp_candidate_masks[i]))
        .collect();
    assert_eq!(candidate_clues.len(), clues_len);
    for i in 0..clues_len {
        let (x, y, raw) = clues_std[i];
        let clue = &candidate_clues[i];
        assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
    }
}
