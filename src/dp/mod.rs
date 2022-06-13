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
//!     * [509. 斐波那契数](fib)
//! * 困难
//!     * [473. 火柴拼正方形](makesquare)
//!

/// 系列题目
///
/// * [股票买卖系列](stock)
/// * [爬楼梯系列](stair)
pub mod ser;

/// [509. 斐波那契数](https://leetcode.cn/problems/fibonacci-number/)
///
/// 数列的定义就是用递推关系说明的. 即自带状态转移方程.
/// 由于递推关系中其实只需要前面两项, 因此可以只存前面两项
///
/// 初始为 (0, 1)
/// 后续为 (1, 1), (1, 2), (2, 3)
///
/// 初始值不用计算, 后面每个迭代一次, 因此`1..=n` 即可
pub fn fib(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    for _ in 1..=n {
        // 运算n次
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    a
}

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
            if (prev < curr && curr - prev == 1) || (*prev > *curr && *prev == b'z' && *curr == b'a') {
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
    if matchsticks.len() < 4{
        return false;
    }

    let mut matchsticks = matchsticks;

    let sum : i32 = matchsticks.iter().sum();
    if sum % 4 != 0{
        return false;
    }

    let len = sum/4;

    matchsticks.sort();
    if matchsticks.last().copied().unwrap() > len{
        return false;
    }

    // 总共有 2**n个可能
    // dp的索引枚举, 就变成了 全0, 到全1 (位数为n)
    // 对应数位, 0表示不选, 1表示选择
    let mut dp = vec![-1; 1<<matchsticks.len()]; 
    // 初始都没选, 边长为0
    dp[0] = 0;

    for s in 1..dp.len(){
        // s 为第几种组合
        for (k, &v) in matchsticks.iter().enumerate(){
            if s & (1<<k) == 0{
                // 表示 已经 去掉第k根火柴
                continue;
            }
            // 去掉第k根火柴
            let s1 = s & (!(1<<k));
            // 加上第k根, 也不超过正方形边长
            // 同时 通过 `dp[s1] >= 0`保证火柴的选取顺序
            if dp[s1] >= 0 && dp[s1] + v <= len{
                dp[s] = (dp[s1] + v) % len; // 取余
                break;
            }
        }
    }
    dp.last().copied().unwrap() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makesquare(){
        struct TestCase{
            name: &'static str,
            matchsticks: &'static[i32],
            expect: bool
        }

        vec![
            TestCase{
                name: "basic 1",
                matchsticks: &[1,1,2,2,2],
                expect: true,
            },
            TestCase{
                name: "basic 2",
                matchsticks: &[3,3,3,3,4],
                expect: false,
            },
        ].iter().for_each(|testcase|{
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

    #[test]
    fn test_fib() {
        struct TestCase {
            name: &'static str,
            n: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                n: 2,
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                n: 3,
                expect: 2,
            },
            TestCase {
                name: "basic 3",
                n: 4,
                expect: 3,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let acutal = fib(testcase.n);
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
    }
}
