//! DP
//!
//! DP的关键在于状态转移方程, 对应数学上, 也就是递推公式.
//!
//! 既然是递推, 也就是规模可以缩减. 只是思考的方向是 后一项能否用前面的状态推出.
//! 在确定了递推关系后, 还有一个难点就是初始状态.
//!
//! 其实从数据的角度看, 就是一个数列, 给出了前几项和递推关系, 然后让你求数列的第N项.
//!
//! 状态转移, 又有点记忆化搜索的味道, 不然就会出现大量的重复计算, 因此常见的就是维护一个dp数组, 维护出现过的状态.
//!
//! 这里面有一些是系列的题目, 放到了[ser]中;
//! 其他一些散装的, 就直接挂载了mod下.
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [926. 将字符串翻转到单调递增](min_flips_mono_incr)
//! * 困难
//!     * [473. 火柴拼正方形](makesquare)
//!

pub mod ser;

/// [467. 环绕字符串中唯一的子字符串](https://leetcode.cn/problems/unique-substrings-in-wraparound-string/)
pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    let mut cache = [0; 26];

    let pb = p.as_bytes();
    let mut k = 0;
    for i in 0..pb.len() {
        let curr = pb.get(i).unwrap();
        if i == 0 {
            k = 1;
        } else {
            let prev = pb.get(i - 1).unwrap();
            // 一般都是 abc这种
            // 出现反复的, 只有 za
            if (prev < curr && curr - prev == 1)
                || (*prev > *curr && *prev == b'z' && *curr == b'a')
            {
                k += 1;
            }
        }
        let chi = (*curr - b'a') as usize;
        cache[chi] = std::cmp::max(cache[chi], k);
    }
    cache.into_iter().sum::<i32>()
}

/// [473. 火柴拼正方形](https://leetcode.cn/problems/matchsticks-to-square/)
///
/// 思路: dp
/// 题目要求每个都用上. 因此选与不选, 2**n种情况
///
/// 状态压缩方式: 数位
pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    // 一些预检查
    if matchsticks.len() < 4 {
        return false;
    }

    let mut matchsticks = matchsticks;

    let sum: i32 = matchsticks.iter().sum();
    if sum % 4 != 0 {
        return false;
    }

    let len = sum / 4;

    matchsticks.sort();
    if matchsticks.last().copied().unwrap() > len {
        return false;
    }

    // 总共有 2**n个可能
    // dp的索引枚举, 就变成了 全0, 到全1 (位数为n)
    // 对应数位, 0表示不选, 1表示选择
    let mut dp = vec![-1; 1 << matchsticks.len()];
    // 初始都没选, 边长为0
    dp[0] = 0;

    for s in 1..dp.len() {
        // s 为第几种组合
        for (k, &v) in matchsticks.iter().enumerate() {
            if s & (1 << k) == 0 {
                // 表示 已经 去掉第k根火柴
                continue;
            }
            // 去掉第k根火柴
            let s1 = s & (!(1 << k));
            // 加上第k根, 也不超过正方形边长
            // 同时 通过 `dp[s1] >= 0`保证火柴的选取顺序
            if dp[s1] >= 0 && dp[s1] + v <= len {
                dp[s] = (dp[s1] + v) % len; // 取余
                break;
            }
        }
    }
    dp.last().copied().unwrap() == 0
}

/// [926. 将字符串翻转到单调递增](https://leetcode.cn/problems/flip-string-to-monotone-increasing/)
///
/// 每个位置, 其实有两种选择, 一个是0, 一个是1
/// 总共有 2**n中可能.
/// 和 [473. 火柴拼正方形](makesquare) 相似
/// 但很多状态不符合要求, 因此如果按照 473 的套路, 还需要剪枝
///
/// 记 dp[i-1] = (x, y) 分别为 第i-1为0, 为1需要的变化次数
/// dp[i].x = x + 1 如果 s[i]为1, 否则为 x
/// dp[i].y = min(dp[i].x, dp[i].y) + 1 如果s[i]为0, 否则为 min(dp[i].x, dp[i].y)
///
/// 最终 min(x, y)
pub fn min_flips_mono_incr(s: String) -> i32 {
    let mut dp = vec![(0, 0); s.len()];
    let bs = s.as_bytes();
    if bs[0] == b'0' {
        dp[0].1 = 1; // 原本为1, 变为0, 需要次数加1
    } else {
        dp[0].0 = 1; // 反之
    }

    bs.iter().enumerate().skip(1).for_each(|(i, &c)| {
        let prev = dp.get(i - 1).copied().unwrap();
        let d = dp.get_mut(i).unwrap();
        if c == b'0' {
            d.0 = prev.0; // 自身为0, 要求前面也为0才能递增, 当前为0, 不用变化, 不用加次数
            d.1 = std::cmp::min(prev.0, prev.1) + 1; // 自身为1, 前面可以是0, 也可以是1, 都递增. 当前为0, 变为1, 加次数1
        } else {
            d.0 = prev.0 + 1; // 原本为1, 变为0, 次数加1. 同时要求前面一个只能为0
            d.1 = std::cmp::min(prev.0, prev.1); // 自身为1, 不用变. 同时前面是0是1都可
        }
    });
    let last = dp.last().unwrap();
    std::cmp::min(last.0, last.1)
}

