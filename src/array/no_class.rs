//! 暂时不知道怎么归类的数组题目
//!
//! ## 题目
//! * 简单
//!     * [1636. 按照频率将数组升序排序](frequency_sort)
//!     * [1672. 最富有客户的资产总量](maximum_wealth)
//!     * [2032. 至少在两个数组中出现的值](two_out_of_three)
//!     * [1779. 找到最近的有相同 X 或 Y 坐标的点](nearest_valid_point)
//! * 中等
//! * 困难
//!     * [827. 最大人工岛](largest_island)
//!     * [37. 解数独](solve_sudoku)
//!

/// [827. 最大人工岛](https://leetcode.cn/problems/making-a-large-island/)
pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};

    type AreaSize = i32;
    type IslandID = i32;
    type Coord = (usize, usize);

    fn dfs(
        id: IslandID,
        start: Coord,
        visited_map: &mut Vec<Vec<IslandID>>,
        zero: &mut Vec<Coord>,
        grid: &Vec<Vec<i32>>,
    ) -> AreaSize {
        let mut dequeue = VecDeque::new();
        dequeue.push_back(start);

        let mut ret: AreaSize = 0;
        while !dequeue.is_empty() {
            let (row, col) = dequeue.pop_front().unwrap();
            visited_map[row][col] = id;
            ret += 1;

            for (r, c) in [
                (row, col.checked_sub(1).unwrap_or(500)), // 左边
                (row.checked_sub(1).unwrap_or(500), col), // 上边
                (row, col + 1),                           // 右边
                (row + 1, col),                           // 下边
            ] {
                if let Some(line) = grid.get(r) {
                    if let Some(ele) = line.get(c) {
                        if visited_map[r][c] != 0 {
                            continue;
                        }
                        if *ele == 1 {
                            visited_map[r][c] = id;
                            dequeue.push_back((r, c));
                        } else {
                            visited_map[r][c] = -1;
                            zero.push((r, c));
                        }
                    }
                }
            }
        }
        ret
    }

    let n = grid.len();
    let mut visited_map: Vec<Vec<IslandID>> = vec![vec![0; n]; n];
    let mut empty_coord: Vec<Coord> = vec![];
    let mut area_size_map: HashMap<IslandID, AreaSize> = HashMap::new();

    let mut area_id = 1;
    for (row, line) in grid.iter().enumerate() {
        for (col, ele) in line.iter().enumerate() {
            if visited_map[row][col] != 0 {
                continue;
            }
            if *ele == 0 {
                visited_map[row][col] = -1;
                empty_coord.push((row, col));
                continue;
            }
            let size = dfs(
                area_id,
                (row, col),
                &mut visited_map,
                &mut empty_coord,
                &grid,
            );
            area_size_map.insert(area_id, size);
            area_id += 1;
        }
    }

    let mut max_area = area_size_map
        .values()
        .max_by(|a, b| a.cmp(b))
        .copied()
        .unwrap_or(0);

    for (row, col) in empty_coord {
        let mut area = 1;
        let mut visited_area = HashSet::new();
        for (r, c) in [
            (row, col.checked_sub(1).unwrap_or(500)), // 左边
            (row.checked_sub(1).unwrap_or(500), col), // 上边
            (row, col + 1),                           // 右边
            (row + 1, col),                           // 下边
        ] {
            if let Some(line) = grid.get(r) {
                if line.get(c).is_some() {
                    let id = visited_map[r][c];
                    if visited_area.insert(id) {
                        area += area_size_map.get(&id).copied().unwrap_or(0);
                    }
                }
            }
        }
        max_area = max_area.max(area);
    }
    max_area
}

/// [1636. 按照频率将数组升序排序](https://leetcode.cn/problems/sort-array-by-increasing-frequency/)
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut counter = {
        let mut tmp: HashMap<i32, usize> = HashMap::new();
        nums.iter().for_each(|num| {
            let entry = tmp.entry(*num).or_default();
            *entry += 1;
        });
        tmp.into_iter().collect::<Vec<(i32, usize)>>()
    };
    counter.sort_by(|a, b| {
        let cc = a.1.cmp(&b.1);
        match cc {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0).reverse(),
            _ => cc,
        }
    });

    let mut result = Vec::with_capacity(nums.len());
    for (k, v) in counter {
        result.extend(vec![k; v]);
    }

    result
}

