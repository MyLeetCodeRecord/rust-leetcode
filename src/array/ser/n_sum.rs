//! n数之和
//!
//! 涉及点比较多:
//! * 哈希
//! * 排序
//! * 双指针
//!
//! * [1. 两数之和](two_sum)
//! * [15. 三数之和](three_sum)
//! * [18. 四数之和](four_sum)
//! * [454. 四数相加 II](four_sum_count)

/// [1. 两数之和](https://leetcode.cn/problems/two-sum/)
/// 思路一:
/// 可以先排序, 然后用左右指针
/// ```
/// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
///     let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
///     nums.sort_by(|a, b| a.1.cmp(&b.1));
///     let (mut left, mut right) = (0, nums.len() - 1);
///     while left < right {
///         let l = nums.get(left).unwrap();
///         let r = nums.get(right).unwrap();
///         let curr = l.1 + r.1;
///         if curr == target {
///             return vec![l.0 as i32, r.0 as i32];
///         } else if curr > target {
///             right -= 1;
///         } else {
///             left += 1;
///         }
///     }
///     unreachable!()
/// }
/// ```
/// 由于是返回原始的位置,因此排序前, 需要记录原来的索引信息. 存储O(N)
///
/// 思路二:
/// hash加速查找
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut cache: HashMap<i32, usize> = HashMap::new();
    for (idx, &num) in nums.iter().enumerate() {
        if let Some(other) = cache.get(&(target - num)) {
            return vec![*other as i32, idx as i32];
        }
        cache.insert(num, idx);
    }
    unreachable!()
}

/// [15. 三数之和](https://leetcode.cn/problems/3sum/)
///
/// 解法1:
/// 思路: 降维, 转化为寻找两数之和
/// ```
/// pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
///     if nums.len()< 3{
///         return vec![];
///     }
///     use std::collections::HashMap;
///     use std::collections::HashSet;
///     use std::iter::FromIterator;
///     fn two_sum(nums: &[i32], target: i32, start: usize) -> Vec<Vec<usize>>{
///         if nums.len() - start < 2 {
///             return vec![];
///         }
///         let mut result = vec![];
///         let mut cache: HashMap<i32, Vec<usize>> = HashMap::new();
///         for (idx, &num) in nums.iter().enumerate().skip(start){
///             if let Some(other) = cache.get(&(target-num)){
///                 other.iter().for_each(|old|{
///                     result.push(vec![*old, idx]);
///                 });
///             }
///             cache.entry(num).or_insert(vec![]).push(idx);
///         }
///         return result;
///     }
///     let mut result = vec![];
///     for i in 0..nums.len(){
///         let target = nums.get(i).copied().unwrap();
///         let part = two_sum(&nums, 0-target, i+1);
///         if !part.is_empty(){
///             for p in part.into_iter(){
///                 let mut tmp = Vec::with_capacity(3);
///                 tmp.push(target);
///                 for idx in p{
///                     tmp.push(nums.get(idx).copied().unwrap());
///                 }
///                 tmp.sort();
///                 result.push(tmp);
///             }
///         }
///     }
///     let tmp:HashSet<Vec<i32>> = HashSet::from_iter(result.into_iter());
///     tmp.into_iter().collect()
/// }
/// ```
///
/// 也可不用存全量的组合情况, 在中间就用`HashSet`, 不过还是会**超时**.
/// 问题在于 **去重** 的方式问题.
/// 如果原始数组中有大量重复数据, 这样的组合重复会暴增, 最终超时
/// 由于最终重复的定义是数值是否相等, 而非位置是非相同
/// 因此可以先预处理, 比如排序后把重复的数据合并.
///
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];

    for first in 0..nums.len() {
        if first > 0 && nums[first] == nums[first - 1] {
            continue;
        }

        let mut third = nums.len() - 1;
        let target = 0 - nums[first];
        for second in first + 1..nums.len() {
            if second > first + 1 && nums[second] == nums[second - 1] {
                continue;
            }
            while second < third && nums[second] + nums[third] > target {
                third -= 1;
            }
            if second == third {
                break;
            }
            if nums[second] + nums[third] == target {
                result.push(vec![nums[first], nums[second], nums[third]]);
            }
        }
    }
    result
}

