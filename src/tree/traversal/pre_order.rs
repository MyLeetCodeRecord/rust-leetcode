//! # 先序遍历
//! ## 题目
//! * 简单
//!     * [144. 二叉树的前序遍历](preorder_traversal)
//! * 中等
//!     * [437. Path Sum III](path_sum_3)

use datastructure::TreeNode;

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

/// [437. Path Sum III](https://leetcode.cn/problems/path-sum-iii/)
pub fn path_sum_3(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    fn count(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let root_val = root.borrow().val as i64;

        let mut cnt = 0;
        if root_val == target_sum {
            cnt = cnt + 1;
        }
        cnt = cnt + count(root.borrow().left.clone(), target_sum - root_val);
        cnt = cnt + count(root.borrow().right.clone(), target_sum - root_val);
        cnt
    }

    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32{
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let root_val = root.borrow().val as i64;

        let mut cnt = 0;
        if root_val == target_sum {
            cnt = cnt + 1;
        }

        // 包含 root 的
        cnt = cnt + count(root.borrow().left.clone(), target_sum - root_val);
        cnt = cnt + count(root.borrow().right.clone(), target_sum - root_val);
        cnt = cnt + path_sum(root.borrow().left.clone(), target_sum);
        cnt = cnt + path_sum(root.borrow().right.clone(), target_sum);
        cnt
    }

    path_sum(root, target_sum as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_path_sum_3() {
        struct TestCase {
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            expect: i32,
        }

        vec![
            TestCase{
                root: tree!({
                    val: 10,
                    left: {
                        val: 5, 
                        left: {
                            val: 3, 
                            left: {3},
                            right: {-2}
                        },
                        right: {
                            val: 2, 
                            right: {1}
                        }
                    },
                    right: {
                        -3,
                        right: {
                            11
                        }
                    }
                }),
                target_sum: 8,
                expect: 3,
            },
            TestCase{
                root: tree!({val: 5, left: {val: 4, left: {val: 11, left: {val: 7}, right: {val: 2}}}, right: {val: 8, left: {val: 13}, right: {val: 4, left: {val: 5}, right: {val: 1}}}}),
                target_sum: 22,
                expect: 3,
            },
        ].into_iter().enumerate().for_each(|(idx, TestCase{root, target_sum, expect})|{
            // println!("{:?}", &root);
            let ret = path_sum_3(root, target_sum);
            assert_eq!(ret, expect, "test case {} failed", idx);
        });
    }

    #[test]
    fn test_preorder_traversal() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            expect: &'static [i32],
        }

        vec![
            Testcase {
                tree: tree!({1, right:{2, left: {3}}}),
                expect: &[1, 2, 3],
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
                expect: &[1, 2],
            },
            Testcase {
                tree: tree!({1, right:{2}}),
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
