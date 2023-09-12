//! dfs相关题目

use datastructure::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [863. 二叉树中所有距离为 K 的结点](https://leetcode.cn/problems/all-nodes-distance-k-in-binary-tree/)
///
/// - 利用栈, 找到target, 栈里都是父节点的父节点,
/// - target父节点到其他节点距离需要为k-1, 依次往上, 然后递归
/// - Node.val 中所有值不同, 所以可以通过记录值, 标示访问过的点, 去重
pub fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    use std::collections::HashSet;
    let mut res = vec![];

    let mut stack = vec![];
    let mut visited = HashSet::new();
    let target_val = target.unwrap().borrow().val;
    stack.push(root.unwrap().clone());
    while let Some(node) = stack.pop() {
        if node.borrow().val == target_val {
            // 找到target, 此时stack中都是父节点的父节点
            // 栈顶是target
            stack.push(node);
            break;
        }
        // 如果没找到, 优先找左节点, 再找右节点
        // 同时为了防止重复, 需要检查是否已经访问过
        if let Some(left) = node.borrow().left.clone() {
            if !visited.contains(&left.borrow().val) {
                // 在深入一层之前, 先把当前节点放入栈中
                stack.push(node.clone());
                stack.push(left);
                continue;
            }
        }
        if let Some(right) = node.borrow().right.clone() {
            if !visited.contains(&right.borrow().val) {
                stack.push(node.clone());
                stack.push(right);
                continue;
            }
        }
        // 没有子节点, 或者子节点已经访问过, 那么就pop
        // 同时标记为已访问
        // 此时不再压栈
        visited.insert(node.borrow().val);
    }

    fn dfs(
        ret: &mut Vec<i32>,
        visited: &mut HashSet<i32>,
        node: Option<Rc<RefCell<TreeNode>>>,
        lvl: usize,
    ) {
        if lvl == 0 {
            ret.push(node.unwrap().borrow().val);
            return;
        }
        if let Some(left) = node.clone().unwrap().borrow().left.clone() {
            if !visited.contains(&left.borrow().val) {
                dfs(ret, visited, Some(left), lvl - 1);
            }
        }
        if let Some(right) = node.clone().unwrap().borrow().right.clone() {
            if !visited.contains(&right.borrow().val) {
                dfs(ret, visited, Some(right), lvl - 1);
            }
        }
        visited.insert(node.unwrap().borrow().val);
    }

    // 重置visited
    visited.clear();
    let k = k as usize;
    // 从栈顶依次往上, 寻找k-i的节点
    for (i, node) in stack.into_iter().rev().enumerate() {
        if i == k {
            res.push(node.borrow().val);
            break;
        }

        // 以node为root, 寻找其子树中距离为k-i的节点
        // 由于node是从栈中pop出来的, 所以不会重复访问
        // 为防止子树重复, 需要记录已访问的节点
        dfs(&mut res, &mut visited, Some(node), k - i);
    }

    res
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use datastructure::TreeNode;

    #[test]
    fn test_distance_k() {
        struct TestCase {
            t: Option<Rc<RefCell<TreeNode>>>,
            target: Option<Rc<RefCell<TreeNode>>>,
            k: i32,
            expect: Vec<i32>,
        }

        vec![
            TestCase{
                t: macros::tree!({val: 3, left: {5, left: {6}, right: {2, left: {7}, right: {4}}}, right: {1, left: {0}, right: {8}}}),
                target: macros::tree!({val: 5}),
                k: 2,
                expect: vec![7, 4, 1],
            },
            TestCase{
                t: macros::tree!(val: 1),
                target: macros::tree!(val: 1),
                k: 3,
                expect: vec![],
            },
        ].into_iter().enumerate().for_each(|(idx, TestCase { t, target, k, expect })| {
            assert_eq!(distance_k(t, target, k), expect, "case {} failed", idx);
        });
    }
}