/// [18. 四数之和](https://leetcode.cn/problems/4sum/)
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    let mut nums = nums.into_iter().map(|num| num as i64).collect::<Vec<i64>>();
    nums.sort();
    let length = nums.len();
    let target = target as i64;

    let mut result = vec![];
    for a in 0..length - 3 {
        // 留3个位置
        if a > 0 && nums[a] == nums[a - 1] {
            continue; // 去重1
        }
        if nums[a] + nums[a + 1] + nums[a + 2] + nums[a + 3] > target {
            continue; // 剪枝1
        }
        if nums[a] + nums[length - 1] + nums[length - 2] + nums[length - 3] < target {
            continue; // 剪枝2
        }
        for b in (a + 1)..(length - 2) {
            // 留两个位置
            if b > a + 1 && nums[b] == nums[b - 1] {
                continue; // 去重2
            }
            if nums[a] + nums[b] + nums[b + 1] + nums[b + 2] > target {
                continue; // 剪枝3
            }
            if nums[a] + nums[b] + nums[length - 1] + nums[length - 2] < target {
                continue; // 剪枝4
            }
            let (mut c, mut d) = (b + 1, length - 1);
            while c < d {
                let sum = nums[a] + nums[b] + nums[c] + nums[d];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![
                            nums[a] as i32,
                            nums[b] as i32,
                            nums[c] as i32,
                            nums[d] as i32,
                        ]);
                        while c < d && nums[c] == nums[c + 1] {
                            c += 1;
                        }
                        c += 1;
                        while c < d && nums[d] == nums[d - 1] {
                            d -= 1;
                        }
                        d -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        d -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        c += 1;
                    }
                }
            }
        }
    }
    result
}

/// [454. 四数相加 II](https://leetcode.cn/problems/4sum-ii/)
pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut count_ab = HashMap::new();
    for num1 in nums1 {
        for num2 in nums2.iter() {
            *count_ab.entry(num1 + *num2).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for num3 in nums3 {
        for num4 in nums4.iter() {
            ans += count_ab.get(&(0 - num3 - *num4)).copied().unwrap_or(0);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_count() {
        struct TestCase {
            name: &'static str,
            nums1: &'static [i32],
            nums2: &'static [i32],
            nums3: &'static [i32],
            nums4: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic",
                nums1: &[1, 2],
                nums2: &[-2, -1],
                nums3: &[-1, 2],
                nums4: &[0, 2],
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                nums1: &[0],
                nums2: &[0],
                nums3: &[0],
                nums4: &[0],
                expect: 1,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = four_sum_count(
                testcase.nums1.to_vec(),
                testcase.nums2.to_vec(),
                testcase.nums3.to_vec(),
                testcase.nums4.to_vec(),
            );
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_four_sum() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            target: i32,
            expect: &'static [&'static [i32]],
        }

        [TestCase {
                name: "basic",
                nums: &[1, 0, -1, 0, -2, 2],
                target: 0,
                expect: &[&[-2, -1, 1, 2], &[-2, 0, 0, 2], &[-1, 0, 0, 1]],
            },
            TestCase {
                name: "basic 2",
                nums: &[2, 2, 2, 2, 2],
                target: 8,
                expect: &[&[2, 2, 2, 2]],
            },
            TestCase {
                name: "fix 1",
                nums: &[0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000],
                target: 1000000000,
                expect: &[&[0, 0, 0, 1000000000]],
            }]
        .iter()
        .for_each(|testcase| {
            let mut actual = four_sum(testcase.nums.to_vec(), testcase.target);
            let mut expect = testcase
                .expect
                .iter()
                .map(|p| p.to_vec())
                .collect::<Vec<Vec<i32>>>();

            actual.iter_mut().for_each(|l| l.sort());
            actual.sort();
            expect.iter_mut().for_each(|l| l.sort());
            expect.sort();

            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_three_sum() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: &'static [&'static [i32]],
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[-1, 0, 1, 2, -1, -4],
                expect: &[&[-1, -1, 2], &[-1, 0, 1]],
            },
            TestCase {
                name: "basic 2",
                nums: &[],
                expect: &[],
            },
            TestCase {
                name: "basic 3",
                nums: &[0],
                expect: &[],
            },
            TestCase {
                name: "fix 1",
                nums: &[1, 2, -2, -1],
                expect: &[],
            },
            TestCase {
                name: "fix 2",
                nums: &[0, 0, 0, 0],
                expect: &[&[0, 0, 0]],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let mut actual = three_sum(testcase.nums.to_vec());
            let mut expect = testcase
                .expect
                .iter()
                .map(|p| p.to_vec())
                .collect::<Vec<Vec<i32>>>();

            actual.iter_mut().for_each(|l| l.sort());
            actual.sort();
            expect.iter_mut().for_each(|l| l.sort());
            expect.sort();

            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_two_sum() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            target: i32,
            expect: &'static [i32],
        }

        [TestCase {
                name: "basic",
                nums: &[2, 7, 11, 15],
                target: 9,
                expect: &[0, 1],
            },
            TestCase {
                name: "basic 2",
                nums: &[3, 2, 4],
                target: 6,
                expect: &[1, 2],
            },
            TestCase {
                name: "basic 3",
                nums: &[3, 3],
                target: 6,
                expect: &[0, 1],
            }]
        .iter()
        .for_each(|testcase| {
            let mut actual = two_sum(testcase.nums.to_vec(), testcase.target);
            let mut expect = testcase.expect.to_vec();
            actual.sort();
            expect.sort();

            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }
}
