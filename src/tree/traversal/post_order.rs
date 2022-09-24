//! # 后序遍历
//! ## 题目
//! * 简单
//!     * [145. 二叉树的后序遍历](postorder_traversal)

use datastructure::TreeNode;

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

/// [2236. 判断根结点是否等于子结点之和](https://leetcode.cn/problems/root-equals-sum-of-children/)
pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if node.is_none() {
            return (0, true);
        }
        let inner = node.unwrap().clone();
        if inner.borrow().left.is_none() && inner.borrow().right.is_none(){
            return (inner.borrow().val, true);
        }
        
        let left = postorder(inner.borrow().left.clone());
        if !left.1 {
            return (0, false);
        }
        let right = postorder(inner.borrow().right.clone());
        if !right.1 {
            return (0, false);
        }

        return (left.0 + right.0, left.0 + right.0 == inner.borrow().val);
    }

    let (_, flag) = postorder(root);
    flag
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_check_tree(){
        struct Testcase{
            root: Option<Rc<RefCell<TreeNode>>>,
            expect: bool
        }
        
        vec![
            Testcase{
                root: tree!({val: 10, left: {val :4 }, right: { val: 6}}),
                expect: true
            },
            Testcase{
                root: tree!({val: 5, left: {val :3 }, right: { val: 1}}),
                expect: false
            },
        ].into_iter().enumerate().for_each(|(idx, testcase)|{
            let Testcase{root, expect} = testcase;
            let actual = check_tree(root);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_postorder_traversal() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: &'static [i32],
        }

        vec![
            Testcase {
                tree: tree!({1, right:{2, left: {3}}}),
                expect: &[3, 2, 1],
            },
            Testcase {
                tree: None,
                expect: &[],
            },
            Testcase {
                tree: tree!({ 1 }),
                expect: &[1],
            },
            Testcase {
                tree: tree!({1, left: {2}}),
                expect: &[2, 1],
            },
            Testcase {
                tree: tree!({1, right:{2}}),
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
