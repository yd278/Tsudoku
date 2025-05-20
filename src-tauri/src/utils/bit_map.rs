use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct BitMap(u16);

impl Default for BitMap {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Debug for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        write!(f, "{{")?;
        for i in 0..9 {
            if (self.0 & (1 << i)) != 0 {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", i + 1)?;
                first = false;
            }
        }
        write!(f, "}}")
    }
}
impl BitMap {
    //This is a special value and should not used in set operations.
    pub const NAN: Self = Self(0xFFFF);
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
        (next < 1 << limit).then_some(Self(next))
    }

    fn get_combo_with_limit(n: usize, limit: usize) -> impl Iterator<Item = BitMap> {
        std::iter::successors(Some(BitMap::first_combination(n)), move |&prev| {
            prev.next_combination(limit)
        })
    }

    pub fn get_combinations(n: usize) -> impl Iterator<Item = BitMap> {
        Self::get_combo_with_limit(n, 9)
    }

    fn re_mapping(m: BitMap, c: BitMap) -> BitMap {
        m.iter_ones()
            .enumerate()
            .filter_map(|(i, b)| c.contains(i).then_some(b))
            .collect()
    }

    pub fn get_masked_combo(n: usize, mask: BitMap) -> Box<dyn Iterator<Item = BitMap>> {
        Self::get_combos_in_subset(n, mask.complement())
    }

    pub fn get_combos_in_subset(n: usize, subset: BitMap) -> Box<dyn Iterator<Item = BitMap>> {
        let limit = subset.count();
        if n > limit {
            Box::new(std::iter::empty())
        } else {
            Box::new(Self::get_combo_with_limit(n, limit).map(move |c| Self::re_mapping(subset, c)))
        }
    }

    pub fn contains(&self, num: usize) -> bool {
        self.0 & (1 << num) != 0
    }

    pub fn insert(&mut self, num: usize) {
        self.0 |= 1 << num;
    }

    pub fn insert_set(&mut self, other: Self) {
        self.0 |= other.0
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
    pub fn intersect(&self, other: Self) -> Self {
        BitMap(self.0 & other.0)
    }
    pub fn union(&self, other: Self) -> Self {
        BitMap(self.0 | other.0)
    }

    pub fn symmetric_difference(&self, other: Self) -> Self {
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
    pub fn difference(&self, other: Self) -> Self {
        self.intersect(other.complement())
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
    #[test]
    fn test_combo() {
        let exp = [
            15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86,
            89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147,
            149, 150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197,
            198, 201, 202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240, 263, 267, 269, 270,
            275, 277, 278, 281, 282, 284, 291, 293, 294, 297, 298, 300, 305, 306, 308, 312, 323,
            325, 326, 329, 330, 332, 337, 338, 340, 344, 353, 354, 356, 360, 368, 387, 389, 390,
            393, 394, 396, 401, 402, 404, 408, 417, 418, 420, 424, 432, 449, 450, 452, 456, 464,
            480,
        ];
        for (index, combo) in BitMap::get_combinations(4).enumerate() {
            assert_eq!(combo.get_raw(), exp[index])
        }

        assert_eq!(BitMap::get_combinations(4).count(), 126);
    }

    #[test]
    fn test_combo_mask() {
        let exp = [
            11, 19, 25, 26, 67, 73, 74, 81, 82, 88, 131, 137, 138, 145, 146, 152, 193, 194, 200,
            208,
        ];

        assert_eq!(
            BitMap::get_masked_combo(3, BitMap(0b11011011).complement()).count(),
            20
        );
        for (index, combo) in
            BitMap::get_masked_combo(3, BitMap(0b11011011).complement()).enumerate()
        {
            assert_eq!(combo.get_raw(), exp[index])
        }
    }
}
