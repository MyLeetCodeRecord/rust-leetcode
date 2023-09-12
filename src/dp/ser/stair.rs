//! 爬楼梯系列
//!
//! ## 题目
//! * [509. 斐波那契数](fib)
//! * [70. 爬楼梯](climb_stairs)
//! * [6058. 统计打字方案数](count_texts)
//! * [746. 使用最小花费爬楼梯](min_cost_climbing_stairs)
//!


/// [509. 斐波那契数](https://leetcode.cn/problems/fibonacci-number/)
pub fn fib(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

/// [70. 爬楼梯](https://leetcode.cn/problems/climbing-stairs/)
///
/// 递推关系: `f(i) = f(i-1) + f(i-2)`
/// 走到当前台阶, 可以由前面一个, 迈一步, 也可以由前面两个, 迈两步, 总共两种可能.
/// 假设走到前面一步, 总共有`f(i-1)`种方式, 再延伸一步, 方式数没有变,
/// 假设走到前面两步, 总共有`f(i-2)`种方式, 再延伸两步, 方式数没有变.
///
/// 因此走到当前台阶, 有两种来源, 一个是`f(i-1)`, 一个是`f(i-2)`
///
/// 由于递推关系只需要前两个, 因此不用数组全存, 存一点就好.
///
/// 有点 斐波那契数 的味道
pub fn climb_stairs(n: i32) -> i32 {
    // 初始状态 (1, 2)
    let (mut a, mut b) = (1, 2);
    // 衍生一次得到(2, 3), 因此循环次数 n-1 就好
    for _ in 2..=n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    a
}

/// [6058. 统计打字方案数](https://leetcode.cn/problems/count-number-of-texts/)
/// 只有连续相同的数字才有歧义, 不同的数字没有歧义
/// 2, 3, 4, 5, 6, 8的处理方式相同, 都是3个字母
/// 7, 9处理方式相同, 都是4个字母
///
/// 不同数字之间是乘法关系
///
/// 这里的递推关系, 其实可以视为爬楼梯的另一种说法,
/// 比如 `2222222`, 其实可以视为高度为7的楼梯, 可以一次上一步, 一次上两步, 一次上三步, 问总共有多少种方法爬上
/// 因此递推关系为 `f(i) = f(i-1) + f(i-2) + f(i-3)`
///
/// 同理推出7, 9的递推公式: `f(i) = f(i-1) + f(i-2) + f(i-3) + f(i-4)`
///
/// 为了减少重复计算, 使用数组存储已经计算的结果.
///
pub fn count_texts(pressed_keys: String) -> i32 {
    const MOD: i64 = 1000000007;

    fn perm_1(dp: &mut Vec<i64>, count: usize) -> i64 {
        while dp.len() < count {
            let length = dp.len();
            let new = dp[length - 1] + dp[length - 2] + dp[length - 3];
            dp.push(new % MOD);
        }

        dp.get(count - 1).copied().unwrap()
    }
    fn perm_7(dp: &mut Vec<i64>, count: usize) -> i64 {
        while dp.len() < count {
            let length = dp.len();
            let new = dp[length - 1] + dp[length - 2] + dp[length - 3] + dp[length - 4];
            dp.push(new % MOD);
        }

        dp.get(count - 1).copied().unwrap()
    }

    let mut dp_1: Vec<i64> = vec![1, 2, 4];
    let mut dp_7: Vec<i64> = vec![1, 2, 4, 8];

    let mut ans = 1i64;

    let bytes = pressed_keys.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let curr_c = bytes[i];
        let mut curr_cnt = 1;
        while i + curr_cnt < bytes.len() && bytes[i + curr_cnt] == curr_c {
            curr_cnt += 1;
        }
        let part = {
            if curr_c == b'7' || curr_c == b'9' {
                perm_7(&mut dp_7, curr_cnt)
            } else {
                perm_1(&mut dp_1, curr_cnt)
            }
        };

        ans = (ans * (part % MOD)) % MOD;

        i += curr_cnt;
    }

    ans as i32
}

/// [746. 使用最小花费爬楼梯](https://leetcode.cn/problems/min-cost-climbing-stairs/)
///
/// 记到i位置时, 最低费用为`f(i)`, 来源可以是前面一步, 也可能是前面的两步,
/// 加上走最后一步的cost, 看哪个小, 也就是到达这个位置的最小花费.
/// 因此`f(i) = min(f(i-1)+cost[i-1], f(i-2)+cost[i-2])`
///
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let (mut prev, mut curr) = (0, 0);
    for i in 2..=cost.len() {
        let nxt = std::cmp::min(curr + cost[i - 1], prev + cost[i - 2]);
        prev = curr;
        curr = nxt;
    }
    curr
}

