use crate::utils::House;
pub struct Coord;

impl Coord {
    pub fn same(px: usize, py: usize, qx: usize, qy: usize) -> bool {
        (px == qx) && (py == qy)
    }
    pub fn house(h: &House) -> Box<dyn Iterator<Item = (usize, usize)>> {
        match *h {
            House::Box(b) => Box::new(Self::box_coords(b)),
            House::Col(c) => Box::new(Self::col(c)),
            House::Row(r) => Box::new(Self::row(r)),
        }
    }

    pub fn intersect(h1: House, h2: House) -> Box<dyn Iterator<Item = (usize, usize)>> {
        match h1 {
            House::Row(r) => match h2 {
                House::Row(_) => Box::new(std::iter::empty()),
                House::Col(c) => Box::new([(r, c)].into_iter()),
                House::Box(b) => {
                    if r / 3 == b / 3 {
                        let offset = (b % 3) * 3;
                        Box::new([(r, offset), (r, offset + 1), (r, offset + 2)].into_iter())
                    } else {
                        Box::new(std::iter::empty())
                    }
                }
            },
            House::Col(c) => match h2 {
                House::Row(r) => Box::new([(r, c)].into_iter()),
                House::Col(_) => Box::new(std::iter::empty()),
                House::Box(b) => {
                    if c / 3 == b % 3 {
                        let offset = (b / 3) * 3;
                        Box::new([(offset, c), (offset + 1, c), (offset + 2, c)].into_iter())
                    } else {
                        Box::new(std::iter::empty())
                    }
                }
            },
            House::Box(b) => match h2 {
                House::Row(r) => Self::intersect(h2, h1),
                House::Col(r) => Self::intersect(h2, h1),
                House::Box(_) => Box::new(std::iter::empty()),
            },
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
                let x_offset = x % 3;
                let y_offset = y % 3;
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

    pub fn get_box_id_by_tuple((x, y): (usize, usize)) -> usize {
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
            .chain(Self::iter_box(x, y).filter(move |&(i, j)| i != x && j != y))
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
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        // same cell, nothing should be returned
        if px == qx && py == qy {
            return Box::new(std::iter::empty());
        }
        let same_row = px == qx;
        let same_col = py == qy;
        let mut res = vec![];
        // same row, put all cells in the row in
        if same_row {
            res.extend((0..9).filter(|&y| y != py && y != qy).map(|y| (px, y)));
        }
        // same col, put all cells in the col in
        if same_col {
            res.extend((0..9).filter(|&x| x != px && x != qx).map(|x| (x, py)));
        }
        // same box:
        let box_id_p = Coord::get_box_id(px, py);
        let box_id_q = Coord::get_box_id(qx, qy);
        if box_id_p == box_id_q {
            // same box with same row, put other two rows in
            if same_row {
                for shift in 0..3 {
                    if box_id_p / 3 * 3 + shift != px {
                        res.extend(Self::intersect(
                            House::Row(box_id_p / 3 * 3 + shift),
                            House::Box(box_id_p),
                        ));
                    }
                }
            } else if same_col {
                // same box with wame col, put other two cols in
                for shift in 0..3 {
                    if box_id_p % 3 * 3 + shift != py {
                        res.extend(Self::intersect(
                            House::Col(box_id_p % 3 * 3 + shift),
                            House::Box(box_id_p),
                        ));
                    }
                }
            } else {
                // same box but not same row and same col, put all other cells in the box in
                res.extend(
                    Self::box_coords(box_id_p)
                        .filter(|&(x, y)| !((x == px && y == py) || (x == qx && y == qy))),
                )
            }
        } else if !same_row && !same_col {
            // not same box, not same row, not same col
            if box_id_p / 3 == box_id_q / 3 {
                // if same floor
                res.extend(Coord::intersect(House::Row(px), House::Box(box_id_q)));
                res.extend(Coord::intersect(House::Row(qx), House::Box(box_id_p)));
            } else if box_id_p % 3 == box_id_q % 3 {
                // if same tower
                res.extend(Coord::intersect(House::Col(py), House::Box(box_id_q)));
                res.extend(Coord::intersect(House::Col(qy), House::Box(box_id_p)));
            } else {
                res.push((px, qy));
                res.push((qx, py));
            }
        }
        Box::new(res.into_iter())
    }
}

#[cfg(test)]
mod test_coord {
    use super::*;
    #[test]

    fn test_pincer_same_row() {
        let res: Vec<_> = Coord::pinched_by(0, 2, 0, 4).collect();
        let exp = [(0, 0), (0, 1), (0, 3), (0, 5), (0, 6), (0, 7), (0, 8)];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }

    #[test]
    fn test_pincer_same_col() {
        let res: Vec<_> = Coord::pinched_by(4, 5, 8, 5).collect();
        let exp = [(0, 5), (1, 5), (2, 5), (3, 5), (5, 5), (6, 5), (7, 5)];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }
    #[test]

    fn test_pincer_same_row_box() {
        let res: Vec<_> = Coord::pinched_by(4, 4, 4, 5).collect();
        let exp = [
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 6),
            (4, 7),
            (4, 8),
            (3, 3),
            (3, 4),
            (3, 5),
            (5, 3),
            (5, 4),
            (5, 5),
        ];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }

    #[test]
    fn test_pincer_same_col_box() {
        let res: Vec<_> = Coord::pinched_by(4, 4, 5, 4).collect();
        let exp = [
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (6, 4),
            (7, 4),
            (8, 4),
            (3, 3),
            (4, 3),
            (5, 3),
            (3, 5),
            (4, 5),
            (5, 5),
        ];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }

    #[test]
    fn test_pincer_same_floor() {
        let res: Vec<_> = Coord::pinched_by(0, 0, 2, 8).collect();
        let exp = [(0, 6), (0, 7), (0, 8), (2, 0), (2, 1), (2, 2)];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }

    fn test_pincer_same_tower() {
        let res: Vec<_> = Coord::pinched_by(0, 0, 8, 2).collect();
        let exp = [(6, 0), (7, 0), (8, 0), (0, 2), (1, 2), (2, 2)];
        assert_eq!(res.len(), exp.len());
        for i in 0..res.len() {
            assert_eq!(res[i], exp[i])
        }
    }
}
