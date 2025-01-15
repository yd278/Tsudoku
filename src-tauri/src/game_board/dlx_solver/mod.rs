mod dlx_node;
pub mod dlx_solution;
use super::super::game_board::{Cell, GameBoard};

use dlx_node::DLXNode;
use dlx_node::Link;
use dlx_solution::DLXSolution;
use std::cell::RefCell;
use std::rc::Rc;
pub struct DLXSolver {
    header: Rc<RefCell<DLXNode>>,       // 矩阵的头节点
    columns: Vec<Rc<RefCell<DLXNode>>>, // 列头节点
    col_count: Vec<usize>,              // 每列的节点数
}

impl DLXSolver {
    fn new(num_columns: usize) -> Self {
        let header = DLXNode::new(usize::MAX, usize::MAX);
        let mut columns = Vec::new();
        let col_count = vec![0; num_columns];

        // 初始化列头节点
        let mut prev = header.clone();
        for i in 0..num_columns {
            let col_node = DLXNode::new(0, i);
            col_node.borrow_mut().left = Some(prev.clone());
            prev.borrow_mut().right = Some(col_node.clone());
            columns.push(col_node.clone());
            col_node.borrow_mut().up = Some(col_node.clone());
            col_node.borrow_mut().down = Some(col_node.clone());

            prev = col_node;
        }

        // 连接首尾做成循环链表
        prev.borrow_mut().right = Some(header.clone());
        header.borrow_mut().left = Some(prev);

        DLXSolver {
            header,
            columns,
            col_count,
        }
    }
    // 在矩阵中添加一行
    // Parameters:
    // row_id: 行号
    // cols: 该行中包含的列号

    fn add_row(&mut self, row_id: usize, cols: &[usize]) {
        let mut first: Link = None;
        let mut prev: Link = None;

        for &col_id in cols {
            let col_node = self.columns[col_id].clone();
            let new_node = DLXNode::new(row_id, col_id);

            //将新节点插入到列中
            new_node.borrow_mut().up = col_node.borrow().up.clone();
            new_node.borrow_mut().down = Some(col_node.clone());

            new_node.borrow_mut().up.as_ref().unwrap().borrow_mut().down = Some(new_node.clone());

            col_node.borrow_mut().up = Some(new_node.clone());

            self.col_count[col_id] += 1;

            // 将新节点的列头指针指向列头节点
            new_node.borrow_mut().column = Some(col_node.clone());

            // 连接行
            //如果prev存在，建立prev和new_node的双向连接
            // 否则，将new_node记作first
            if let Some(prev_node) = &prev {
                new_node.borrow_mut().left = Some(prev_node.clone());
                prev_node.borrow_mut().right = Some(new_node.clone());
            } else {
                first = Some(new_node.clone());
            }

            prev = Some(new_node);
        }
        // 连接行的首尾
        if let Some(first_node) = &first {
            if let Some(last_node) = &prev {
                first_node.borrow_mut().left = Some(last_node.clone());
                last_node.borrow_mut().right = Some(first_node.clone());
            }
        }
    }
    // 覆盖某一列
    // Parameters:
    // col: 需要覆盖的那一列的列头节点

    fn cover(&mut self, col: Rc<RefCell<DLXNode>>) {
        // 从列头链表中移除该列头结点
        col.borrow().left.as_ref().unwrap().borrow_mut().right = col.borrow().right.clone();
        col.borrow().right.as_ref().unwrap().borrow_mut().left = col.borrow().left.clone();

        // 移除列中的所有行
        let mut row = col.borrow().down.clone();
        while let Some(r) = row {
            if Rc::ptr_eq(&r, &col) {
                break;
            }
            let mut node = r.borrow().right.clone();
            while let Some(n) = node {
                if Rc::ptr_eq(&n, &r) {
                    break;
                }
                n.borrow().up.as_ref().unwrap().borrow_mut().down = n.borrow().down.clone();
                n.borrow().down.as_ref().unwrap().borrow_mut().up = n.borrow().up.clone();
                self.col_count[n.borrow().col_id] -= 1;
                node = n.borrow().right.clone();
            }
            row = r.borrow().down.clone();
        }
    }

    // 恢复某一列
    // Parameters:
    // col: 需要恢复的那一列的列头节点
    fn uncover(&mut self, col: Rc<RefCell<DLXNode>>) {
        // 恢复列中的所有行
        let mut row = col.borrow().up.clone();
        while let Some(r) = row {
            if Rc::ptr_eq(&r, &col) {
                break;
            }
            let mut node = r.borrow().left.clone();
            while let Some(n) = node {
                if Rc::ptr_eq(&n, &r) {
                    break;
                }
                n.borrow().down.as_ref().unwrap().borrow_mut().up = Some(n.clone());
                n.borrow().up.as_ref().unwrap().borrow_mut().down = Some(n.clone());
                self.col_count[n.borrow().col_id] += 1;
                node = n.borrow().left.clone();
            }
            row = r.borrow().up.clone();
        }
        // 恢复列头结点
        col.borrow().left.as_ref().unwrap().borrow_mut().right = Some(col.clone());
        col.borrow().right.as_ref().unwrap().borrow_mut().left = Some(col.clone());
    }

