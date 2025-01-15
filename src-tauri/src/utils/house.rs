#[derive(Debug)]
pub enum House {
    Row(usize),
    Col(usize),
    Box(usize),
}
