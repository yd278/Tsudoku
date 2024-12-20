#![allow(unused_variables)] // 禁用本文件中所有 unused_variables 警告
#![allow(dead_code)]

mod coord;
pub use coord::Coord;

mod blank_cell;
use blank_cell::{BitMap, BlankCell};

enum Cell {
    Printed(u8),
    Blank(BlankCell),
}

struct GameBoard {
    grid: [[Cell; 9]; 9],
}

impl GameBoard {
    // check if the given target shouldn't be deleted
    // the caller should ensure that the cell is a blank cell
    pub fn check_pencil_mark_deletion_error(&self, x: usize, y: usize, target: u8) -> bool {
        matches!(self.grid[x][y], Cell::Blank(ref cell) if target == cell.get_answer())
    }

    // check if this cell collides with the target
    fn check_cell_collision(&self, x: usize, y: usize, target: u8) -> Option<(usize, usize)> {
        match &self.grid[x][y] {
            Cell::Printed(ans) if target == *ans => Some((x, y)),
            Cell::Blank(blank_cell) if blank_cell.check_collision(target) => Some((x, y)),
            _ => None,
        }
    }

    // take an cell at (x,y) and return the vector of coordinates that collide with the target
    // the caller should ensure that the cell is a blank cell

    fn check_clue_collision(&self, x: usize, y: usize, target: u8) -> Vec<(usize, usize)> {
        let box_coords = Coord::iter_box(x, y);

        Coord::row(x)
            .filter(move |&(_, j)| j != y)
            .chain(Coord::col(y).filter(move |&(i, _)| i != x))
            .chain(box_coords)
            .filter_map(|(xi, yi)| self.check_cell_collision(xi, yi, target))
            .collect()
    }

    // check if the given target shouldn't be added
    // the caller should ensure that the cell is a blank cell with no pen mark
    pub fn check_pencil_mark_addition_collision(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Vec<(usize, usize)> {
        self.check_clue_collision(x, y, target)
    }
    // check if the given target shouldn't be set as pen mark
    // the caller should ensure that the cell is a blank with no pen mark
    pub fn check_pen_mark_addition_error(
        &self,
        x: usize,
        y: usize,
        target: u8,
    ) -> Option<Vec<(usize, usize)>> {
        match &self.grid[x][y] {
            Cell::Blank(cell) if target != cell.get_answer() => {
                Some(self.check_clue_collision(x, y, target))
            }
            _ => None,
        }
    }

    // delete target in a cell's candidate list
    // and mark it as user deleted if user_deleted_flag is true
    fn delete_candidate(&mut self, x: usize, y: usize, target: u8, user_deleted_flag: bool) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            cell.modify(|candidates, user_deleted| {
                if candidates.contains(target) {
                    candidates.remove(target);
                    if user_deleted_flag {
                        user_deleted.insert(target);
                    }
                }
            });
        }
    }

    // erase an pencil mark in given cell by user
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is in the candidate set
    pub fn erase_pencil_mark(&mut self, x: usize, y: usize, target: u8) {
        self.delete_candidate(x, y, target, true);
    }

    // add an pencil mark in given cell by user
    // and remove the user deleted flag
    // the caller should ensure that the cell is a blank cell with no pen mark and the target is not in the candidate set
    pub fn add_pencil_mark(&mut self, x: usize, y: usize, target: u8) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            cell.modify(|candidates, user_deleted| {
                if !candidates.contains(target) {
                    candidates.insert(target);
                    user_deleted.remove(target);
                }
            });
        }
    }
    // set a cell as pen mark by user
    // the caller should ensure that the cell is a blank cell with no pen mark
    pub fn add_pen_mark(&mut self, x: usize, y: usize, target: u8) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
            cell.set_pen_mark(target);

            let box_coords = Coord::iter_box(x, y);
            Coord::row(x)
                .filter(move |&(_, j)| j != y)
                .chain(Coord::col(y).filter(move |&(i, _)| i != x))
                .chain(box_coords)
                .for_each(|(xi, yi)| self.delete_candidate(xi, yi, target, false));
        }
    }
    // erase a pen mark in given cell by user
    // the caller should ensure that the cell is a blank cell with a pen mark
    pub fn erase_pen_mark(&mut self, x: usize, y: usize) {
        if let Cell::Blank(cell) = &mut self.grid[x][y] {
 
            if let Some(target) = cell.get_pen_mark() {
                cell.erase_pen_mark();
                let mut possible_candidates =  BitMap::all();
    
                let box_coords = Coord::iter_box(x, y);
    
                Coord::row(x)
                    .filter(move |&(_, j)| j != y)
                    .chain(Coord::col(y).filter(move |&(i, _)| i != x))
                    .chain(box_coords)
                    .for_each(|(xi, yi)| match self.grid[xi][yi] {
                        Cell::Printed(ans) => {
                            possible_candidates.remove(ans);
                        }
                        Cell::Blank(ref mut cell) => {
                            cell.update_or_collide(target, &mut possible_candidates);
                        }
                    });
            }   
        }
    }

    // fn update_or_collide(&mut self, x: usize, y: usize, possible_candidates: &mut u16, target: u8) {
    //     match &mut self.grid[x][y] {
    //         Cell::Blank {
    //             pen_mark: Some(mark),
    //             ..
    //         } => {
    //             *possible_candidates &= !Self::candidate_mask(*mark);
    //         }
    //         Cell::Printed(ans) => {
    //             *possible_candidates &= !Self::candidate_mask(*ans);
    //         }
    //         Cell::Blank {
    //             pen_mark: None,
    //             ref mut candidates,
    //             user_deleted,
    //             ..
    //         } => {
    //             if *candidates & Self::candidate_mask(target) == 0
    //                 && *user_deleted & Self::candidate_mask(target) == 0
    //             {
    //                 *candidates |= Self::candidate_mask(target);
    //             }
    //         }
    //     }
    // }
    // pub fn erase_pen_mark(&mut self, x: usize, y: usize) {
    //     let mut possible_candidates = Self::ALL_CANDIDATES;
    //     if let Cell::Blank {
    //         ref mut pen_mark, ..
    //     } = &mut self.grid[x][y]
    //     {
    //         let mut target = 0;
    //         let cur_box = (x / 3) * 3 + y / 3;
    //         if let Some(mark) = pen_mark {
    //             target = *mark;
    //         }
    //         *pen_mark = None;
    //         (0..9)
    //             .filter(|&i| i != y)
    //             .for_each(|i| self.update_or_collide(x, i, &mut possible_candidates, target));
    //         (0..9)
    //             .filter(|&i| i != x)
    //             .for_each(|i| self.update_or_collide(i, y, &mut possible_candidates, target));
    //         Coord::iter_box(cur_box)
    //             .filter(|Coord { x: xi, y: yi }| *xi != x || *yi != y)
    //             .for_each(|Coord { x, y }| {
    //                 self.update_or_collide(x, y, &mut possible_candidates, target)
    //             });
    //     }
    //     self.modify_blank(x, y, |candidates, user_deleted| {
    //         *candidates = possible_candidates & !*user_deleted;
    //     });
    // }
}
