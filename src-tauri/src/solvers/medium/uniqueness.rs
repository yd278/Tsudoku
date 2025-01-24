
use crate::{
    game_board::GameBoard,
    impl_with_id,
    solvers::{
        solution::{Action, Candidate, EliminationDetails, Solution},
        Solver,
    },
    utils::{Coord, House},
};

impl_with_id!(UniquenessTest1);
struct UniquenessTest1 {
    id: usize,
}

impl Solver for UniquenessTest1 {
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        Coord::all_cells()
            .filter_map(|(x, y)| {
                game_board
                    .get_candidates(x, y)
                    .and_then(|candidates| (candidates.count() == 2).then_some((x, y, candidates)))
            })
            .find_map(|(px, py, bi_value)| {
                //closure: returns Option<Solution>
                (0..9)
                    .filter(|&qy| qy != py)
                    .filter_map(|qy| {
                        game_board.get_candidates(px, qy).and_then(|candidates| {
                            (candidates == bi_value).then_some((qy, py / 3 == qy / 3))
                        })
                    })
                    .flat_map(|(qy, same_box_flag)| {
                        (0..9)
                            .filter(|&rx| rx != px)
                            .filter(move |&rx| (rx / 3 == px / 3) != same_box_flag)
                            .filter_map(move |rx| {
                                game_board.get_candidates(rx, py).and_then(|candidates| {
                                    (candidates == bi_value).then_some((rx, qy))
                                })
                            })
                    })
                    .map(|(rx, qy)| {
                        game_board.get_candidates(rx, qy).and_then(|candidates| {
                            (candidates.intersect(&bi_value) == bi_value).then_some(Solution {
                                actions: vec![Action::Elimination(EliminationDetails {
                                    x: rx,
                                    y: qy,
                                    target: bi_value,
                                })],
                                house_clues: vec![House::Row(px), House::Row(rx), House::Col(py),House::Col(qy)],
                                candidate_clues: vec![
                                    Candidate::new(px, py, bi_value),
                                    Candidate::new(px, qy, bi_value),
                                    Candidate::new(rx, py, bi_value),
                                ],
                                solver_id: self.id,
                            })
                        })
                    })
                    .find_map(|solution| solution)
            })
    }
}

// UT2:
// 枚举一条线
// 筛选出里面的bi-values
// 因为同种bi-v最多两个所以可以hash map……？
// 手写状态压缩的map……？
// 让 gameboard 提前准备好bi-values？
#[cfg(test)]
mod uniqueness_test{
    use super::*;
    use crate::solvers::solution::Action::Elimination;
    use crate::utils::House::{ Col, Row};
    use crate::{game_board::GameBoard, utils::House};
    fn test_function(
        solver: impl Solver,
        raws: [u16; 81],
        target: u16,
        exp_actions: Vec<(usize, usize)>,
        exp_house_clues: Vec<House>,
        exp_candidate_clues: Vec<(usize, usize)>,
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
            .into_iter()
            .map(|(a, b)| (a, b, target))
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
            .into_iter()
            .map(|(a, b)| (a, b, target))
            .collect();
        assert_eq!(candidate_clues.len(), clues_len);
        for i in 0..clues_len {
            let (x, y, raw) = clues_std[i];
            let clue = &candidate_clues[i];
            assert_matches!(clue,Candidate{x,y,candidates} if candidates.get_raw()==raw);
        }
    }
    #[test]
    fn uniqueness_test_1(){
        test_function(
            UniquenessTest1::with_id(1),
            [64,16,256,136,4,1,2,32,136,129,8,32,16,192,2,256,65,4,4,130,131,392,32,328,16,65,136,16,480,129,169,448,4,41,384,2,393,482,135,169,448,104,41,388,16,393,416,133,2,16,40,41,388,64,2,4,64,288,8,288,128,16,1,32,1,16,4,2,128,64,8,256,384,384,8,64,1,16,4,2,32],
            136,
            vec![ (2,3)],
            vec![Row(0),Row(2),Col(8),Col(3)],
            vec![(0,8), (0,3), (2,8),],
        );
    }
}