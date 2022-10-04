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

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec2;

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
