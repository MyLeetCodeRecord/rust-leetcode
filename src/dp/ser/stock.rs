//! 股票买卖相关系列
//!
//! ## 题目
//! * [121. 买卖股票的最佳时机](max_profit)
//! * [122. 买卖股票的最佳时机 II](max_profit_1)
//! * [123. 买卖股票的最佳时机 III](max_profit_2)
//! * [188. 买卖股票的最佳时机 IV](max_profit_3)
//! * [309. 最佳买卖股票时机含冷冻期](max_profit_4)
//! * [714. 买卖股票的最佳时机含手续费](max_profit_5)
//! * [剑指 Offer 63. 股票的最大利润](max_profit_6)

/// [121. 买卖股票的最佳时机](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/)
///
/// 只能交易一次, 因此最低价买入, 最高价卖出即可.
///
/// 思路:
/// * 在第i天买入, 在后面最高价时卖出, 获得最大利润
/// * 取最大利润中的最大值
///
/// 同样的, 正向遍历, 记录最低价格也是可以的.
/// ```
/// pub fn max_profit(prices: Vec<i32>) -> i32 {
///     let mut max = prices.last().copied().unwrap();
///     let mut profit = 0;
///
///     for &price in prices.iter().rev() {
///         profit = std::cmp::max(profit, max - price);
///         max = std::cmp::max(max, price);
///     }
///
///     profit
/// }
/// ```
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices.first().copied().unwrap();
    let mut profit = 0;

    for price in prices {
        profit = std::cmp::max(profit, price - min);
        min = std::cmp::min(min, price);
    }
    profit
}

/// [122. 买卖股票的最佳时机 II](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/)
///
/// 思路1: 贪心
/// 关键点:
///     * 由于同一天可以先卖出再买入
///     * 因此最终的利润可以被拆解到每天(每两天的差值)
///     * 最大利润即全部正向收益.
/// ```
/// pub fn max_profit_1(prices: Vec<i32>) -> i32 {
///     prices.windows(2).filter_map(|win|{
///         let (yestday, today) = (win.first().unwrap(), win.last().unwrap());
///         if today > yestday{
///             Some(today-yestday)
///         } else{
///             None
///         }
///     }).sum()
/// }
/// ```
/// 思路2: dp
/// 逻辑:
///     * **最大利润, 可以理解为最终手头现金最多**
///     * 假设初始为0, 买入为贷款(现金变为负数), 卖出收回现金
///     * 由于至多持有一只股票, 因此每天结束的状态就两种, 手头有一只, 或没有
///     * 如果手头没有,
///         * 可能是昨天没有, 今天不操作, 因此现金不变
///         * 可能昨天有, 今天卖出, 因此现金变为 昨天的现金数 + 今日卖出收回的
///     * 如果手头有,
///         * 可能昨天没有, 今天买入, 因此现金变为 昨天的现金数 - 今天买入花费
///         * 可能昨天有, 今天不操作, 因此现金数不变
///     * 于是, 现金数最多的, 即为这四种情况的最终最大值
///     * 不过手头没有股票, 必然比当前结束手头有一只的现金多
///     * 因此最终返回没有股票时的现金数就好.
/// ```
/// pub fn max_profit_1(prices: Vec<i32>) -> i32 {
///     let mut dp = vec![(0, 0);prices.len()];
///     dp[0].0 = 0;
///     dp[0].1 = 0 - prices.first().copied().unwrap();
///
///     for i in 1..prices.len(){
///         dp[i].0 = std::cmp::max(dp[i-1].0, dp[i-1].1+prices[i]);
///         dp[i].1 = std::cmp::max(dp[i-1].1, dp[i-1].0-prices[i]);
///     }
///     dp.last().unwrap().0
/// }
/// ```
///
/// dp优化, 由于递推关系中, 只需要关注前面一天的结果, 因此可以不用存储全量数据
/// 同时也将状态做了化简:
/// * 当天买入
/// * 当天卖出
/// 看两种状态最后的现金数量.
pub fn max_profit_1(prices: Vec<i32>) -> i32 {
    let mut today = (0, 0 - prices.first().copied().unwrap());
    for price in prices {
        let yestday = today;
        today.0 = std::cmp::max(yestday.0, yestday.1 + price);
        today.1 = std::cmp::max(yestday.1, yestday.0 - price);
    }
    today.0
}

