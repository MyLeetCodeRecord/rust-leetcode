/// [845. 数组中的最长山脉](https://leetcode.cn/problems/longest-mountain-in-array/)
/// * 阶段1: [滑动窗口](crate::array::ser::windows::longest_mountain)
/// * 阶段2: [DP 解法](crate::dp::no_class::longest_mountain)
///     * 如果序列是 "上山"(严格递增), 那必然不是 "下山"(严格递减)
///     * 可以将这个判定结果存下来
/// * 阶段3: [双指针](crate::array::ser::two_pointers::longest_mountain)
pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    if arr.len() < 3 {
        return 0;
    }
    let mut lefts = vec![0; arr.len()];
    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            lefts[i] = lefts[i - 1] + 1; // 上山路径加1
        }
    }
    let mut rights = vec![0; arr.len()];
    for i in (0..arr.len() - 1).rev() {
        if arr[i] > arr[i + 1] {
            rights[i] = rights[i + 1] + 1; // 下山路径加1
        }
    }
    let mut ans = 0;
    for i in 0..arr.len() {
        if lefts[i] > 0 && rights[i] > 0 {
            ans = ans.max(lefts[i] + rights[i] + 1);
        }
    }
    ans
}

/// [849. 到最近的人的最大距离](https://leetcode.cn/problems/maximize-distance-to-closest-person/)
///
/// 1. 阶段1: [dp解法](crate::dp::no_class::max_dist_to_closest)
///     * 从左, 从右各扫描一次, 记录当前位置到 "前" 一个有人的位置的距离, 最后取 _左右_ 距离中的最小值, 即为这个位置到最近人的距离
///     * 两端的特殊处理:
///         * 从左到右, 如果第一个位置为0, 则其相对左边的距离, 应为无限大
///         * 可以记为n, 这样取最小值时可以用右边的距离值,
///         * 同理从右到左
///     * 如果位置已经被占用, 则使用-1标识
/// 2. 阶段2: [双指针解法](crate::array::ser::two_pointers::max_dist_to_closest)
///
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len() as i32;

    let (mut left, mut right) = (vec![-1; seats.len()], vec![-1; seats.len()]);
    let (mut from_left, mut from_right) = (-n, n + n - 1);

    for i in 0..seats.len() {
        if seats[i] == 1 {
            // 已经被占住了
            left[i] = -1;
            from_left = i as i32; // 更新左边的人的位置
        } else {
            left[i] = (i as i32) - from_left;
        }
    }
    for i in (0..seats.len()).rev() {
        if seats[i] == 1 {
            right[i] = -1;
            from_right = i as i32;
        } else {
            right[i] = from_right - (i as i32);
        }
    }

    left.into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.min(b))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_dist_to_closest() {
        struct TestCase {
            seats: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                seats: vec![1, 0, 0, 0, 1, 0, 1],
                expect: 2,
            },
            TestCase {
                seats: vec![1, 0, 0, 0],
                expect: 3,
            },
            TestCase {
                seats: vec![0, 1],
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { seats, expect } = testcase;
            let actual = max_dist_to_closest(seats);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_longest_mountain() {
        struct TestCase {
            arr: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                arr: vec![2, 1, 4, 7, 3, 2, 5],
                expect: 5,
            },
            TestCase {
                arr: vec![2, 2, 2],
                expect: 0,
            },
            TestCase {
                arr: vec![0],
                expect: 0,
            },
            TestCase {
                arr: vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0],
                expect: 11,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { arr, expect } = testcase;
            let actual = longest_mountain(arr);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
