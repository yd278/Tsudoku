#[derive(Debug,PartialEq)]
pub enum House {
    Row(usize),
    Col(usize),
    Box(usize),
}
