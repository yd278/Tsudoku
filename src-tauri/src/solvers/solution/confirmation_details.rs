use std::fmt;

pub struct ConfirmationDetails {
    pub x: usize,
    pub y: usize,
    pub target: usize,
}
impl fmt::Debug for ConfirmationDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cell ({}, {}) is confirmed to be {}",
            self.x + 1,
            self.y + 1,
            self.target + 1
        )
    }
}
