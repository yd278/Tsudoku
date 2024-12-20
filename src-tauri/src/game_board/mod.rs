#![allow(unused_variables)] // 禁用本文件中所有 unused_variables 警告
#![allow(dead_code)] // 禁用本文件中所有 dead_code 警告
enum Cell {
    Printed(u8),
    Blank {
        ans: u8,
        pen_mark: Option<u8>,
        candidates: u16,
        user_deleted: u16,
    },
}

#[derive(Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from_box(box_id: usize, cell: usize) -> Self {
        let row_shift = (box_id / 3) * 3;
        let col_shift = (box_id % 3) * 3;
        let row = cell / 3;
        let col = cell % 3;
        Coord {
            x: (row_shift + row),
            y: col_shift + col,
        }
    }
}

struct GameBoard {
    grid: [[Cell; 9]; 9],
}

impl GameBoard {
    pub fn check_pencil_mark_deletion_error(&self, x: usize, y: usize, target: u8) -> bool {
        if let Cell::Blank {
            ans,
            pen_mark: None,
            ..
        } = &self.grid[x][y]
        {
            if target == *ans {
                return true;
            } else {
                return false;
            }
        };
        false
    }

    fn check_clue_collision(&self, x: usize, y: usize, target: u8) -> Vec<Coord> {
        let mut res: Vec<Coord> = Vec::new();

        // check row
        for i in 0usize..9 {
            if i == y {
                continue;
            }
            let cell = &self.grid[x][i];
            match cell {
                Cell::Printed(ans) => {
                    if target == *ans {
                        res.push(Coord { x, y: i });
                    }
                }
                Cell::Blank {
                    ans,
                    pen_mark: Some(mark),
                    ..
                } => {
                    if target == *mark {
                        res.push(Coord { x, y: i });
                    }
                }
                _ => {}
            }
        }

        // check column
        for j in 0usize..9 {
            if j == x {
                continue;
            }

            let cell = &self.grid[j][y];
            match cell {
                Cell::Printed(ans) => {
                    if target == *ans {
                        res.push(Coord { x: j, y });
                    }
                }
                Cell::Blank {
                    ans,
                    pen_mark: Some(mark),
                    ..
                } => {
                    if target == *mark {
                        res.push(Coord { x: j, y });
                    }
                }
                _ => {}
            }
        }

        //check box
        let cur_box = (x / 3) * 3 + y / 3;
        for i in 0usize..9 {
            let Coord { x: xi, y: yi } = Coord::from_box(cur_box, i);
            if xi == x && yi == y {
                continue;
            }
            // let mark = &self.grid[xi][yi].mark;
            // match mark {
            //     Mark::PenMark { mark, .. } if target == *mark => {
            //         res.push(Coord { x: xi, y: yi });
            //     }
            //     Mark::PrintMark(num) if target == *num => {
            //         res.push(Coord { x: xi, y: yi });
            //     }
            //     _ => {}
            // }
            let cell = &self.grid[xi][yi];
            match cell {
                Cell::Printed(ans) => {
                    if target == *ans {
                        res.push(Coord { x: xi, y: yi });
                    }
                }
                Cell::Blank {
                    ans,
                    pen_mark: Some(mark),
                    ..
                } => {
                    if target == *mark {
                        res.push(Coord { x: xi, y: yi });
                    }
                }
                _ => {}
            }
        }

        res
    }

    pub fn check_pencil_mark_addition_collision(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Option<Vec<Coord>> {
        let res = self.check_clue_collision(x, y, target);
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    pub fn check_pen_mark_addition_error(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Option<Vec<Coord>> {
        let cell = &self.grid[x][y];
        if let Cell::Blank { ans, .. } = cell {
            if target != *ans {
                let res = self.check_clue_collision(x, y, target);
                return Some(res);
            } else {
                return None;
            }
        } else {
            None
        }
    }

    fn delete_candidate(&mut self, x: usize, y: usize, target: u8, user_deleted_flag: bool) -> () {
        let cell = &mut self.grid[x][y];
        if let Cell::Blank {
            pen_mark: None,
            ref mut candidates,
            ref mut user_deleted,
            ..
        } = cell
        {
            if *candidates & (1 << target) != 0 {
                *candidates &= !(1 << target);
                if user_deleted_flag {
                    *user_deleted |= 1 << target;
                }
            }
        }
    }

    pub fn erase_pencil_mark(&mut self, x: usize, y: usize, target: u8) -> () {
        self.delete_candidate(x, y, target, true);
    }

    pub fn add_pencil_mark(&mut self, x: usize, y: usize, target: u8) -> () {
        let cell = &mut self.grid[x][y];
        if let Cell::Blank {
            pen_mark: None,
            ref mut candidates,
            ref mut user_deleted,
            ..
        } = cell
        {
            if *candidates & (1 << target) == 0 {
                *candidates |= 1 << target;
                *user_deleted &= !(1 << target);
            }
        }
    }

    pub fn add_pen_mark(&mut self, x: usize, y: usize, target: u8) -> () {
        let cell = &mut self.grid[x][y];
        if let Cell::Blank {
            ref mut pen_mark, ..
        } = cell
        {
            *pen_mark = Some(target);
        }
        let cur_box = (x / 3) * 3 + y / 3;
        for i in 0usize..9 {
            self.delete_candidate(x, i, target, false);
            self.delete_candidate(i, y, target, false);
            let Coord { x: xi, y: yi } = Coord::from_box(cur_box, i);
            self.delete_candidate(xi, yi, target, false);
        }
    }
    fn update_candidate(&mut self, x: usize, y: usize) -> () {

        if let Cell::Blank {
            pen_mark: Some(_),
            ..
        } = &self.grid[x][y]{
            return;
        }

        if let Cell::Printed(_) = &self.grid[x][y]{
            return;
        }

        let mut possible_candidate:u16 = 0b111111111;
        for target in 0u8..9{
            let res = self.check_clue_collision(x, y, target);
            if ! res.is_empty(){
                possible_candidate &= !(1 << target);
            }
        }
        let cell = &mut self.grid[x][y];
        if let Cell::Blank {
            candidates,
            user_deleted,
            pen_mark: None,
            ..
        } = cell
        {
            *candidates = possible_candidate;
            *candidates &= !*user_deleted;
        }
    }
    pub fn erase_pen_mark(&mut self, x: usize, y: usize) -> () {
        let cell = &mut self.grid[x][y];
        if let Cell::Blank {
            ref mut pen_mark, ..
        } = cell
        {
            if let Some(mark) = *pen_mark {
                let target = mark;
                *pen_mark = None;
                let cur_box = (x / 3) * 3 + y / 3;
                for i in 0usize..9 {
                    self.update_candidate(x, i);
                    self.update_candidate(i, y);
                    let Coord { x: xi, y: yi } = Coord::from_box(cur_box, i);
                    self.update_candidate(xi, yi);
                }
            }
        }
    }
}