/// [837. 新 21 点](https://leetcode.cn/problems/new-21-game/)
///
/// 信息:
/// 1. 获得 k 分 或更多分 时，停止抽取数字
///     * 因此最终停止的分数范围为 `[k, k-1+max_pts]`
/// 2. 用 `dp[i]` 存下从 i 变成 `[K, K-1+max_pts]` 内, 不超过n的概率
///     * 可以倒着看, 跳跃的台阶数从1到`max_pts`都可能
///     * 最终目标变成了求 `dp[0]`
/// 3. 假设跳跃一步之后到达j, 则 `dp[i] = sum( dp[j] * 1/max_pts)`
///     * 不同的链路之间是概率加和关系
///     * 同一条链路是乘法关系
/// 4. 初始状态:
///     * `dp[k..=n..min(k-1+max_pts)] = 1.0`
///
/// 实现1: 复杂度 O(n + k * max_pts), 会超时
/// ```rust
/// pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
///     if k == 0 {
///         return 1.0f64;
///     }
///     let mut dp = vec![0.0f64; (k + max_pts) as usize];
///     for i in k..=n.min(k+max_pts-1){
///         dp[i as usize] = 1.0;
///     }
///
///     for i in (0..k as usize).rev() {
///         for j in 1..=max_pts as usize {
///             dp[i] += dp[i + j] / (max_pts as f64);
///         }
///     }
///     dp[0]
/// }
/// ```
///
/// 实现2: 将上面的累加 做差, 可以计算出相邻两项的递推关系
///
pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    if k == 0 {
        return 1.0f64;
    }
    let mut dp = vec![0f64; (k + max_pts) as usize];
    for i in k..=n.min(k - 1 + max_pts) {
        dp[i as usize] = 1.0f64;
    }
    dp[k as usize - 1] = (max_pts.min(n - k + 1) as f64) / (max_pts as f64); // O(1) 计算 dp[k-1]
    for i in (0..k - 1).rev() {
        let i = i as usize;
        dp[i] = dp[i + 1] - (dp[i + max_pts as usize + 1] - dp[i + 1]) / (max_pts as f64);
    }
    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new21_game() {
        struct TestCase {
            n: i32,
            k: i32,
            max_pts: i32,
            expect: f64,
        }

        vec![
            TestCase {
                n: 10,
                k: 1,
                max_pts: 10,
                expect: 1.0000,
            },
            TestCase {
                n: 6,
                k: 1,
                max_pts: 10,
                expect: 0.6,
            },
            TestCase {
                n: 21,
                k: 17,
                max_pts: 10,
                expect: 0.73278,
            },
            TestCase {
                n: 0,
                k: 0,
                max_pts: 1,
                expect: 1.0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                n,
                k,
                max_pts,
                expect,
            } = testcase;
            let actual = new21_game(n, k, max_pts);
            assert!(
                (actual - expect).abs() <= 0.00001,
                "case {} failed, expect {}, got {}",
                idx,
                expect,
                actual
            );
        });
    }

    #[test]
    fn test_min_cost_climbing_stairs() {
        struct TestCase {
            name: &'static str,
            cost: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                cost: &[10, 15, 20],
                expect: 15,
            },
            TestCase {
                name: "basic 2",
                cost: &[1, 100, 1, 1, 1, 100, 1, 1, 100, 1],
                expect: 6,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = min_cost_climbing_stairs(testcase.cost.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_count_texts() {
        struct TestCase {
            name: &'static str,
            pressed_keys: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                pressed_keys: "22233",
                expect: 8,
            },
            TestCase {
                name: "basic 2",
                pressed_keys: "222222222222222222222222222222222222",
                expect: 82876089,
            },
            TestCase{
                name: "fix 1",
                pressed_keys: "444444444444444444444444444444448888888888888888999999999999333333333333333366666666666666662222222222222222666666666666666633333333333333338888888888888888222222222222222244444444444444448888888888888222222222222222288888888888889999999999999999333333333444444664",
                expect: 537551452
            },
            TestCase{
                name: "fix 2",
                pressed_keys: "88888888888888888888888888888999999999999999999999999999994444444444444444444444444444488888888888888888888888888888555555555555555555555555555556666666666666666666666666666666666666666666666666666666666222222222222222222222222222226666666666666666666666666666699999999999999999999999999999888888888888888888888888888885555555555555555555555555555577777777777777777777777777777444444444444444444444444444444444444444444444444444444444433333333333333333333333333333555555555555555555555555555556666666666666666666666666666644444444444444444444444444444999999999999999999999999999996666666666666666666666666666655555555555555555555555555555444444444444444444444444444448888888888888888888888888888855555555555555555555555555555555555555555555555555555555555555555555555555555555555999999999999999555555555555555555555555555554444444444444444444444444444444555",
                expect: 886136986
            }
        ]
        .iter()
        .for_each(|testcase| {
            let actual = count_texts(testcase.pressed_keys.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_climb_stairs() {
        struct TestCase {
            name: &'static str,
            n: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                n: 2,
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                n: 3,
                expect: 3,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let acutal = climb_stairs(testcase.n);
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
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
                name: "basic 1",
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
            let actual = fib(testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
