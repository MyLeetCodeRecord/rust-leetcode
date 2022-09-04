//! 最长子序列
//! 题目：
//! * [300. 最长递增子序列](length_of_lis)
//! * [646. 最长数对链](find_longest_chain)

/// [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/)
/// dp解法的复杂度是 `O(n^2)` 更优方法见 [贪心解法](crate::moreandmore::length_of_lis)
///
/// 以 nums[i] 结尾， 如果前一个是 nums[j], 则 `f{i} = max{f(i), f(j)+1}`
///
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        dp[i] = 1;
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }
    dp.iter().max().copied().unwrap_or(0)
}

/// [646. 最长数对链](https://leetcode.cn/problems/maximum-length-of-pair-chain/)
///
/// [贪心解法](crate::moreandmore::find_longest_chain)
///
/// dp 解法思路和[300. 最长递增子序列](length_of_lis)一样
///
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort();

    let mut dp = vec![1; pairs.len()];
    for i in 1..pairs.len() {
        dp[i] = 1;
        let stub = pairs.get(i).unwrap();
        for j in 0..i {
            let cursor = pairs.get(j).unwrap();
            if stub[0] > cursor[1] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }
    dp.iter().max().copied().unwrap_or(0)
}

/// [435. 无重叠区间](https://leetcode.cn/problems/non-overlapping-intervals/)
///
/// [646. 最长数对链](find_longest_chain) 的变种问法， 能组成最长的递增序列， 也就需要剔除的最少
/// 
/// dp的复杂度是 `O(n^2)`, 会超时, [贪心算法](crate::moreandmore::erase_overlap_intervals)时间ok
///
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut pairs = intervals;
    pairs.sort();

    let mut dp = vec![1; pairs.len()];
    for i in 1..pairs.len() {
        dp[i] = 1;
        let stub = pairs.get(i).unwrap();
        for j in 0..i {
            let cursor = pairs.get(j).unwrap();
            if stub[0] >= cursor[1] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }
    let max =dp.iter().max().copied().unwrap_or(0);

    pairs.len() as i32 - max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_overlap_intervals() {
        struct TestCase {
            name: &'static str,
            intervals: &'static [[i32; 2]],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                intervals: &[[1, 2], [2, 3], [3, 4], [1, 3]],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                intervals: &[[1, 2], [1, 2], [1, 2]],
                expect: 2,
            },
            TestCase {
                name: "basic 1",
                intervals: &[[1, 2], [2, 3]],
                expect: 0,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let intervals = testcase.intervals.iter().map(|p| p.to_vec()).collect();
            let actual = erase_overlap_intervals(intervals);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_longest_chain() {
        struct TestCase {
            name: &'static str,
            pair: &'static [(i32, i32)],
            expect: i32,
        }

        vec![TestCase {
            name: "basic 1",
            pair: &[(1, 2), (2, 3), (3, 4)],
            expect: 2,
        }]
        .iter()
        .for_each(|testcase| {
            let pair = testcase.pair.iter().map(|p| vec![p.0, p.1]).collect();
            let actual = find_longest_chain(pair);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_length_of_lis() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[10, 9, 2, 5, 3, 7, 101, 18],
                expect: 4,
            },
            TestCase {
                name: "basic 2",
                nums: &[0, 1, 0, 3, 2, 3],
                expect: 4,
            },
            TestCase {
                name: "basic 3",
                nums: &[7, 7, 7, 7, 7, 7, 7],
                expect: 1,
            },
            TestCase {
                name: "basic 4",
                nums: &[],
                expect: 0,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = length_of_lis(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
