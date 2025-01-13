#[derive(Clone, Copy)]
pub struct BitMap(u16);

impl BitMap {
    pub fn all() -> Self{
        BitMap(0b111111111)
    }

    pub fn new() -> Self {
        BitMap(0)
    }

    pub fn from(num: usize) -> Self {
        BitMap(1 << num)
    }

    pub fn contains(self, num: usize) -> bool {
        self.0 & (1 << num) != 0
    }

    pub fn insert(&mut self, num: usize) {
        self.0 |= 1 << num;
    }

    pub fn remove(&mut self, num: usize) {
        self.0 &= !(1 << num);
    }

    pub fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn trailing_zeros(self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn complement(self) -> Self {
        BitMap(!self.0 & 0b111111111)
    }
    pub fn and(self, other: Self) -> Self {
        BitMap(self.0 & other.0)
    }
}