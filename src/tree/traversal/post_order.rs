//! # 后序遍历
//! ## 题目
//! * 简单
//!     * [145. 二叉树的后序遍历](postorder_traversal)
//!     * [2236. 判断根结点是否等于子结点之和](check_tree)
//! * 中等
//!    * [865. 具有所有最深节点的最小子树](subtree_with_all_deepest)

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
        if inner.borrow().left.is_none() && inner.borrow().right.is_none() {
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

/// [865. 具有所有最深节点的最小子树](https://leetcode.cn/problems/smallest-subtree-with-all-the-deepest-nodes/)
///
/// 相同题目 [1123. Lowest Common Ancestor of Deepest Leaves](https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/)
pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if root.is_none() {
            return (None, 0);
        }
        let root = root.unwrap();
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return (Some(root), depth);
        }
        let (left, left_depth) = postorder(root.borrow().left.clone(), depth + 1);
        let (right, right_depth) = postorder(root.borrow().right.clone(), depth + 1);
        if left_depth == right_depth {
            (Some(root), left_depth)
        } else if left_depth > right_depth {
            return (left, left_depth);
        } else {
            return (right, right_depth);
        }
    }

    let (ret, _) = postorder(root, 0);
    ret
}


#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_subtree_with_all_deepest() {
        struct TestCase {
            t: Option<Rc<RefCell<TreeNode>>>,
            expect: Option<Rc<RefCell<TreeNode>>>,
        }

        vec![
            TestCase{
                t: macros::tree!({val: 3, left: {5, left: {6}, right: {2, left: {7}, right: {4}}}, right: {1, left: {0}, right: {8}}}),
                expect: macros::tree!({val: 2, left: {7}, right: {4}}),
            },
            TestCase{
                t: macros::tree!({val: 1}),
                expect: macros::tree!({val: 1}),
            },
            TestCase{
                t: macros::tree!({val: 0, left: {1, right: {2}}, right: {3}}),
                expect: macros::tree!({val: 2}),
            },
        ].into_iter().enumerate().for_each(|(idx, TestCase{t, expect})|{
            let ret = subtree_with_all_deepest(t);
            assert_eq!(ret, expect, "test case {} failed", idx);
        });
    }

    #[test]
    fn test_check_tree() {
        struct Testcase {
            root: Option<Rc<RefCell<TreeNode>>>,
            expect: bool,
        }

        vec![
            Testcase {
                root: tree!({val: 10, left: {val :4 }, right: { val: 6}}),
                expect: true,
            },
            Testcase {
                root: tree!({val: 5, left: {val :3 }, right: { val: 1}}),
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { root, expect } = testcase;
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
