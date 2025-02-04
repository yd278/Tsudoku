//枚举一个pen mark，P
// 找到能形成UR的两个同数字的pen mark
// 夹角的地方如果有p，消除


use crate::{game_board::GameBoard, solvers::{solution::{Action, EliminationDetails, Solution}, Solver}, utils::{BitMap, Coord, House}};

use super::{iter_pen_cell, AvoidableRectangle1, PenCell};

#[derive(Clone, Copy)]
struct BaseRow {
    x: usize,
    py: usize,
    qy: usize,
    target: usize,
    pincer: usize,
}

impl BaseRow {
    pub fn new(p: PenCell, qy: usize, pincer: usize) -> Self {
        Self {
            x: p.x,
            py: p.y,
            qy,
            target: p.target,
            pincer,
        }
    }
}
struct AR1 {
    px: usize,
    py: usize,
    sx: usize,
    sy: usize,
    target: usize,
    pincer: usize,
}

impl AR1 {
    pub fn new(row: BaseRow, rx: usize) -> Self {
        Self {
            px: row.x,
            py: row.py,
            sx: rx,
            sy: row.qy,
            target: row.target,
            pincer: row.pincer,
        }
    }
    pub fn get_house_clues(&self) -> Vec<House> {
        vec![
            House::Row(self.px),
            House::Row(self.sx),
            House::Col(self.py),
            House::Col(self.sy),
        ]
    }
    pub fn get_actions(&self) -> Vec<Action> {
        vec![
            Action::Elimination(EliminationDetails {
                x: self.sx,
                y: self.sy,
                target: BitMap::from(self.target)
            }),
        ]
    }
    fn get_solution(&self,solver_id:usize) -> Solution{
        Solution{
            actions: self.get_actions(),
            house_clues:self.get_house_clues(),
            candidate_clues: vec![],
            solver_id,
        }
    }
}

impl AvoidableRectangle1 {

    fn iter_q(game_board: &GameBoard, p:PenCell) -> impl Iterator<Item = BaseRow> +'_{
        (0..9).filter(move |&qy| qy!=p.y)
        .filter_map(move |qy|{
            game_board.get_pen_mark(p.x, qy).map(|pincer|{
                BaseRow::new(p,qy,pincer)
            })
        })
    }

    fn iter_ar(game_board: &GameBoard, row: BaseRow) -> impl Iterator<Item = AR1> + '_{
        (0..9).filter(move |&rx| rx!=row.x && (rx / 3 == row.x / 3) != (row.py / 3 == row.qy / 3))
        .filter_map(move |rx|{
            game_board.get_pen_mark(rx , row.py).map(|num|{
                (num==row.pincer).then_some(AR1::new(row, rx))
            })
            .flatten()
        })
    }

}

impl Solver for AvoidableRectangle1{
    fn solve(&self, game_board: &GameBoard) -> Option<Solution> {
        iter_pen_cell(game_board)
        .flat_map(|p|Self::iter_q(game_board, p))
        .flat_map(|row|Self::iter_ar(game_board, row))
        .find_map(|ar|
            game_board.contains_candidate(ar.sx, ar.sy, ar.target).then_some(ar.get_solution(self.id))
        )
    }
}
