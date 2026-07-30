//! 组合问题

/// [377. 组合总和 Ⅳ](https://leetcode.cn/problems/combination-sum-iv)
///
/// 思路:
/// 整体问题可以变成f(n) = f(n-nums[0]) + f(n-nums[1]) + ... + f(n-nums[i])
/// 但是这种朴素的递归操作, 会因为重复计算, 使整体的计算规模扩大
///
/// 可以和 [70. 爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/) 一样, 用dp来解决
///
/// 起点一定是0, 从0开始, 递推到target
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for i in 1..=target {
        for &num in nums.iter() {
            if i >= num {
                dp[i as usize] += dp[(i - num) as usize];
            }
        }
    }
    dp[target as usize]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_combination_sum4() {
        struct TestCase {
            nums: Vec<i32>,
            target: i32,
            expected: i32,
        }

        vec![
            TestCase {
                nums: vec![1, 2, 3],
                target: 4,
                expected: 7,
            },
            TestCase {
                nums: vec![9],
                target: 3,
                expected: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                TestCase {
                    nums,
                    target,
                    expected,
                },
            )| {
                let actual = combination_sum4(nums, target);
                assert_eq!(expected, actual, "case {} failed", idx);
            },
        );
    }
}
