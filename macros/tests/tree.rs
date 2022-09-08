use std::cell::RefCell;
use std::rc::Rc;

use macros::tree;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    let t = tree!({1, right:{2, left: 3}});
    dbg!(t);
}