/// [1672. 最富有客户的资产总量](https://leetcode.cn/problems/richest-customer-wealth/)
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .into_iter()
        .map(|acc| acc.into_iter().sum::<i32>())
        .max()
        .unwrap()
}

/// [2032. 至少在两个数组中出现的值](https://leetcode.cn/problems/two-out-of-three/)
pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut mask = [[0u8; 101]; 3];
    [nums1, nums2, nums3]
        .into_iter()
        .enumerate()
        .for_each(|(idx, nums)| {
            nums.into_iter().for_each(|num| {
                if mask[idx][num as usize] == 0 {
                    mask[idx][num as usize] += 1;
                }
            });
        });
    mask[0]
        .into_iter()
        .zip(mask[1])
        .zip(mask[2])
        .enumerate()
        .filter_map(|(idx, ((num1, num2), num3))| {
            if num1 + num2 + num3 >= 2 {
                Some(idx as i32)
            } else {
                None
            }
        })
        .collect()
}

/// [1779. 找到最近的有相同 X 或 Y 坐标的点](https://leetcode.cn/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/)
pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    points
        .into_iter()
        .enumerate()
        .rev()
        .filter_map(|(_idx, point)| {
            let (x0, y0) = (point[0], point[1]);
            if x0 == x || y0 == y {
                Some((_idx as i32, (x - x0).abs() + (y - y0).abs()))
            } else {
                None
            }
        })
        .min_by(|(idx0, dis0), (idx1, dis1)| dis0.cmp(dis1).then(idx0.cmp(idx1)))
        .unwrap_or((-1, 0))
        .0
}

/// [37. 解数独](https://leetcode.cn/problems/sudoku-solver/)
pub fn solve_sudoku(_board: &mut Vec<Vec<char>>) {
    todo!()
}

/// [857. 雇佣 K 名工人的最低成本](https://leetcode.cn/problems/minimum-cost-to-hire-k-workers)
///
/// 思路:
/// 1. 至少有一个员工是其最低工资要求
///     - 如果不是, 则说明整体工资可以再降
/// 2.
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    todo!()
}

