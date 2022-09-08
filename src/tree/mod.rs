use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let t = macros::tree!({val: 1, left: {2, right: {3}}});
        dbg!(t);
    }
}

pub mod traversal;
