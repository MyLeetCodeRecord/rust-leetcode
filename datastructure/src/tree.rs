
use std::cell::RefCell;
use std::fmt::{Display, Debug};
use std::rc::Rc;

#[derive(Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        if self.val != other.val {
            return false;
        }

        match (&self.left, &other.left) {
            (None, None) => {}
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (Some(l), Some(r)) => {
                if !l.borrow().eq(&r.borrow()) {
                    return false;
                }
            }
        }
        match (&self.right, &other.right) {
            (None, None) => {}
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (Some(l), Some(r)) => {
                if !l.borrow().eq(&r.borrow()) {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for TreeNode{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)
    }
}

impl TreeNode{
    fn display(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let mut ret = String::new();
        if depth > 0 {
            for _ in 0..depth-1 {
                ret.push_str("│  ");
            }
            ret.push_str("├─ ");
        }
        ret.push_str(&self.val.to_string());
        ret.push_str("\n");
        write!(f, "{}", ret)?;
        if let Some(left) = &self.left {
            left.borrow().display(f, depth+1)?;
        }
        if let Some(right) = &self.right {
            right.borrow().display(f, depth+1)?;
        }
        Ok(())
    }
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f, 0)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_tree_display(){
        let root = Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
        }));
        println!("{}", root.borrow());
    }
}