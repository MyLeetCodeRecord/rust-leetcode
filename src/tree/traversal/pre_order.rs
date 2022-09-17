use crate::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

/// [144. 二叉树的前序遍历](https://leetcode.cn/problems/binary-tree-preorder-traversal/)
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];

    fn preorder(store: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let inner = node.unwrap().clone();
        store.push(inner.borrow().val);
        preorder(store, inner.borrow().left.clone());
        preorder(store, inner.borrow().right.clone());
    }

    preorder(&mut ret, root);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_preorder_traversal() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: &'static [i32],
        }

        vec![
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({1, right:{2, left: {3}}})))),
                expect: &[1, 2, 3],
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
                expect: &[1, 2],
            },
            Testcase {
                tree: Some(Rc::new(RefCell::new(tree!({1, right:{2}})))),
                expect: &[1, 2],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { tree, expect } = testcase;
            let acutal = preorder_traversal(tree);
            assert_eq!(expect, acutal, "case {} failed", idx);
        })
    }
}
