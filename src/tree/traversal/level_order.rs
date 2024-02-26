//! 层序遍历
use datastructure::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [2583. 二叉树中的第 K 大层和](https://leetcode.cn/problems/kth-largest-sum-in-a-binary-tree/)
pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    use std::collections::{BinaryHeap, VecDeque};

    let mut heap = BinaryHeap::new();
    let mut travel_queue = VecDeque::new();

    travel_queue.push_back(root);

    // let mut lvl = 0;
    while !travel_queue.is_empty() {
        let mut lvl_sum = 0i64;

        for _ in 0..travel_queue.len() {
            if let Some(node) = travel_queue.pop_front().unwrap() {
                let node = node.borrow();
                lvl_sum += node.val as i64;
                if let Some(left) = &node.left {
                    travel_queue.push_back(Some(left.clone()));
                }
                if let Some(right) = &node.right {
                    travel_queue.push_back(Some(right.clone()));
                }
            }
        }
        // println!("lvl: {}, lvl_sum: {}", lvl, lvl_sum);
        heap.push(lvl_sum);
        // lvl += 1;
    }
    // dbg!("heap", &heap);
    if heap.len() < k as usize {
        -1
    } else {
        heap.into_sorted_vec()
            .into_iter()
            .rev()
            .nth(k as usize - 1)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_kth_largest_level_sum() {
        struct TestCase {
            tree: Option<Rc<RefCell<TreeNode>>>,
            k: i32,
            expected: i64,
        }
        vec![
            TestCase{
                tree: tree!{val:5, left: {val: 8, left: {val: 2, left: {val: 4}, right: {val: 6}}, right: {val: 1}}, right: {val: 9, left: {val: 3}, right: {val: 7}}},
                k: 2,
                expected: 13,
            },
            TestCase{
                tree: tree!{val: 1, left: {val: 2, left: {val:3}}},
                k: 1,
                expected: 3
            }

        ].into_iter().enumerate().for_each(|(idx, TestCase{tree, k, expected})|{
            let actual = kth_largest_level_sum(tree.clone(), k);
            assert_eq!(actual, expected, "Test case {} failed", idx+1);
        });
    }
}
