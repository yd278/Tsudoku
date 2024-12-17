use std::vec;

enum Mark {
    PencilMark(u16),
    PenMark { candidates: u16, mark: u8 },
    PrintMark(u8),
}
struct Cell {
    ans: u8,
    mark: Mark,
}

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
    pub fn check_candidate_deletion_error(&self, x: usize, y: usize, target: u8) -> bool {
        self.grid[x][y].ans == target
    }
    fn check_collision_with_exist_clues(&self, x: usize, y: usize, target: u8) -> Vec<Coord> {
        let mut res: Vec<Coord> = Vec::new();

        // check row
        for i in 0usize..9 {
            if i == y {
                continue;
            }

            let mark = &self.grid[x][i].mark;
            match mark {
                Mark::PencilMark(_) => continue,
                Mark::PenMark { mark, .. } if target == *mark => {
                    res.push(Coord { x, y: i });
                }
                Mark::PrintMark(num) if target == *num => {
                    res.push(Coord { x, y: i });
                }
                _ => {}
            }
        }

        // check column
        for j in 0usize..9 {
            if j == x {
                continue;
            }

            let mark = &self.grid[j][y].mark;
            match mark {
                Mark::PencilMark(_) => continue,
                Mark::PenMark { mark, .. } if target == *mark => {
                    res.push(Coord { x: j, y });
                }
                Mark::PrintMark(num) if target == *num => {
                    res.push(Coord { x: j, y });
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
            let mark = &self.grid[xi][yi].mark;
            match mark {
                Mark::PencilMark(_) => continue,
                Mark::PenMark { mark, .. } if target == *mark => {
                    res.push(Coord { x: xi, y: yi });
                }
                Mark::PrintMark(num) if target == *num => {
                    res.push(Coord { x: xi, y: yi });
                }
                _ => {}
            }
        }

        res
    }
    pub fn check_candidate_addition_collision(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Option<Vec<Coord>> {
        let res = self.check_collision_with_exist_clues(x, y, target);
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    pub fn check_set_number_error(&self, x: usize, y: usize, target: u8) -> Option<Vec<Coord>> {
        let cell = &self.grid[x][y];
        if target == cell.ans {
            return None;
        };
        let collisions = self.check_collision_with_exist_clues(x, y, target);
        Some(collisions)
    }
    


}
