use crate::utils::BitMap;
pub struct BlankCell{
    ans: u8,
    pen_mark: Option<u8>,
    candidates: BitMap,
    user_deleted: BitMap,
}
impl BlankCell{
    pub fn new(ans: u8) -> Self{
        Self{
            ans,
            pen_mark: None,
            candidates: BitMap::all(),
            user_deleted: BitMap::new(),
        }
    }

    pub fn get_candidates(&self) -> &BitMap {
        &self.candidates
    }

    // modify candidates and user_deleted with the given function
    pub fn modify<F: FnOnce(&mut BitMap, &mut BitMap)>(&mut self, mutator: F) {
        mutator(&mut self.candidates, &mut self.user_deleted);
    }


    pub fn get_answer(&self) -> u8{
        self.ans
    }

    pub fn check_collision(&self, target: u8) -> bool {
        match self.pen_mark {
            Some(mark) => target == mark,
            None => false
        }
    }

    pub fn set_pen_mark(&mut self, mark: u8) {
        self.pen_mark = Some(mark);
    }
    pub fn get_pen_mark(&self) -> Option<u8> {
        self.pen_mark
    }
    pub fn erase_pen_mark(&mut self) {
        self.pen_mark = None;
    }

    pub fn update_or_collide(&mut self, target: u8, possible_candidates: &mut BitMap) {
        if let Some(mark) = self.pen_mark{
            possible_candidates.remove(mark);
        }else{
            if !self.user_deleted.contains(target){
                self.candidates.insert(target);
            }
        }

    }
        
}