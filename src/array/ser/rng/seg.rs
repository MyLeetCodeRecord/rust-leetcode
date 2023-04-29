//! # 线段树
//! > <https://oi-wiki.org/ds/seg/>
//!
//! * [327. 区间和的个数](count_range_sum)
//! * [715. Range 模块](https://leetcode.cn/problems/range-module/)

/// [327. 区间和的个数](https://leetcode.cn/problems/count-of-range-sum/)
pub fn count_range_sum(_nums: Vec<i32>, _lower: i32, _upper: i32) -> i32 {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_count_range_sum() {
//         struct Testcase {
//             nums: Vec<i32>,
//             lower: i32,
//             upper: i32,
//             expect: i32,
//         }

//         vec![
//             Testcase {
//                 nums: vec![-2, 5, -1],
//                 lower: -2,
//                 upper: 2,
//                 expect: 3,
//             },
//             Testcase {
//                 nums: vec![0],
//                 lower: 0,
//                 upper: 0,
//                 expect: 1,
//             },
//         ]
//         .into_iter()
//         .enumerate()
//         .for_each(|(idx, testcae)| {
//             let Testcase {
//                 nums,
//                 lower,
//                 upper,
//                 expect,
//             } = testcae;
//             let actual = count_range_sum(nums, lower, upper);
//             assert_eq!(expect, actual, "case {} failed", idx);
//         });
//     }
// }
