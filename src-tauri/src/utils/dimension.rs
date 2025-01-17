use super::House;
#[derive(Clone)]
pub enum Dimension {
    Row,
    Col,
}

impl Dimension {
    pub fn other(&self) -> Dimension {
        match self {
            Dimension::Row => Dimension::Col,
            Dimension::Col => Dimension::Row,
        }
    }
    pub fn house(&self, x: usize) -> House {
        match self {
            Dimension::Row => House::Row(x),
            Dimension::Col => House::Col(x),
        }
    }
}