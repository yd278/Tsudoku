use std::rc::Rc;
use std::cell::RefCell;

pub type Link = Option<Rc<RefCell<DLXNode>>>;

#[derive(Debug,PartialEq)]
pub struct DLXNode {
    pub left: Link,
    pub right: Link,
    pub up: Link,
    pub down: Link,
    pub column: Link, // 列头节点
    pub row_id: usize, 
    pub col_id: usize, 
}

impl DLXNode {
    pub fn new(row_id: usize, col_id: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DLXNode {
            left: None,
            right: None,
            up: None,
            down: None,
            column: None,
            row_id,
            col_id,
        }))
    }
    
}