    // 递归求解
    // Parameters:
    // acc: 用于存储当前解
    // Returns: dlx_solution::DLXSolution
    fn search(&mut self, acc: &mut Vec<usize>) -> DLXSolution {
        // 如果所有列都被覆盖，则找到一个解
        if Rc::ptr_eq(&self.header, self.header.borrow().right.as_ref().unwrap()) {
            return DLXSolution::Solution(acc.clone());
        }

        // 选择节点数最少的列
        let mut min = usize::MAX;
        let mut min_col = self.header.clone();
        let mut col = self.header.borrow().right.clone();
        while let Some(c) = col {
            if Rc::ptr_eq(&c, &self.header) {
                break;
            }
            if self.col_count[c.borrow().col_id] < min {
                min = self.col_count[c.borrow().col_id];
                min_col = c.clone();
            }
            col = c.borrow().right.clone();
        }
        // 如果该列没有节点，则无解
        if min == 0 {
            return DLXSolution::NoSolution;
        }
        // 覆盖该列
        self.cover(min_col.clone());

        let mut tmp = None;
        let mut row = min_col.borrow().down.clone();
        while let Some(r) = row {
            //对于其中一行：
            if Rc::ptr_eq(&r, &min_col) {
                break;
            }
            // 将该行加入解中
            acc.push(r.borrow().row_id);
            // 将该行中的所有列覆盖
            let mut node = r.borrow().right.clone();
            while let Some(n) = node {
                if Rc::ptr_eq(&n, &r) {
                    break;
                }
                self.cover(n.borrow().column.clone().unwrap());
                node = n.borrow().right.clone();
            }
            // 递归搜索
            match self.search(acc) {
                DLXSolution::Solution(solution) => {
                    if let Some(t) = tmp {
                        return DLXSolution::MultipleSolutions;
                    } else {
                        tmp = Some(solution);
                    }
                }
                DLXSolution::MultipleSolutions => return DLXSolution::MultipleSolutions,
                _ => (),
            }
            // 回溯
            acc.pop();
            // 恢复该行中的所有列
            let mut node = r.borrow().left.clone();
            while let Some(n) = node {
                if Rc::ptr_eq(&n, &r) {
                    break;
                }
                self.uncover(n.borrow().column.clone().unwrap());
                node = n.borrow().left.clone();
            }
            // 下一行
            row = r.borrow().down.clone();
        }
        // 恢复该列
        self.uncover(min_col.clone());
        if let Some(t) = tmp {
            DLXSolution::Solution(t)
        } else {
            DLXSolution::NoSolution
        }
    }

    pub fn solve_sudoku(game_board: &mut GameBoard) -> Result<(), DLXSolution> {
        let mut solver = DLXSolver::new(324);
        let mut row_id = 0;
        let solution_mapping = &mut Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                let cell = &game_board.grid[i][j];
                match cell {
                    Cell::Printed(ans) => {
                        let cols = vec![
                            i * 9 + j,
                            81 + i * 9 + *ans,
                            162 + j * 9 + *ans,
                            243 + (i / 3 * 3 + j / 3) * 9 + *ans,
                        ];
                        solver.add_row(row_id, &cols);
                        solution_mapping.push((i, j, *ans));
                        row_id += 1;
                    }
                    Cell::Blank(cell) => {
                        for k in 0..9 {
                            let cols = vec![
                                i * 9 + j,
                                81 + i * 9 + k,
                                162 + j * 9 + k,
                                243 + (i / 3 * 3 + j / 3) * 9 + k,
                            ];
                            solver.add_row(row_id, &cols);
                            solution_mapping.push((i, j, k));
                            row_id += 1;
                        }
                    }
                }
            }
        }
        match solver.search(&mut Vec::new()) {
            DLXSolution::Solution(solution) => {
                for row_id in solution {
                    let (i, j, ans) = solution_mapping[row_id];
                    if let Cell::Blank(cell) = &mut game_board.grid[i][j] {
                        cell.set_answer(ans);
                    }
                }
                Ok(())
            }
            DLXSolution::NoSolution => Err(DLXSolution::NoSolution),
            DLXSolution::MultipleSolutions => Err(DLXSolution::MultipleSolutions),
        }
    }
}
