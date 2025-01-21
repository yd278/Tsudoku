use super::House;
#[derive(Clone)]
pub enum HouseType {
    Row,
    Col,
    Box,
}


impl HouseType {
    pub fn other(&self) -> HouseType {
        match self {
            HouseType::Row => HouseType::Col,
            HouseType::Col => HouseType::Row,
            HouseType::Box => panic!(),
        }
    }
    pub fn house(&self, x: usize) -> House {
        match self {
            HouseType::Row => House::Row(x),
            HouseType::Col => House::Col(x),
            HouseType::Box => House::Box(x),
        }
    }
    pub fn as_index(&self) -> usize{
        match self {
            HouseType::Row => 0,
            HouseType::Col => 1,
            HouseType::Box => 2,
        }

    }
    pub fn from_index(i : usize) ->Self{
        match i{
            0=> Self::Row,
            1=> Self::Col,
            2=> Self::Box,
            _=> panic!()
        }
    }

}