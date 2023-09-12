//! # 前缀和/前缀树
//!
//! 特点: 前缀; 区间; 原数组不动
//!
//! ## 题目
//! * 简单
//!     * [303. 区域和检索 - 数组不可变](NumArray)
//!     * [1480. 一维数组的动态和](running_sum)
//! * 中等
//!     * [304. 二维区域和检索 - 矩阵不可变](NumMatrix)
//!     * [560. 和为 K 的子数组](subarray_sum)
//! * 困难
//!

/// [303. 区域和检索 - 数组不可变](https://leetcode-cn.com/problems/range-sum-query-immutable/)
#[allow(dead_code)]
pub struct NumArray {
    pre: Vec<i32>,
}

#[allow(dead_code)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut pre = vec![0];
        let mut curr_sum = 0;
        for num in nums {
            curr_sum += num;
            pre.push(curr_sum);
        }
        Self { pre }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (mut left, mut right) = (left, right);
        if left < 0 {
            left = 0;
        }
        if right >= self.pre.len() as i32 {
            right = self.pre.len() as i32 - 2;
        }
        let (l, r) = (
            self.pre.get(left as usize).unwrap(),
            self.pre.get(right as usize + 1).unwrap(),
        );
        r - l
    }
}

/// [304. 二维区域和检索 - 矩阵不可变](https://leetcode-cn.com/problems/range-sum-query-2d-immutable/)
#[allow(dead_code)]
pub struct NumMatrix {
    pre: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (row, col) = (matrix.len(), matrix.first().unwrap().len());
        let mut pre = vec![vec![0; col + 1]; row + 1];

        for r in 1..=row {
            // 从 1 开始的
            for c in 1..=col {
                // 从1开始的, 含终点
                pre[r][c] =
                    pre[r - 1][c] + pre[r][c - 1] - pre[r - 1][c - 1] + matrix[r - 1][c - 1];
            }
        }

        Self { pre }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.pre[row2 + 1][col2 + 1] - self.pre[row1][col2 + 1] - self.pre[row2 + 1][col1]
            + self.pre[row1][col1]
    }
}

/// [560. 和为 K 的子数组](https://leetcode-cn.com/problems/subarray-sum-equals-k/)
///
/// 和[1. 两数之和](https://leetcode-cn.com/problems/two-sum/) 相似, 只是这个是两数之差
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut curr = 0;
    let mut store = HashMap::new();

    // 用来处理 前缀和 恰好等于k 的情况
    // store.insert(0, 1);

    let mut ret = 0;
    for num in nums {
        curr += num;
        // 或者手动判是否为k, 手动+1
        if curr == k {
            ret += 1;
        }

        // 两数之差, 目标值
        let target = curr - k;
        ret += store.get(&target).unwrap_or(&0);

        *store.entry(curr).or_insert(0) += 1;
    }

    ret as i32
}

/// [1480. 一维数组的动态和](https://leetcode.cn/problems/running-sum-of-1d-array/)
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for i in 1..nums.len() {
        nums[i] = nums[i] + nums[i - 1];
    }
    nums
}

/// [848. 字母移位](https://leetcode.cn/problems/shifting-letters/)
///
/// 从后向前累加的前缀和
///
pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let s = s;
    let mut ss = s.as_bytes().to_vec();

    let mut sum = 0;
    for i in (0..ss.len()).rev() {
        sum = (sum + shifts[i]) % 26;
        let tmp = (ss[i] - b'a') as i32 + sum;
        ss[i] = (tmp % 26) as u8 + b'a';
    }
    String::from_utf8(ss).unwrap()
}

