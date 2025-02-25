use super::{Coord, HouseType};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum House {
    Row(usize),
    Col(usize),
    Box(usize),
}

impl House {
    pub fn as_iter(&self) -> Box<dyn Iterator<Item = (usize, usize)>> {
        Coord::house(self)
    }

    pub fn from_dim_id(dim: usize, id: usize) -> Self {
        match dim {
            0 => Self::Row(id),
            1 => Self::Col(id),
            2 => Self::Box(id),
            _ => panic!(),
        }
    }

    pub fn ith_cell(&self, index: usize) -> (usize, usize) {
        match self {
            House::Row(x) => (*x, index),
            House::Col(y) => (index, *y),
            House::Box(b) => {
                let x_offset = b / 3 * 3;
                let y_offset = b % 3 * 3;
                (x_offset + index / 3, y_offset + index % 3)
            }
        }
    }

    pub fn get_dim(&self) -> usize {
        match self {
            House::Row(_) => 0,
            House::Col(_) => 1,
            House::Box(_) => 2,
        }
    }

    pub fn get_type(&self) -> HouseType {
        match self {
            House::Row(_) => HouseType::Row,
            House::Col(_) => HouseType::Col,
            House::Box(_) => HouseType::Box,
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            House::Row(x) => *x,
            House::Col(x) => *x,
            House::Box(x) => *x,
        }
    }
    pub fn get_parallel(&self, other: usize) -> Self {
        match self {
            House::Row(x) if *x != other => Self::Row(other),
            House::Col(x) if *x != other => Self::Col(other),
            House::Box(x) if *x != other => Self::Box(other),
            _ => panic!(),
        }
    }
    pub fn get_perpendicular(&self, other: usize) -> Self {
        match self {
            House::Row(_) => House::Col(other),
            House::Col(_) => House::Row(other),
            House::Box(_) => panic!(),
        }
    }
}
