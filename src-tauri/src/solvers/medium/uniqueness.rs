
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
                                    target: candidates.symmetric_difference(&bi_value),
                                })],
                                house_clues: vec![House::Row(px), House::Col(py)],
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