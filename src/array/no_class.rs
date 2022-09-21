//! [827. 最大人工岛](largest_island)

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
                if let Some(_) = line.get(c) {
                    let id = visited_map[r][c];
                    if visited_area.insert(id) {
                        area = area + area_size_map.get(&id).copied().unwrap_or(0);
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
#[cfg(test)]
mod tests {
    use super::*;

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
