//! 路径DP
//!

/// [62. 不同路径](https://leetcode.cn/problems/unique-paths/)
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n]; m];
    // 由于只能向下, 向右, 因此边界上的只有一种方式
    for i in 0..n {
        dp[0][i] = 1;
    }
    for i in 0..m {
        dp[i][0] = 1;
    }

    for row in 1..m {
        for col in 1..n {
            dp[row][col] = dp[row][col - 1] + dp[row - 1][col];
        }
    }
    dp[m - 1][n - 1]
}
/// [63. 不同路径 II](https://leetcode.cn/problems/unique-paths-ii/)
/// 相较[62. 不同路径](unique_paths)多了路障, 移动规则还是一样的
/// 1. 如果遇到路障, 路径不用计算
/// 2. 如果前一个点中有一个是路障, 则路径数量不变
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid.first().unwrap().len());

    // if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
    //     // SB边界case
    //     return 0;
    // }

    let mut dp = vec![vec![0; n]; m];
    for i in 0..n {
        if obstacle_grid[0][i] == 1 {
            break;
        }
        dp[0][i] = 1;
    }
    for i in 0..m {
        if obstacle_grid[i][0] == 1 {
            // 如果边界出现路障, 后续的不用赋值了
            break;
        }
        dp[i][0] = 1;
    }

    for row in 1..m {
        for col in 1..n {
            if obstacle_grid[row][col] == 1 {
                dp[row][col] = 0;
            } else {
                dp[row][col] = dp[row][col - 1] + dp[row - 1][col];
            }
        }
    }
    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_unique_paths_with_obstacles() {
        struct TestCase {
            obstacle_grid: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            TestCase {
                obstacle_grid: vec2![[0, 0, 0], [0, 1, 0], [0, 0, 0]],
                expect: 2,
            },
            TestCase {
                obstacle_grid: vec2![[0, 1], [0, 0]],
                expect: 1,
            },
            // SB测试用例
            TestCase {
                obstacle_grid: vec2![[1, 0]],
                expect: 0,
            },
            TestCase {
                obstacle_grid: vec2![[0, 0], [1, 1], [0, 0]],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                obstacle_grid,
                expect,
            } = testcase;
            let actual = unique_paths_with_obstacles(obstacle_grid);
            assert_eq!(expect, actual, "case {} failaed", idx);
        });
    }

    #[test]
    fn test_unique_paths() {
        struct TestCase {
            m: i32,
            n: i32,
            expect: i32,
        }

        vec![
            TestCase {
                m: 3,
                n: 7,
                expect: 28,
            },
            TestCase {
                m: 3,
                n: 2,
                expect: 3,
            },
            TestCase {
                m: 7,
                n: 3,
                expect: 28,
            },
            TestCase {
                m: 3,
                n: 3,
                expect: 6,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { m, n, expect } = testcase;
            let actual = unique_paths(m, n);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
