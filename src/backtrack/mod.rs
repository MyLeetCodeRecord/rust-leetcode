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
    HashSet::<Vec<i32>>::from_iter(ret)
        .into_iter()
        .collect()
}

/// [842. 将数组拆分成斐波那契序列](https://leetcode.cn/problems/split-array-into-fibonacci-sequence/)
#[rustfmt::skip]
pub fn split_into_fibonacci(num: String) -> Vec<i32> {
    const MAX : i64 = 1 << 31;
    fn backtrack(
        ans: &mut Vec<i64>, num: &[u8],
        length: usize, idx: usize, 
        sum: i64, prev: i64 // 前两个数的加和, 前一个数
    ) -> bool {
        if idx == length {
            // 返回是否已经找到结果, 如果是, 可以提前终止, 剪枝4
            return ans.len() >= 3;
        }
        // 从 idx 开始, 枚举所有可能的数
        let mut curr = 0;
        for i in idx..length {
            // 自身为0是可以的, 但是下一个数不能以0开头
            // 因此跳过后续枚举, 剪枝1
            if i > idx && num[idx] == b'0' {
                break;
            }
            curr = curr * 10 + (num[i] - b'0') as i64;
            if curr >= MAX {
                // 题目要求每个数, 大于0, 小于 2**31
                // 因此如果发现当前的数不符合, 则整个尝试无效, 剪枝2
                break;
            }
            if ans.len() >= 2 {
                // 斐波那契序列, 至少有3个, 因此在已经有2个数时,
                // 第三个不能比前两个的加和小, 如果小, 也不用再套一层, 直接再吃一位
                // 如果大, 那就必然组不成序列, 整个尝试无效, 剪枝3
                if curr < sum {
                    continue;
                } else if curr > sum {
                    break;
                }
            }
            ans.push(curr);
            if backtrack(ans, num, length, i+1, prev+curr, curr) {
                return true;
            }
            ans.pop();
        }
        false
    }

    let mut ans = vec![];
    if backtrack(&mut ans, num.as_bytes(), num.as_bytes().len(), 0, 0, 0) {
        ans.into_iter().map(|n| n as i32).collect()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_fibonacci() {
        struct TestCase {
            num: &'static str,
            expect: Vec<i32>,
        }

        vec![
            TestCase {
                num: "1101111",
                expect: vec![11, 0, 11, 11],
            },
            TestCase {
                num: "112358130",
                expect: vec![],
            },
            TestCase {
                num: "0123",
                expect: vec![],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { num, expect } = testcase;
            let actual = split_into_fibonacci(num.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_find_subsequences() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: &'static [&'static [i32]],
        }

        [TestCase {
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
            }]
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
