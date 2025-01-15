use crate::utils::BitMap;

#[derive(Clone, Copy)]
pub struct BlankCell {
    ans: usize,
    pen_mark: Option<usize>,
    candidates: BitMap,
    user_deleted: BitMap,
}
impl BlankCell {
    pub fn set_answer(&mut self, ans: usize) {
        self.ans = ans;
    }
    pub fn new(ans: usize) -> Self {
        Self {
            ans,
            pen_mark: None,
            candidates: BitMap::all(),
            user_deleted: BitMap::new(),
        }
    }

    pub fn new_empty_cell() -> Self {
        Self {
            ans: 0,
            pen_mark: None,
            candidates: BitMap::all(),
            user_deleted: BitMap::new(),
        }
    }

    pub fn get_candidates(&self) -> &BitMap {
        &self.candidates
    }

    pub fn set_candidates(&mut self, candidates: BitMap) {
        self.candidates = candidates;
    }

    pub fn get_user_deleted(&self) -> &BitMap {
        &self.user_deleted
    }
    // modify candidates and user_deleted with the given function
    pub fn modify<F: FnOnce(&mut BitMap, &mut BitMap)>(&mut self, mutator: F) {
        mutator(&mut self.candidates, &mut self.user_deleted);
    }

    pub fn get_answer(&self) -> usize {
        self.ans
    }

    pub fn check_collision(&self, target: usize) -> bool {
        match self.pen_mark {
            Some(mark) => target == mark,
            None => false,
        }
    }

    pub fn set_pencil_mark(&mut self, target: usize) {
        self.candidates.insert(target);
    }
    pub fn set_pen_mark(&mut self, mark: usize) {
        self.pen_mark = Some(mark);
    }
    pub fn get_pen_mark(&self) -> Option<usize> {
        self.pen_mark
    }
    pub fn erase_pen_mark(&mut self) {
        self.pen_mark = None;
    }
    pub fn update_candidates(&mut self, possible_candidates: &BitMap) {
        self.candidates = possible_candidates.intersect(&self.user_deleted.complement());
    }
    pub fn is_pen_mark(&self) -> bool {
        self.pen_mark.is_some()
    }

    pub fn contains_candidate(&self, target: usize) -> bool {
        self.candidates.contains(target)
    }
}
