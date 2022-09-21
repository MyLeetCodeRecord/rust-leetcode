//! # 后序遍历
//! ## 题目
//! * 简单
//!     * [145. 二叉树的后序遍历](postorder_traversal)

use crate::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

/// [145. 二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/)
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];

    fn postorder(store: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let inner = node.unwrap().clone();
        postorder(store, inner.borrow().left.clone());
        postorder(store, inner.borrow().right.clone());
        store.push(inner.borrow().val);
    }

    postorder(&mut ret, root);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_postorder_traversal() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: &'static [i32],
        }

        vec![
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({1, right:{2, left: {3}}})))),
                expect: &[3, 2, 1],
            },
            Testcase {
                tree: None,
                expect: &[],
            },
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({ 1 })))),
                expect: &[1],
            },
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({1, left: {2}})))),
                expect: &[2, 1],
            },
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({1, right:{2}})))),
                expect: &[2, 1],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { tree, expect } = testcase;
            let acutal = postorder_traversal(tree);
            assert_eq!(expect, acutal, "case {} failed", idx);
        })
    }
}
