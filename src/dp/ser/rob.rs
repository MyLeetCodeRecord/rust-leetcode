//! 打家劫舍系列
//!
//! * 中等
//!     * [198. 打家劫舍](rob_1)
//!     * [213. 打家劫舍 II](rob_2)
//!     * [337. 打家劫舍 III](rob_3)
//!     * [2560. 打家劫舍 IV](crate::array::ser::binary_search::min_capability)
//!

use datastructure::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [198. 打家劫舍](https://leetcode.cn/problems/house-robber/description)
///
/// 记 `f(k)` 为前 `k` 个的最终结果
/// 可以得到递推关系: `f(k) = max(f(k-1), H_k + f(k-2)) k >=2`
/// 对于边界, `f(0) = 0, f(1) = H_0`
pub fn rob_1(nums: Vec<i32>) -> i32 {
    rob(&nums)
}

fn rob(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let (mut a, mut b) = (0, nums[0]);
    for k in 1..nums.len() {
        let (k_2, k_1) = (a, b);
        a = k_1;
        b = (nums[k] + k_2).max(k_1);
    }

    return b;
}

/// [213. 打家劫舍 II](https://leetcode.cn/problems/house-robber-ii)
///
/// 相对于[198. 打家劫舍](rob_1), 变动是将首位相连
/// 取第一个, 则结尾不能包含,
/// 取最后一个, 则结尾不能包含
///
/// 因此, 可以将问题转化为两个[198. 打家劫舍](rob_1)的问题
/// 1. 包含第一个, 舍弃最后一个: 等效的是对 `nums[0..=n-2]` 求解
/// 2. 包含最后一个, 舍弃第一个: 等效的是对 `nums[1..=n-1]` 求解
///
/// 两个问题的最大值即为所求
pub fn rob_2(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    } else if nums.len() <= 3 {
        return nums.iter().max().copied().unwrap();
    }
    let n = nums.len();
    let one = rob(&nums[0..=n - 2]);
    let two = rob(&nums[1..=n - 1]);

    one.max(two)
}

/// [337. 打家劫舍 III](https://leetcode.cn/problems/house-robber-iii/)
///
/// 和[213. 打家劫舍 II](rob_2)类似, 分类讨论即可
/// 1. 包含当前root节点的最大值
///     - 进而递归时, 要求左右子树的不包含当前root节点的最大值
/// 2. 不包含当前root节点的最大值
///     - 进而递归时, 要求左右子树的最大值
///     - 可以包含左右子树的根
///
/// 为防止重复计算, 一次返回两种情况的值
pub fn rob_3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let mut need = root.as_ref().unwrap().borrow().val;
        let mut needless = 0;

        let (l_need, l_needless) = dfs(root.as_ref().unwrap().borrow().left.clone());
        let (r_need, r_needless) = dfs(root.as_ref().unwrap().borrow().right.clone());

        need += l_needless + r_needless;
        needless += l_need.max(l_needless) + r_need.max(r_needless);

        (need, needless)
    }

    let (need, needless) = dfs(root);
    need.max(needless)
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros::tree;

    #[test]
    fn test_rob_3() {
        struct TestCase {
            root: Option<Rc<RefCell<TreeNode>>>,
            expect: i32,
        }

        vec![
            TestCase{
                root: tree!(val:3, left: {val: 2, right: {val: 3}}, right: {val: 3, right: {val: 1}}),
                expect: 7
            },
            TestCase{
                root: tree!(val: 3, left: {val: 4, left: {val: 1}, right: {val: 3}}, right: {val: 5, right: {val: 1}}),
                expect: 9
            },
        ].into_iter().enumerate().for_each(|(idx, TestCase{root, expect})|{
            let actual = rob_3(root);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_rob_2() {
        struct TestCase {
            nums: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                nums: vec![2, 3, 2],
                expect: 3,
            },
            TestCase {
                nums: vec![1, 2, 3, 1],
                expect: 4,
            },
            TestCase {
                nums: vec![1, 2, 3],
                expect: 3,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { nums, expect })| {
            let actual = rob_2(nums);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_rob_1() {
        struct TestCase {
            nums: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                nums: vec![1, 2, 3, 1],
                expect: 4,
            },
            TestCase {
                nums: vec![2, 7, 9, 3, 1],
                expect: 12,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { nums, expect })| {
            let actual = rob_1(nums);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
