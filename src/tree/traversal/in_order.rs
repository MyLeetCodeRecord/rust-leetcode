//! # 中序遍历
//! ## 题目
//! * 简单
//!     * [94. 二叉树的中序遍历](inorder_traversal)
//! * 中等
//!     * [98. 验证二叉搜索树](is_valid_bst)
//!     * [98. 验证二叉搜索树](is_valid_bst_2)
//!     * [2476. 二叉搜索树最近节点查询](closest_nodes)
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
        }
        if !lnone && v <= lmax {
            return (false, 0, 0, false);
        }

        let (r, rmin, rmax, rnone) = inorder(inner.borrow().right.clone());
        if !r {
            return (false, 0, 0, false);
        }
        if !rnone && v >= rmin {
            return (false, 0, 0, false);
        }

        (true, std::cmp::min(lmin, v), std::cmp::max(v, rmax), false)
    }

    let (flag, _, _, _) = inorder(root);
    flag
}

/// [2476. 二叉搜索树最近节点查询](https://leetcode.cn/problems/closest-nodes-queries-in-a-binary-search-tree/)
///
/// ## 思路2: 利用二茬搜索树的性质, 递归. 实际不可行, 因为题目没有保证树的平衡性, 会超时
/// ## 思路1: 展开成有序数组, 二分查找
pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    fn inorder(store: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
        if node.is_none() {
            return;
        }
        let inner = node.unwrap().clone();

        inorder(store, inner.borrow().left.clone());
        store.push(inner.borrow().val);
        inorder(store, inner.borrow().right.clone());
    }
    let mut store = vec![];
    inorder(&mut store, root);

    let mut result = vec![];
    for q in queries {
        match store.binary_search(&q) {
            Ok(_) => {
                result.push(vec![q, q]);
                continue;
            }
            Err(idx) => {
                if idx == 0 {
                    result.push(vec![-1, store[0]]);
                } else if idx == store.len() {
                    result.push(vec![store[store.len() - 1], -1]);
                } else {
                    result.push(vec![store[idx - 1], store[idx]]);
                }
            }
        }
    }
    result
}

/// [235. 二叉搜索树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/description/)
///
/// ## 思路
/// 由于是二叉搜索树, 因此只需要找到一个能将两个值左右分割的点即可, 不用存储父节点的栈
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap().borrow().val;
    let q = q.unwrap().borrow().val;

    let mut node = root;
    while let Some(inner) = node {
        let v = inner.borrow().val;
        if p < v && q < v {
            node = inner.borrow().left.clone();
        } else if p > v && q > v {
            node = inner.borrow().right.clone();
        } else {
            return Some(inner);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use crate::vec2;

    use super::*;
    use macros::tree;

    #[test]
    fn test_lowest_common_ancestor() {
        struct Testcase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
            expect: Option<Rc<RefCell<TreeNode>>>,
        }

        vec![
            Testcase {
                tree: tree!({6, left: {2, left: {0}, right: {4, left: {3}, right: {5}}}, right: {8, left: {7}, right: {9}}}),
                p: tree!({2}),
                q: tree!({8}),
                expect: tree!({6}),
            },
            Testcase {
                tree: tree!({6, left: {2, left: {0}, right: {4, left: {3}, right: {5}}}, right: {8, left: {7}, right: {9}}}),
                p: tree!({2}),
                q: tree!({4}),
                expect: tree!({2}),
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase {
                tree,
                p,
                q,
                expect,
            } = testcase;
            let acutal = lowest_common_ancestor(tree, p, q);
            let actual_val = acutal.map(|x| x.borrow().val).unwrap();
            let expect_val = expect.map(|x| x.borrow().val).unwrap();
            assert_eq!(expect_val, actual_val, "case {} failed", idx);
        });
    }

    #[test]
    fn test_closest_nodes() {
        struct TestCase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            queries: Vec<i32>,
            expected: Vec<Vec<i32>>,
        }

        vec![
            TestCase{
                tree: tree!{val:6, left:{val:2, left: {val:1}, right: {val:4}}, right: {val:13, left: {val:9}, right: {val: 15, left: {val: 14}}}},
                queries: vec![2, 5, 16],
                expected: vec![vec![2, 2], vec![4, 6], vec![15, -1]],
            },
            TestCase{
                tree: tree!{val: 4, right: {val: 9}},
                queries: vec![3],
                expected: vec![vec![-1, 4]],
            },
            TestCase{
                tree: tree!{val: 16, left: {val: 8, left: {val: 1, right: {val: 2, right: {val: 7}}}, right: {val:12, left: {val: 9}}}, right: {val: 18, right: {val: 20}}},
                queries: vec![8,14,285508,6],
                expected: vec2![[8,8],[12,16],[20,-1],[2,7]],
            }
        ].into_iter().enumerate().for_each(|(idx, TestCase{tree, queries, expected})|{
            let result = closest_nodes(tree, queries);
            assert_eq!(result, expected, "index: {}", idx);
        })
    }

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
                tree: tree!({5, left: {1}, right:{4, left: {3}, right:{6} }}),

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
                tree: tree!({5, left: {1}, right:{4, left: {3}, right:{6} }}),

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
            expect: Vec<i32>,
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
            let Testcase { tree, expect } = testcase;
            let acutal = inorder_traversal(tree);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }
}
