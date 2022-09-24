//! # 中序遍历
//! ## 题目
//! * 简单
//!     * [94. 二叉树的中序遍历](inorder_traversal)
//! * 中等
//!     * [98. 验证二叉搜索树](is_valid_bst)
//!     * [98. 验证二叉搜索树](is_valid_bst_2)
//! * 困难

use datastructure::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

/// [94. 二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/)
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];

    fn inorder(store: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let inner = node.unwrap().clone();

        inorder(store, inner.borrow().left.clone());
        store.push(inner.borrow().val);
        inorder(store, inner.borrow().right.clone());
    }

    inorder(&mut ret, root);
    ret
}

/// [98. 验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/)
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut store = vec![];

    fn inorder(store: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let inner = node.unwrap().clone();

        inorder(store, inner.borrow().left.clone());
        store.push(inner.borrow().val);
        inorder(store, inner.borrow().right.clone());
    }

    inorder(&mut store, root);

    store.windows(2).all(|w| &w[0] < &w[1])
}

/// [98. 验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/)
pub fn is_valid_bst_2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32, bool) {
        if node.is_none() {
            // (是否是bst， 子树最小值， 子树最大值, 自身是否为none)
            return (true, i32::MAX, i32::MIN, true);
        }
        let inner = node.unwrap().clone();
        let v = inner.borrow().val;

        let (l, lmin, lmax, lnone) = inorder(inner.borrow().left.clone());
        if !l {
            return (false, 0, 0, false);
        } else if !lnone && v <= lmax {
            return (false, 0, 0, false);
        }

        let (r, rmin, rmax, rnone) = inorder(inner.borrow().right.clone());
        if !r {
            return (false, 0, 0, false);
        } else if !rnone && v >= rmin {
            return (false, 0, 0, false);
        }

        (true, std::cmp::min(lmin, v), std::cmp::max(v, rmax), false)
    }

    let (flag, _, _, _) = inorder(root);
    flag
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_is_valid_bst_2() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: bool,
        }

        vec![
            Testcase {
                tree: tree!({2, left: {1}, right:{3}}),
                expect: true,
            },
            Testcase {
                tree: 
                    tree!({5, left: {1}, right:{4, left: {3}, right:{6} }}),
                
                expect: false,
            },
            Testcase {
                tree: tree!({ 2147483647 }),
                expect: true,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { tree, expect } = testcase;
            let acutal = is_valid_bst_2(tree);
            assert_eq!(expect, acutal, "case {} failed", idx + 1);
        });
    }

    #[test]
    fn test_is_valid_bst() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: bool,
        }

        vec![
            Testcase {
                tree: tree!({2, left: {1}, right:{3}}),
                expect: true,
            },
            Testcase {
                tree: 
                    tree!({5, left: {1}, right:{4, left: {3}, right:{6} }}),
                
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { tree, expect } = testcase;
            let acutal = is_valid_bst(tree);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }

    #[test]
    fn test_inorder_traversal() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect:Vec<i32>,
        }

        vec![
            Testcase {
                tree: tree!({1, right:{2, left: {3}}}),
                expect: vec![1, 3, 2],
            },
            Testcase {
                tree: None,
                expect: vec![],
            },
            Testcase {
                tree: tree!({ 1 }),
                expect: vec![1],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase {tree, expect } = testcase;
            let acutal = inorder_traversal(tree);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }
}