/// [862. 和至少为 K 的最短子数组](https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/)
/// 
/// 误区:
/// 1. 滑动窗口: 添加负数之后窗口的滑动就没有单向性了，因此无法使用滑动窗口解决
/// 2. 数据输入可能有溢出
/// 
pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;

    let k = k as i64;

    let mut pre = vec![0];
    let mut curr_sum = 0i64;
    for num in nums {
        curr_sum += num as i64;
        pre.push(curr_sum);
    }

    let mut ret = std::usize::MAX;
    let mut deque = VecDeque::new();
    for (i, &num) in pre.iter().enumerate() {
        while !deque.is_empty() && num - pre[*deque.front().unwrap()] >= k {
            ret = ret.min(i - deque.pop_front().unwrap());
        }
        while !deque.is_empty() && num <= pre[*deque.back().unwrap()] {
            deque.pop_back();
        }
        deque.push_back(i);
    }

    if ret == std::usize::MAX {
        -1
    } else {
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_subarray() {
        struct TestCase {
            nums: Vec<i32>,
            k: i32,
            expect: i32,
        }

        vec![
            TestCase {
                nums: vec![1],
                k: 1,
                expect: 1,
            },
            TestCase {
                nums: vec![1, 2],
                k: 4,
                expect: -1,
            },
            TestCase {
                nums: vec![2, -1, 2],
                k: 3,
                expect: 3,
            },
            TestCase{
                nums: vec![84,-37,32,40,95],
                k: 167,
                expect: 3
            },
            TestCase{
                nums: vec![45,95,97,-34,-42],
                k: 21,
                expect: 1
            },
            TestCase{
                nums: vec![-34,37,51,3,-12,-50,51,100,-47,99,34,14,-13,89,31,-14,-44,23,-38,6],
                k: 151,
                expect: 2
            },
            TestCase{
                nums: vec![-100000;100000],
                k: 1000000000,
                expect: -1
            }
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { nums, k, expect })| {
            let ans = shortest_subarray(nums, k);
            assert_eq!(ans, expect, "case {} failed", idx);
        })
    }

    #[test]
    fn test_shifting_letters() {
        struct TestCase {
            s: &'static str,
            shifts: Vec<i32>,
            expect: &'static str,
        }

        vec![
            TestCase {
                s: "abc",
                shifts: vec![3, 5, 9],
                expect: "rpl",
            },
            TestCase {
                s: "aaa",
                shifts: vec![1, 2, 3],
                expect: "gfd",
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, shifts, expect } = testcase;
            let actual = shifting_letters(s.to_string(), shifts);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_running_sum() {
        struct TestCase {
            nums: Vec<i32>,
            expect: Vec<i32>,
        }

        vec![
            TestCase {
                nums: vec![1, 2, 3, 4],
                expect: vec![1, 3, 6, 10],
            },
            TestCase {
                nums: vec![1, 1, 1, 1, 1],
                expect: vec![1, 2, 3, 4, 5],
            },
            TestCase {
                nums: vec![3, 1, 2, 10, 1],
                expect: vec![3, 4, 6, 16, 17],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { nums, expect } = testcase;
            let actual = running_sum(nums);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_subarray_sum() {
        struct TestCase {
            nums: &'static [i32],
            k: i32,
            expect: i32,
        }

        vec![
            TestCase {
                nums: &[1, 1, 1],
                k: 2,
                expect: 2,
            },
            TestCase {
                nums: &[1, 2, 3],
                k: 3,
                expect: 2,
            },
            TestCase {
                nums: &[1],
                k: 0,
                expect: 0,
            },
            TestCase {
                nums: &[-1, -1, 1],
                k: 0,
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let actual = subarray_sum(testcase.nums.to_vec(), testcase.k);
            assert_eq!(testcase.expect, actual, "case {} failed", idx);
        })
    }

    #[test]
    fn test_sum_region() {
        struct Range {
            row1: i32,
            col1: i32,
            row2: i32,
            col2: i32,
        }

        struct TestCase {
            name: &'static str,
            matrix: &'static [&'static [i32]],
            query: &'static [(Range, i32)],
        }

        vec![TestCase {
            name: "basic",
            matrix: &[
                &[3, 0, 1, 4, 2],
                &[5, 6, 3, 2, 1],
                &[1, 2, 0, 1, 5],
                &[4, 1, 0, 1, 7],
                &[1, 0, 3, 0, 5],
            ],
            query: &[
                (
                    Range {
                        row1: 2,
                        col1: 1,
                        row2: 4,
                        col2: 3,
                    },
                    8,
                ),
                (
                    Range {
                        row1: 1,
                        col1: 1,
                        row2: 2,
                        col2: 2,
                    },
                    11,
                ),
                (
                    Range {
                        row1: 1,
                        col1: 2,
                        row2: 2,
                        col2: 4,
                    },
                    12,
                ),
            ],
        }]
        .iter()
        .for_each(|testcase| {
            let nums = testcase.matrix.iter().map(|row| row.to_vec()).collect();
            let na = NumMatrix::new(nums);
            testcase
                .query
                .iter()
                .enumerate()
                .for_each(|(idx, (rng, expect))| {
                    let actual = na.sum_region(rng.row1, rng.col1, rng.row2, rng.col2);
                    assert_eq!(*expect, actual, "{} {} failed", testcase.name, &idx);
                });
        })
    }

    #[test]
    fn test_sum_range() {
        struct Range {
            left: i32,
            right: i32,
        }
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            query: &'static [(Range, i32)],
        }

        vec![TestCase {
            name: "basic",
            nums: &[-2, 0, 3, -5, 2, -1],
            query: &[
                (Range { left: 0, right: 2 }, 1),
                (Range { left: 2, right: 5 }, -1),
                (Range { left: 0, right: 5 }, -3),
            ],
        }]
        .iter()
        .for_each(|testcase| {
            let na = NumArray::new(testcase.nums.to_vec());
            testcase
                .query
                .iter()
                .enumerate()
                .for_each(|(idx, (rng, expect))| {
                    let actual = na.sum_range(rng.left, rng.right);
                    assert_eq!(*expect, actual, "{} {} failed", testcase.name, &idx);
                });
        })
    }
}