/// [123. 买卖股票的最佳时机 III](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iii/)
///
/// 要求:
///     * 必须在再次购买前出售掉之前的股票 => 至多持有一只股票
///     * 最多可以完成 两笔 交易 => 挑出利润最大的两次
///
/// 由于限制了最大交易次数, 因此和[122. 买卖股票的最佳时机 II](max_profit_1)有些不同.
///
/// 每天结束的状态, 有以下5种:(只限制了交易次数上限, 没设置下限)
/// * 未进行过任何操作
/// * 只进行过一次买操作, 记为buy1
/// * 进行了一次买操作和一次卖操作，即完成了一笔交易, 记为sell1
/// * 在完成了一笔交易的前提下，进行了第二次买操作, 记为buy2
/// * 完成了全部两笔交易, 即为sell2
///
/// 单纯的buy1和sell1 其实就是[122. 买卖股票的最佳时机 II](max_profit_1)
/// 这个题目最终要得是sell2
pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let (mut buy1, mut sell1) = (0 - prices.first().copied().unwrap(), 0);
    let (mut buy2, mut sell2) = (0 - prices.first().copied().unwrap(), 0);

    for &price in prices.iter().skip(1) {
        buy1 = std::cmp::max(buy1, 0 - price);
        sell1 = std::cmp::max(sell1, buy1 + price);
        buy2 = std::cmp::max(buy2, sell1 - price);
        sell2 = std::cmp::max(sell2, buy2 + price);
    }
    sell2
}

/// [188. 买卖股票的最佳时机 IV](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/)
/// 相较[123. 买卖股票的最佳时机 III](max_profit_2), 将最多交易2次, 变为了k次
/// 如果k不限制(或足够多), 其实就是[122. 买卖股票的最佳时机 II](max_profit_1)
///
/// 通过[123. 买卖股票的最佳时机 III](max_profit_2)可以看出 `buy`和`sell`之间的递推关系
pub fn max_profit_3(k: i32, prices: Vec<i32>) -> i32 {
    if prices.is_empty() || k <= 0 {
        return 0;
    }
    let mut buy = vec![0 - prices.first().copied().unwrap(); k as usize];
    let mut sell = vec![0; k as usize];

    for price in prices.into_iter().skip(1) {
        buy[0] = std::cmp::max(buy[0], 0 - price);
        sell[0] = std::cmp::max(sell[0], buy[0] + price);
        for i in 1..buy.len() {
            buy[i] = std::cmp::max(buy[i], sell[i - 1] - price);
            sell[i] = std::cmp::max(sell[i], buy[i] + price);
        }
    }
    sell.last().copied().unwrap()
}

/// [309. 最佳买卖股票时机含冷冻期](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)
///
/// 冷却期, 增加了没有股票的子场景.
///
/// 由于递推只依赖前一项, 因此可以节省存储空间
pub fn max_profit_4(prices: Vec<i32>) -> i32 {
    let mut cash = vec![(0, 0, 0); prices.len()];
    cash[0].0 = 0 - prices.first().copied().unwrap(); // 持有一支, 要求前面一天离开冷却期
    cash[0].1 = 0; // 没有持有, 可能今天卖出的, 进入冷却期, 要求昨天持有一支
    cash[0].2 = 0; // 没有持有, 昨天卖出的, 今天冷却期; 不能操作, 今天结束, 离开冷却期, 要求前面一天进入冷却期, 或者前面一天也不是冷却期

    for i in 1..prices.len() {
        let price = prices[i];
        cash[i].0 = std::cmp::max(cash[i - 1].0, cash[i - 1].2 - price);
        cash[i].1 = cash[i - 1].0 + price;
        cash[i].2 = std::cmp::max(cash[i - 1].2, cash[i - 1].1); // 为啥要比较?
    }

    let last = cash.last().unwrap();
    std::cmp::max(last.1, last.2)
}

