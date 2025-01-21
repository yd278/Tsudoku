use super::Coord;

#[derive(Debug,PartialEq)]
pub enum House {
    Row(usize),
    Col(usize),
    Box(usize),
}

impl House{
    pub fn to_iter(&self)-> Box<dyn Iterator<Item = (usize, usize)>> {
        Coord::house(&self)
    }
}