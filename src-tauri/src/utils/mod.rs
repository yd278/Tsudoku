#[derive(Clone, Copy)]
pub struct BitMap(u16);

impl BitMap {
    pub fn all() -> Self{
        BitMap(0b111111111)
    }

    pub fn new() -> Self {
        BitMap(0)
    }

    pub fn from(num: u8) -> Self {
        BitMap(1 << num)
    }

    pub fn contains(self, num: u8) -> bool {
        self.0 & (1 << num) != 0
    }

    pub fn insert(&mut self, num: u8) {
        self.0 |= 1 << num;
    }

    pub fn remove(&mut self, num: u8) {
        self.0 &= !(1 << num);
    }

    pub fn count(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn trailing_zeros(self) -> u8 {
        self.0.trailing_zeros() as u8
    }
}

pub struct Coord;

impl Coord {
    pub fn row(x: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |y| (x,y))
    }

    pub fn col(y: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |x| (x,y))
    }

    pub fn box_coords(box_id: usize) -> impl Iterator<Item = (usize, usize)> {
        let start_x = (box_id / 3) * 3;
        let start_y = (box_id % 3) * 3;
        (0..9).map(move |i| (
             start_x + i / 3,
             start_y + i % 3,
        ))
    }

    pub fn iter_box(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let box_id = (x / 3) * 3 + (y / 3);
        Self::box_coords(box_id).filter(move |(xi,yi)| *xi != x || *yi != y)
    }
    
    pub fn seeable_cells(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        Self::row(x).filter(move |&(_, j)| j != y)
            .chain(Self::col(y).filter(move |&(i, _)| i != x))
            .chain(Self::iter_box(x, y))
    }
}
