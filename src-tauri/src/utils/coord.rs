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

    pub fn get_box_id_by_tuple((x,y):(usize,usize)) -> usize{
        Self::get_box_id(x, y)
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
    pub fn components_array(x: usize, y: usize) -> [usize; 3] {
        [x, y, Self::get_box_id(x, y)]
    }

    pub fn all_cells() -> impl Iterator<Item = (usize, usize)> {
        (0..9).flat_map(|x| (0..9).map(move |y| (x, y)))
    }
    pub fn sees(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        (x1 == x2 || y1 == y2 || Coord::get_box_id(x1, y1) == Coord::get_box_id(x2, y2))
            && !(x1 == x2 && y1 == y2)
    }
    pub fn components_proj(x: usize, y: usize, dim: usize) -> usize {
        match dim {
            0 => x,
            1 => y,
            2 => Self::get_box_id(x, y),
            _ => panic!(),
        }
    }
    pub fn pinched_by(
        px: usize,
        py: usize,
        qx: usize,
        qy: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut res = vec![];
        if px == qx && py == qy {
            return res.into_iter();
        }
        if px == qx {
            for y in 0..9 {
                if y == py || y == qy {
                    continue;
                }
                res.push((px, y));
            }
            if py / 3 == qy / 3 {
                for (ux, uy) in Coord::iter_box(px, py).filter(|&(ux, _)| ux != px) {
                    res.push((ux, uy));
                }
            }
            return res.into_iter();
        }
        if py == qy {
            for x in 0..9 {
                if x == px || x == qx {
                    continue;
                }
                res.push((x, py));
            }
            if px / 3 == qx / 3 {
                for (ux, uy) in Coord::iter_box(px, py).filter(|&(_, uy)| uy != py) {
                    res.push((ux, uy));
                }
            }
            return res.into_iter();
        }
        vec![(px, qy), (qx, py)].into_iter()
    }
}
