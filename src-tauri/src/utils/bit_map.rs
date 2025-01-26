#[derive(Clone, Copy, Debug, PartialEq)]
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

    fn next_combination(&self, limit: usize) -> Option<Self> {
        let raw = self.0;
        let u = raw & (!raw + 1);
        let v = raw + u;
        let next = (((raw ^ v) >> 2) / u) | v;
        (next > 1 << limit).then_some(Self(next))
    }
    
    fn get_combo_with_limit(n: usize, limit: usize) ->impl Iterator<Item = BitMap> {
        std::iter::successors(Some(BitMap::first_combination(n)), move |&prev| {
            prev.next_combination(limit)
        })
    }

    pub fn get_combinations(n: usize) -> impl Iterator<Item = BitMap> {
        Self::get_combo_with_limit(n, 9)
    }


    fn re_mapping(m: BitMap, c: BitMap) -> BitMap{
        m.iter_ones().enumerate().filter_map(|(i,b)|{
            c.contains(i).then_some(b)
        }).collect()
    }


    pub fn get_masked_combo(n: usize, mask: BitMap) -> Box<dyn Iterator<Item = BitMap>> {
        let limit = mask.count();
        if n> limit{
            Box::new(std::iter::empty())
        }else{
            Box::new(Self::get_combo_with_limit(n, limit).map(move |c| Self::re_mapping(mask, c)))
        }
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

    pub fn symmetric_difference(&self, other: &Self) -> Self {
        BitMap(self.0 ^ other.0)
    }
    pub fn iter_ones(&self) -> impl Iterator<Item = usize> + '_ {
        (0..9).filter(|x| self.contains(*x))
    }
    pub fn iter_zeros(&self) -> impl Iterator<Item = usize> + '_ {
        (0..9).filter(|x| !self.contains(*x))
    }
    pub fn subset_of_raw(&self, other: u16) -> bool {
        (self.0 & other) == self.0
    }
    pub fn difference(&self, other: &Self) -> Self {
        self.intersect(&other.complement())
    }
    #[cfg(test)]
    pub fn get_raw(&self) -> u16 {
        self.0
    }

    #[cfg(test)]
    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }
}

impl FromIterator<usize> for BitMap {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        iter.into_iter().fold(Self::new(), |mut acc, x| {
            acc.insert(x);
            acc
        })
    }
}

#[cfg(test)]
mod bit_map_test {

    use super::*;

}
