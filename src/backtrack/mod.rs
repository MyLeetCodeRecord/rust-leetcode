//! 回溯
//! 题目：
//! * [491. 递增子序列](find_subsequences)

/// [491. 递增子序列](https://leetcode.cn/problems/increasing-subsequences/)
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn backtrace(nums: &[i32], ret: &mut Vec<Vec<i32>>, ser: &mut Vec<i32>, curr: usize) {
        if ser.len() >= 2 {
            ret.push(ser.clone());
        }
        if curr == nums.len() {
            // 终止条件
            return;
        }

        for i in curr..nums.len() {
            // 如果比结尾大， 扩展一个
            let last = ser.last().copied().unwrap_or(-101);
            let c = nums.get(i).copied().unwrap();
            if c >= last {
                ser.push(c);
            }
            backtrace(nums, ret, ser, i + 1);
            if c >= last {
                ser.pop();
            }
        }
    }

    let mut ret = vec![];
    let mut ser = vec![];
    backtrace(&nums, &mut ret, &mut ser, 0);
    HashSet::<Vec<i32>>::from_iter(ret.into_iter())
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subsequences() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: &'static [&'static [i32]],
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[4, 6, 7, 7],
                expect: &[
                    &[4, 6],
                    &[4, 6, 7],
                    &[4, 6, 7, 7],
                    &[4, 7],
                    &[4, 7, 7],
                    &[6, 7],
                    &[6, 7, 7],
                    &[7, 7],
                ],
            },
            TestCase {
                name: "basic 2",
                nums: &[4, 4, 3, 2, 1],
                expect: &[&[4, 4]],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let mut actual = find_subsequences(testcase.nums.to_vec());
            actual.sort();
            let mut expect: Vec<Vec<i32>> = testcase.expect.iter().map(|x| x.to_vec()).collect();
            expect.sort();
            assert_eq!(expect, actual, "{} failed", testcase.name);
        })
    }
}