/// [714. 买卖股票的最佳时机含手续费](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)
///
/// 买入时不收, 卖出时收,
pub fn max_profit_5(prices: Vec<i32>, fee: i32) -> i32 {
    let (mut keep, mut empty) = (0 - prices.first().copied().unwrap(), 0);
    for &price in prices.iter().skip(1) {
        keep = std::cmp::max(empty - price, keep);
        empty = std::cmp::max(keep + price - fee, empty);
    }
    empty
}

/// [剑指 Offer 63. 股票的最大利润](https://leetcode.cn/problems/gu-piao-de-zui-da-li-run-lcof/)
///
/// 同
/// 只能交易一次, 因此最低价买入, 最高价卖出即可.
pub fn max_profit_6(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let mut min = prices.first().copied().unwrap();
    let mut max = 0;

    for price in prices {
        max = std::cmp::max(price - min, max);
        min = std::cmp::min(min, price);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_6() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[7, 1, 5, 3, 6, 4],
                expect: 5,
            },
            TestCase {
                name: "basic 2",
                prices: &[7, 6, 4, 3, 1],
                expect: 0,
            },
            TestCase {
                name: "fix 1",
                prices: &[],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let acutal = max_profit_6(testcase.prices.to_vec());
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit_5() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            fee: i32,
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[1, 3, 2, 8, 4, 9],
                fee: 2,
                expect: 8,
            },
            TestCase {
                name: "basic 2",
                prices: &[1, 3, 7, 5, 10, 3],
                fee: 3,
                expect: 6,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = max_profit_5(testcase.prices.to_vec(), testcase.fee);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit_4() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[1, 2, 3, 0, 2],
                expect: 3,
            },
            TestCase {
                name: "basic 2",
                prices: &[1],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = max_profit_4(testcase.prices.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit_3() {
        struct TestCase {
            name: &'static str,
            k: i32,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                k: 2,
                prices: &[2, 4, 1],
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                k: 2,
                prices: &[3, 2, 6, 5, 0, 3],
                expect: 7,
            },
            TestCase {
                name: "fix 1",
                k: 2,
                prices: &[],
                expect: 0,
            },
            TestCase {
                name: "fix 2",
                k: 0,
                prices: &[1, 3],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = max_profit_3(testcase.k, testcase.prices.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit_2() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[3, 3, 5, 0, 0, 3, 1, 4],
                expect: 6,
            },
            TestCase {
                name: "basic 2",
                prices: &[1, 2, 3, 4, 5],
                expect: 4,
            },
            TestCase {
                name: "basic 3",
                prices: &[7, 6, 4, 3, 1],
                expect: 0,
            },
            TestCase {
                name: "basic 4",
                prices: &[1],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = max_profit_2(testcase.prices.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit_1() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[7, 1, 5, 3, 6, 4],
                expect: 7,
            },
            TestCase {
                name: "basic 2",
                prices: &[1, 2, 3, 4, 5],
                expect: 4,
            },
            TestCase {
                name: "basic 3",
                prices: &[7, 6, 4, 3, 1],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = max_profit_1(testcase.prices.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_profit() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                prices: &[7, 1, 5, 3, 6, 4],
                expect: 5,
            },
            TestCase {
                name: "basic 2",
                prices: &[7, 6, 4, 3, 1],
                expect: 0,
            }]
        .iter()
        .for_each(|testcase| {
            let acutal = max_profit(testcase.prices.to_vec());
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
    }
}
