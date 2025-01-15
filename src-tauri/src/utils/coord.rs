use crate::utils::House;
pub struct Coord;

impl Coord {
    pub fn house(h: &House) -> Box<dyn Iterator<Item = (usize, usize)>> {
        match *h {
            House::Box(b) => Box::new(Self::box_coords(b)),
            House::Col(c) => Box::new(Self::col(c)),
            House::Row(r) => Box::new(Self::row(r)),
        }
    }

    pub fn is_in_house(x: usize, y: usize, h: &House) -> bool {
        match *h {
            House::Col(c) => y == c,
            House::Row(r) => x == r,
            House::Box(b) => Self::get_box_id(x, y) == b,
        }
    }

    pub fn from_house_and_index(h: &House, i: usize) -> (usize, usize) {
        match *h {
            House::Col(c) => (i, c),
            House::Row(r) => (r, i),
            House::Box(b) => {
                let start_x = (b / 3) * 3;
                let start_y = (b % 3) * 3;
                (start_x + i / 3, start_y + i % 3)
            }
        }
    }

    pub fn get_index_from_house(h: &House, x: usize, y: usize) -> usize {
        match *h {
            House::Col(c) => x,
            House::Row(r) => y,
            House::Box(b) => {
                let x_offset = x / 3;
                let y_offset = y / 3;
                x_offset * 3 + y_offset
            }
        }
    }

    pub fn row(x: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |y| (x, y))
    }

    pub fn col(y: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..9).map(move |x| (x, y))
    }

    pub fn box_coords(box_id: usize) -> impl Iterator<Item = (usize, usize)> {
        let start_x = (box_id / 3) * 3;
        let start_y = (box_id % 3) * 3;
        (0..9).map(move |i| (start_x + i / 3, start_y + i % 3))
    }

    pub fn get_box_id(x: usize, y: usize) -> usize {
        (x / 3) * 3 + (y / 3)
    }

    pub fn iter_box(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let box_id = (x / 3) * 3 + (y / 3);
        Self::box_coords(box_id).filter(move |(xi, yi)| *xi != x || *yi != y)
    }

    pub fn seeable_cells(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        Self::row(x)
            .filter(move |&(_, j)| j != y)
            .chain(Self::col(y).filter(move |&(i, _)| i != x))
            .chain(Self::iter_box(x, y))
    }
}