/// [1491. 去掉最低工资和最高工资后的工资平均值](https://leetcode.cn/problems/average-salary-excluding-the-minimum-and-maximum-salary)
pub fn average(salary: Vec<i32>) -> f64 {
    let (mut min, mut max) = (i32::MAX, i32::MIN);
    let (mut total, mut count) = (0.0f64, 0.0f64);
    for one in salary {
        total += one as f64;
        count += 1.0f64;

        min = min.min(one);
        max = max.max(one);
    }

    total = total - min as f64 - max as f64;
    count = count - 2.0;

    total / count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_average() {
        struct Testcase {
            salary: Vec<i32>,
            expect: f64,
        }
        vec![
            Testcase {
                salary: vec![4000, 3000, 1000, 2000],
                expect: 2500.00000,
            },
            Testcase {
                salary: vec![1000, 2000, 3000],
                expect: 2000.0000,
            },
            Testcase {
                salary: vec![6000, 5000, 4000, 3000, 2000, 1000],
                expect: 3500.00000,
            },
            Testcase {
                salary: vec![8000, 9000, 2000, 3000, 6000, 1000],
                expect: 4750.00000,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, Testcase { salary, expect })| {
            let actual = average(salary);
            assert_eq!(expect, actual, "case {} failed", idx);
        })
    }

    #[test]
    fn test_mincost_to_hire_workers() {
        struct Testcase {
            quality: Vec<i32>,
            wage: Vec<i32>,
            k: i32,
            expect: f64,
        }

        vec![
            Testcase {
                quality: vec![10, 20, 5],
                wage: vec![70, 50, 30],
                k: 2,
                expect: 105.00000,
            },
            Testcase {
                quality: vec![3, 1, 10, 10, 1],
                wage: vec![4, 8, 2, 2, 7],
                k: 3,
                expect: 30.66667,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                Testcase {
                    quality,
                    wage,
                    k,
                    expect,
                },
            )| {
                let actual = mincost_to_hire_workers(quality, wage, k);
                assert_eq!(expect, actual, "case {} failed", idx);
            },
        )
    }

    #[test]
    #[ignore = "暂时不知道怎么解"]
    fn test_solve_sudoku() {
        struct Testcase {
            board: Vec<Vec<char>>,
            expect: Vec<Vec<char>>,
        }

        vec![Testcase {
            board: vec2![
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ],
            expect: vec2![
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ],
        }]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let mut board = testcase.board.clone();
            solve_sudoku(&mut board);
            assert_eq!(board, testcase.expect, "test failed at {}", idx);
        });
    }

    #[test]
    fn test_two_out_of_three() {
        struct Testcase {
            nums1: Vec<i32>,
            nums2: Vec<i32>,
            nums3: Vec<i32>,
            expect: Vec<i32>,
        }
        vec![
            Testcase {
                nums1: vec![1, 1, 3, 2],
                nums2: vec![2, 3],
                nums3: vec![3],
                expect: vec![3, 2],
            },
            Testcase {
                nums1: vec![3, 1],
                nums2: vec![2, 3],
                nums3: vec![1, 2],
                expect: vec![3, 2, 1],
            },
            Testcase {
                nums1: vec![1, 2, 2],
                nums2: vec![4, 3, 3],
                nums3: vec![5],
                expect: vec![],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase {
                nums1,
                nums2,
                nums3,
                mut expect,
            } = testcase;
            let mut actual = two_out_of_three(nums1, nums2, nums3);
            expect.sort();
            actual.sort();
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
    #[test]
    fn test_nearest_valid_point() {
        struct Testcase {
            x: i32,
            y: i32,
            points: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                x: 3,
                y: 4,
                points: vec2![[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]],
                expect: 2,
            },
            Testcase {
                x: 3,
                y: 4,
                points: vec2![[3, 4]],
                expect: 0,
            },
            Testcase {
                x: 3,
                y: 4,
                points: vec2![[2, 3]],
                expect: -1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase {
                x,
                y,
                points,
                expect,
            } = testcase;
            let actual = nearest_valid_point(x, y, points);
            assert_eq!(expect, actual, "case {} failed", idx);
        })
    }

    #[test]
    fn test_maximum_wealth() {
        struct Testcase {
            accounts: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                accounts: vec2![[1, 2, 3], [3, 2, 1]],
                expect: 6,
            },
            Testcase {
                accounts: vec2![[1, 5], [7, 3], [3, 5]],
                expect: 10,
            },
            Testcase {
                accounts: vec2![[2, 8, 7], [7, 1, 3], [1, 9, 5]],
                expect: 17,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { accounts, expect } = testcase;
            let actual = maximum_wealth(accounts);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_frequency_sort() {
        struct Testcase {
            nums: Vec<i32>,
            expect: Vec<i32>,
        }

        vec![
            Testcase {
                nums: vec![1, 1, 2, 2, 2, 3],
                expect: vec![3, 1, 1, 2, 2, 2],
            },
            Testcase {
                nums: vec![2, 3, 1, 3, 2],
                expect: vec![1, 3, 3, 2, 2],
            },
            Testcase {
                nums: vec![-1, 1, -6, 4, 5, -6, 1, 4, 1],
                expect: vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { nums, expect } = testcase;
            let actual = frequency_sort(nums);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_largest_island() {
        struct Testcase {
            grid: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                grid: vec![vec![1, 0], vec![0, 1]],
                expect: 3,
            },
            Testcase {
                grid: vec![vec![1, 1], vec![0, 1]],
                expect: 4,
            },
            Testcase {
                grid: vec![vec![1, 1], vec![1, 1]],
                expect: 4,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { grid, expect } = testcase;
            let acutal = largest_island(grid);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }
}
