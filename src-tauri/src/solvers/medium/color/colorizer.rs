use crate::{
    game_board::GameBoard,
    utils::{Coord, HouseType},
};
#[derive(Debug)]
pub enum Error {
    FlippingUncoloredCell,
    ColoringConflict,
    Visited,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Color {
    Light(usize),
    Dark(usize),
    Uncolored,
}

impl Color {
    pub fn other(&self) -> Result<Self, Error> {
        match self {
            Color::Light(num) => Ok(Self::Dark(*num)),
            Color::Dark(num) => Ok(Self::Light(*num)),
            Color::Uncolored => Err(Error::FlippingUncoloredCell),
        }
    }

    pub fn colored(&self) -> bool {
        !matches!(self, Self::Uncolored)
    }

    pub fn as_index(&self) -> usize {
        match self {
            Color::Light(num) => *num << 1,
            Color::Dark(num) => *num << 1 | 1,
            Color::Uncolored => unreachable!("shouldn't convert uncolored to index"),
        }
    }
}
pub(super) struct Colorizer {
    pub color: [[Color; 9]; 9],
    pub target: usize,
    pub color_cnt: usize,
}

impl Colorizer {
    pub fn new(target: usize) -> Self {
        Self {
            color: [[Color::Uncolored; 9]; 9],
            target,
            color_cnt: 0,
        }
    }

    fn set_color(&mut self, x: usize, y: usize, color: Color) -> Result<(), Error> {
        if self.color[x][y].colored() {
            if self.color[x][y] == color {
                Err(Error::Visited)
            } else {
                Err(Error::ColoringConflict)
            }
        } else {
            self.color[x][y] = color;
            Ok(())
        }
    }

    fn colorize_rec(&mut self, game_board: &GameBoard, x: usize, y: usize) -> Result<(), Error> {
        for dim in 0..3 {
            if let Some((nx, ny)) =
                game_board.get_hard_link(x, y, self.target, HouseType::from_dim(dim))
            {
                let new_color = self.color[x][y].other()?;
                if let Err(e) = self
                    .set_color(nx, ny, new_color)
                    .and_then(|_| self.colorize_rec(game_board, nx, ny))
                {
                    match e {
                        Error::FlippingUncoloredCell | Error::ColoringConflict => return Err(e),
                        Error::Visited => (),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn colorize(&mut self, game_board: &GameBoard) -> Result<(), Error> {
        for (x, y) in Coord::all_cells() {
            if !self.color[x][y].colored() && game_board.hard_linked(x, y, self.target) {
                self.set_color(x, y, Color::Light(self.color_cnt))
                    .expect("this should not fail as cell(x,y) is not colored");
                self.colorize_rec(game_board, x, y)?;
                self.color_cnt += 1;
            }
        }
        Ok(())
    }
}
