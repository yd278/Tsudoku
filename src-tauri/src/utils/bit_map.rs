#[derive(Clone, Copy,Debug)]
pub struct BitMap(u16);

impl BitMap {
    pub fn all() -> Self {
        BitMap(0b111111111)
    }

    pub fn new() -> Self {
        BitMap(0)
    }

    pub fn from(num: usize) -> Self {
        BitMap(1 << num)
    }

    fn first_combination(size: usize) -> Self {
        BitMap((1 << size) - 1)
    }

    fn next_combination_rec(num: u16, n: usize, k: usize) -> Option<u16> {
        if k == 0 {
            return None;
        }
        let cur = (num & (1 << (n - 1))) != 0;
        let mask = (1 << (n - 1)) - 1;
        let rest = num & mask;
        if cur {
            Self::next_combination_rec(rest, n - 1, k - 1).map(|res| (1 << (n - 1)) | res)
        } else if let Some(res) = Self::next_combination_rec(rest, n - 1, k) {
            Some(res)
        } else {
            Some(1 << (n - 1) | ((1 << (k - 1)) - 1))
        }
    }

    fn next_combination(&self) -> Option<Self> {
        let k = self.0.count_ones() as usize;
        Self::next_combination_rec(self.0, 9, k).map(BitMap)
    }

    pub fn get_combinations(n: usize) -> impl Iterator<Item = BitMap> {
        std::iter::successors(Some(BitMap::first_combination(n)), |&prev| {
            prev.next_combination()
        })
    }

    pub fn contains(&self, num: usize) -> bool {
        self.0 & (1 << num) != 0
    }

    pub fn insert(&mut self, num: usize) {
        self.0 |= 1 << num;
    }

    pub fn remove(&mut self, num: usize) {
        self.0 &= !(1 << num);
    }

    pub fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn trailing_zeros(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn complement(&self) -> Self {
        BitMap(!self.0 & 0b111111111)
    }
    pub fn intersect(&self, other: &Self) -> Self {
        BitMap(self.0 & other.0)
    }
    pub fn union(&self, other: &Self) -> Self {
        BitMap(self.0 | other.0)
    }
    #[cfg(test)]
    pub fn get_raw(&self) -> u16 {
        self.0
    }

    #[cfg(test)]
    pub fn from_raw(raw: u16) -> Self{
        Self(raw)
    }
}

#[cfg(test)]
mod bit_map_test {

    use super::*;

    #[test]
    fn test_combos_count() {
        for i in 0..=9 {
            let iter = std::iter::successors(Some(BitMap::first_combination(i)), |&prev| {
                prev.next_combination()
            });

            let count = iter.count();
            let mut res = 1;
            for j in 0..i {
                res *= 9 - j;
            }
            for j in 1..=i {
                res /= j;
            }
            assert_eq!(count, res);
        }
    }
}
