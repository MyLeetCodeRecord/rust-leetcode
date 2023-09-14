//! 模拟类题目
//!
//! 题目
//! * 简单
//!     * [LCP 50. 宝石补给](give_gem)
//! * 中等
//!     * [2596. Check Knight Tour Configuration](check_valid_grid)
//!     * [1222. Queens That Can Attack the King](queens_attackthe_king)
//! * 困难
//!

/// [2596. Check Knight Tour Configuration](https://leetcode.cn/problems/check-knight-tour-configuration/)
///
/// 思路:
/// 1. 先根据config, 产出一个`(step, x, y)`的数组
/// 2. 按照step从小到大排序
/// 3. 从第一个元素开始, 依次判断是否符合条件
///     - 模拟走位, 看这个位置是否能从上一个位置走过来
pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    fn check_valid_step(curr: (i32, i32), target: (i32, i32)) -> bool {
        let (x0, y0) = curr;
        let (x1, y1) = target;

        // vec![
        //     (x0 - 2, y0 - 1),
        //     (x0 - 1, y0 - 2),
        //     (x0 + 1, y0 - 2),
        //     (x0 + 2, y0 - 1),
        //     (x0 + 2, y0 + 1),
        //     (x0 + 1, y0 + 2),
        //     (x0 - 1, y0 + 2),
        //     (x0 - 2, y0 + 1),
        // ]
        // .into_iter()
        // .filter(|(x, y)| *x >= 0 && *y >= 0)
        // .any(|(x, y)| x == x1 && y == y1)

        let x_diff = (x0 - x1).abs();
        let y_diff = (y0 - y1).abs();
        x_diff * y_diff == 2
    }

    let mut set = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, step) in row.iter().enumerate() {
            set.push((*step, x as i32, y as i32));
        }
    }
    set.sort_unstable_by_key(|(step, _, _)| *step);

    let mut cur_pos = (0, 0);
    for (step, x, y) in set {
        if step == 0 {
            if x != 0 || y != 0 {
                return false;
            }
            continue;
        }

        if !check_valid_step(cur_pos, (x, y)) {
            return false;
        }
        cur_pos = (x, y);
    }
    true
}

/// [1222. Queens That Can Attack the King](https://leetcode.cn/problems/queens-that-can-attack-the-king/)
///
/// 象棋规则: Queen可以横, 竖, 斜走, 但是不能跨越棋子
/// 思路:
/// 1. 从king的位置开始, 向8个方向遍历, 找到第一个queen
pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let queens = queens.into_iter().collect::<HashSet<Vec<i32>>>();
    let mut result = vec![];

    let (x, y) = (king[0], king[1]);

    for dir in [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ]
    .iter()
    {
        let (mut x, mut y) = (x, y);
        loop {
            x += dir.0;
            y += dir.1;
            if x < 0 || y < 0 || x >= 8 || y >= 8 {
                break;
            }
            if queens.contains(&vec![x, y]) {
                result.push(vec![x, y]);
                break;
            }
        }
    }
    result
}

/// [LCP 50. 宝石补给](https://leetcode.cn/problems/WHnhjV)
pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
    for op in operations {
        let (from, to) = (op[0] as usize, op[1] as usize);
        let how_much = gem[from] / 2;
        gem[from] = gem[from] - how_much;
        gem[to] = gem[to] + how_much;
    }
    // 可以不用排序, 遍历两次取最大最小值即可
    // 不能在op的同时计算最大最小, op可能不涉及全部节点
    gem.sort_unstable();
    gem.last().copied().unwrap() - gem.first().copied().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_give_gem() {
        struct TestCase {
            gem: Vec<i32>,
            operations: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            TestCase {
                gem: vec![3, 1, 2],
                operations: vec2![[0, 2], [2, 1], [2, 0]],
                expect: 2,
            },
            TestCase {
                gem: vec![100, 0, 50, 100],
                operations: vec2![[0, 2], [0, 1], [3, 0], [3, 0]],
                expect: 75,
            },
            TestCase {
                gem: vec![0, 0, 0, 0],
                operations: vec2![[1, 2], [3, 1], [1, 2]],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                TestCase {
                    gem,
                    operations,
                    expect,
                },
            )| {
                let actual = give_gem(gem, operations);
                assert_eq!(expect, actual, "case {} failed", idx);
            },
        )
    }

    #[test]
    fn test_queens_attackthe_king() {
        struct Testcase {
            queens: Vec<Vec<i32>>,
            king: Vec<i32>,
            expect: Vec<Vec<i32>>,
        }

        vec![
            Testcase {
                queens: vec2![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]],
                king: vec![0, 0],
                expect: vec2![[0, 1], [1, 0], [3, 3]],
            },
            Testcase {
                queens: vec2![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],
                king: vec![3, 3],
                expect: vec2![[2, 2], [3, 4], [4, 4]],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                Testcase {
                    queens,
                    king,
                    expect,
                },
            )| {
                use std::collections::HashSet;
                let actual = queens_attackthe_king(queens, king);

                let actual = actual.into_iter().collect::<HashSet<Vec<i32>>>();
                let expect = expect.into_iter().collect::<HashSet<Vec<i32>>>();

                assert_eq!(expect, actual, "case {} failed", idx);
            },
        )
    }

    #[test]
    fn test_check_valid_grid() {
        struct Testcase {
            grid: Vec<Vec<i32>>,
            expect: bool,
        }
        vec![
            Testcase {
                grid: vec2![
                    [0, 11, 16, 5, 20],
                    [17, 4, 19, 10, 15],
                    [12, 1, 8, 21, 6],
                    [3, 18, 23, 14, 9],
                    [24, 13, 2, 7, 22]
                ],
                expect: true,
            },
            Testcase {
                grid: vec2![[0, 3, 6], [5, 8, 1], [2, 7, 4]],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { grid, expect } = testcase;
            let actual = check_valid_grid(grid);
            assert_eq!(expect, actual, "case {} failed", idx);
        })
    }
}
