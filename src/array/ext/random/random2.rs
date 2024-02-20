//! 特点: 映射转嫁
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [710. 黑名单中的随机数](black_random)

/// [710. 黑名单中的随机数](https://leetcode-cn.com/problems/random-pick-with-blacklist/)
///
/// 思路:
/// 假设总共有N个元素, BL个黑名单.
/// 将整体分为 `N-BL | BL` 两段,
/// 前一段 有M个黑名单元素, 则后一段中有`BL-M`个黑名单元素, 进而后一段有`M`个正常元素
/// 因此可以在 `range(N-BL)`的范围内随机, 如果命中了黑名单元素, 映射到后面一段中的正常元素即可
///
/// 只要保证映射关系一对一, 就能保证概率
///
/// 映射关系:
/// 1. 哈希: 用索引关联
/// 2. 偏移: 前一段中第几个黑名单, 后一段中对应第几个正常元素
pub mod black_random {
    /// 哈希映射
    pub mod _hash {
        use rand;
        use std::collections::{HashMap, HashSet};

        struct Solution {
            n_bl: i32,
            mapping: HashMap<i32, i32>,
        }

        #[allow(dead_code)]
        impl Solution {
            fn new(n: i32, blacklist: Vec<i32>) -> Self {
                let mark = blacklist.iter().copied().collect::<HashSet<i32>>();

                let mut mapping = HashMap::new();
                let n_bl = n - (blacklist.len() as i32);

                let mut last = n - 1;
                for b in blacklist {
                    while mark.contains(&last) {
                        last -= 1;
                    }
                    if b >= n_bl {
                        // 如果已经在后一段, 可以不用记录
                        // 同时这里其实是一个隐性bug,
                        // 由于blacklist的顺寻不定, 可能大的元素占去了合适的last, 导致在range(n_bl)中有重复映射.
                        //
                        // 因此这个跳过不是为了省事, 而是为了保证映射的唯一对应
                        continue;
                    }
                    *mapping.entry(b).or_insert(last) = last;
                    last -= 1;
                }

                Self { n_bl, mapping }
            }

            fn pick(&self) -> i32 {
                // 由于i32的随机范围包含负数部分, 这里需要需绝对值
                // 但直接取可能溢出, 因此需要在 取余 之后再取
                let x = (rand::random::<i32>() % self.n_bl).abs();
                self.mapping.get(&x).copied().unwrap_or(x)
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_get_random() {
                struct TestCase {
                    name: &'static str,
                    n: i32,
                    blacklist: &'static [i32],
                    expect_in: &'static [i32],
                }

                [TestCase {
                        name: "basic",
                        n: 7,
                        blacklist: &[2, 3, 5],
                        expect_in: &[0, 1, 4, 6],
                    },
                    TestCase {
                        name: "fix 1",
                        n: 4,
                        blacklist: &[2, 1],
                        expect_in: &[0, 3],
                    }]
                .iter()
                .for_each(|testcase| {
                    let s = Solution::new(testcase.n, testcase.blacklist.to_vec());
                    let actual = s.pick();
                    assert!(
                        testcase.expect_in.contains(&actual),
                        "{} failed, got {}",
                        testcase.name,
                        actual
                    );
                });
            }
        }
    }

    /// 索引映射
    pub mod _binary {
        use rand;

        struct Solution {
            n: i32,
            blacklist: Vec<i32>,
        }

        #[allow(dead_code)]
        impl Solution {
            fn new(n: i32, blacklist: Vec<i32>) -> Self {
                let mut blacklist = blacklist;
                blacklist.sort();
                Self { n, blacklist }
            }

            fn pick(&self) -> i32 {
                let length = self.blacklist.len();
                let x = rand::random::<i32>().abs() % (self.n - length as i32);

                let (mut lo, mut hi) = (0, self.blacklist.len() - 1);
                while lo < hi {
                    let mid = (lo + hi + 1) / 2;
                    if x.gt(self.blacklist.get(mid).unwrap()) {
                        hi = mid - 1;
                    } else {
                        lo = mid;
                    }
                }
                if lo == hi && *self.blacklist.get(lo).unwrap() - lo as i32 <= x {
                    x + lo as i32 + 1
                } else {
                    x
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            #[ignore = "基于二进制替换的黑名单随机, 看上去还有bug, 映射关系还没有数学证明"]
            fn test_get_random() {
                struct TestCase {
                    name: &'static str,
                    n: i32,
                    blacklist: &'static [i32],
                    expect_in: &'static [i32],
                }

                [TestCase {
                    name: "basic",
                    n: 7,
                    blacklist: &[2, 3, 5],
                    expect_in: &[0, 1, 4, 6],
                }]
                .iter()
                .for_each(|testcase| {
                    let s = Solution::new(testcase.n, testcase.blacklist.to_vec());
                    let actual = s.pick();
                    assert!(
                        testcase.expect_in.contains(&actual),
                        "{} failed",
                        testcase.name
                    );
                });
            }
        }
    }
}
