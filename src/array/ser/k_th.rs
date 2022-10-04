//! 系列: 第k个
//!
//! ## 题目
//! * 中等
//!     *
//! * 困难
//!     * [719. 找出第 K 小的数对距离](smallest_distance_pair)

/// [719. 找出第 K 小的数对距离](https://leetcode.cn/problems/find-k-th-smallest-pair-distance/)
///
/// 要求
/// 1. 数对 (nums[i], nums[j]) 0 <= i < j < nums.length 即不含自身
/// 2. 绝对差值, 因此(a, b) == (b, a) 顺序不关键
/// 3. 所有数对距离中第 k 小, 相等的绝对差值, 只要不是是同一个数对, 就视为不同的.
///
pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    // 差值二分
    let (mut left, mut right) = (0, nums.last().unwrap() - nums.first().unwrap());
    while left <= right {
        let mid = (left + right) / 2; // 取到中间差值
                                      // 遍历数组, 看有多少小于这个差值
        let cnt = {
            let mut cnt = 0;
            // 思路一: 从头遍历去判差值, 而是查找自身+差值之后, 插入点在哪, 但是由于有重复数据, rust目前的二分查找不支持查边界
            // 思路二: 双指针
            let (mut i, mut j) = (0, 0);
            while j < nums.len() {
                while j > i && nums[j] - nums[i] > mid {
                    // 不能自身, 因此可以约定 j>i
                    i += 1;
                }
                cnt += j - i;
                j += 1;
            }

            cnt
        };

        if cnt as i32 >= k {
            // 说明目标差值不在mid右边
            right = mid - 1;
        } else {
            // 说明目标差值在mid右边
            left = mid + 1;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_distance_pair() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            k: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[1, 3, 1],
                k: 1,
                expect: 0,
            },
            TestCase {
                name: "basic 1",
                nums: &[1, 1, 1],
                k: 2,
                expect: 0,
            },
            TestCase {
                name: "basic 1",
                nums: &[1, 6, 1],
                k: 3,
                expect: 5,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = smallest_distance_pair(testcase.nums.to_vec(), testcase.k);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
