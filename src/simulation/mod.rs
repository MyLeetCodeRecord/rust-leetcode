//! 模拟类题目
//! 
//! 题目
//! * 简单
//! * 中等
//! * 困难
//! 


/// [2596. Check Knight Tour Configuration](https://leetcode.cn/problems/check-knight-tour-configuration/description)
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


#[cfg(test)]
mod tests{
    use super::*;
    use crate::vec2;

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