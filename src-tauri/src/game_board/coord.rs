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
}