/// [256. 粉刷房子](https://leetcode.cn/problems/paint-house/)
///
/// 每个房子最终只有三种状态. 取最后一个房子的三种状态中额最小值, 即为答案
///
/// 同时这三种状态均只由前面一个房子即可推断出(要求相邻不相同)
///
/// 因此可以简化状态存储.
pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    let [mut a, mut b, mut c] =
        <[i32; 3]>::try_from(costs.first().unwrap().as_slice()).expect("unpack array");

    for cost in costs.into_iter().skip(1) {
        let [ax, bx, cx] = <[i32; 3]>::try_from(cost.as_slice()).expect("unpack array");

        let at = std::cmp::min(b, c) + ax;
        let bt = std::cmp::min(a, c) + bx;
        let ct = std::cmp::min(a, b) + cx;

        a = at;
        b = bt;
        c = ct;
    }

    [a, b, c].into_iter().min().expect("result")
}

/// [828. 统计子串中的唯一字符](https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string/)
pub fn unique_letter_string(s: String) -> i32 {
    let (mut prev, mut prev_diff) = ([-1; 26], [0; 26]);
    let mut ans = 0;

    for (i, &b) in s.as_bytes().iter().enumerate() {
        let (i, c) = (i as i32, (b - b'A') as usize);

        // 出现多次的
        ans = ans + prev_diff[c] * (i - prev[c]);
        prev_diff[c] = i - prev[c];
        prev[c] = i;
    }

    let length = s.len() as i32;
    // 对于只出现一次的， 需要补齐
    ans + prev
        .into_iter()
        .zip(prev_diff.into_iter())
        .map(|(a, b)| b * (length - a))
        .sum::<i32>()
}

/// [221. 最大正方形](https://leetcode.cn/problems/maximal-square/)
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let (m, n) = (matrix.len(), matrix.first().unwrap().len());
    // 给整体加一层， 用来处理溢出的问题
    // 这样 matrix[row][col] => dp[row+1][col+1]
    let mut dp = vec![vec![0; n + 1]; m + 1];

    let mut mc = 0;

    for row in 0..m {
        for col in 0..n {
            if matrix[row][col] == '1' {
                dp[row + 1][col + 1] = [dp[row + 1][col], dp[row][col + 1], dp[row][col]]
                    .into_iter()
                    .min() // leetcode版本太低， 这里还需要 .copied()
                    .unwrap_or(0)
                    + 1; // 这里是边长
            } else {
                dp[row + 1][col + 1] = 0;
            }
            mc = std::cmp::max(mc, dp[row + 1][col + 1]);
        }
    }
    mc * mc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_maximal_square() {
        struct TestCase {
            matrix: Vec<Vec<char>>,
            expect: i32,
        }

        vec![
            TestCase {
                matrix: vec2![
                    ['1', '0', '1', '0', '0'],
                    ['1', '0', '1', '1', '1'],
                    ['1', '1', '1', '1', '1'],
                    ['1', '0', '0', '1', '0']
                ],
                expect: 4,
            },
            TestCase {
                matrix: vec2![['0', '1'], ['1', '0']],
                expect: 1,
            },
            TestCase {
                matrix: vec2![['0']],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { matrix, expect } = testcase;
            let actual = maximal_square(matrix);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_unique_letter_string() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                s: "ABC",
                expect: 10,
            },
            TestCase {
                name: "basic 2",
                s: "ABA",
                expect: 8,
            },
            TestCase {
                name: "basic 3",
                s: "LEETCODE",
                expect: 92,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = unique_letter_string(testcase.s.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_min_cost() {
        struct TestCase {
            name: &'static str,
            costs: &'static [&'static [i32]],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                costs: &[&[17, 2, 17], &[16, 16, 5], &[14, 3, 19]],
                expect: 10,
            },
            TestCase {
                name: "basic 2",
                costs: &[&[7, 6, 2]],
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let costs = testcase.costs.iter().map(|c| c.to_vec()).collect();
            let actual = min_cost(costs);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_min_flips_mono_incr() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                s: "00110",
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                s: "010110",
                expect: 2,
            },
            TestCase {
                name: "basic 3",
                s: "00011000",
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = min_flips_mono_incr(testcase.s.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_makesquare() {
        struct TestCase {
            name: &'static str,
            matchsticks: &'static [i32],
            expect: bool,
        }

        vec![
            TestCase {
                name: "basic 1",
                matchsticks: &[1, 1, 2, 2, 2],
                expect: true,
            },
            TestCase {
                name: "basic 2",
                matchsticks: &[3, 3, 3, 3, 4],
                expect: false,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = makesquare(testcase.matchsticks.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_substring_in_wrapround_string() {
        struct TestCase {
            name: &'static str,
            p: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                p: "a",
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                p: "cac",
                expect: 2, // 去重
            },
            TestCase {
                name: "basic 3",
                p: "zab",
                expect: 6,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = find_substring_in_wrapround_string(testcase.p.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }
}
