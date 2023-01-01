//! 贪心
//!
//! * 特点: 选择每一阶段的局部最优，从而达到全局最优
//! * 什么时候用:
//!     * 没固定套路
//!     * 只能先正向看能否用局部最优, 推出全局最优
//!     * 再找找有没有不成功的反例
//!     * 最后看看能不能AC
//! ## 题目
//! * 简单
//!     * [455. 分发饼干](find_content_children)
//!     * [53. 最大子数组和](max_sub_array)
//!     * [942. 增减字符串匹配](di_string_match)
//!     *  [2027. 转换字符串的最少操作次数](minimum_moves)
//! * 中等
//!     * [376. 摆动序列](wiggle_max_length)
//!     * [55. 跳跃游戏](can_jump)
//!     * [646. 最长数对链](find_longest_chain)
//!     * [300. 最长递增子序列](length_of_lis)
//!

/// [455. 分发饼干](https://leetcode-cn.com/problems/assign-cookies/)
///
/// 局部最优就是大饼干喂给胃口大的，充分利用饼干尺寸喂饱一个
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let (mut g, mut s) = (g, s);
    g.sort();
    s.sort();
    let mut cnt = 0;
    let (mut gi, mut si) = (0, 0);
    while gi < g.len() && si < s.len() {
        let gg = g.get(gi).copied().unwrap();
        let ss = s.get(si).copied().unwrap();
        if ss >= gg {
            cnt += 1;
            gi += 1;
            si += 1;
        } else {
            si += 1;
        }
    }
    cnt
}

/// [376. 摆动序列](https://leetcode-cn.com/problems/wiggle-subsequence/)
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let mut diff = nums[1] - nums[0];
    let mut ret = {
        if diff == 0 {
            1
        } else {
            2
        }
    };

    for i in 2..nums.len() {
        let tmp = nums[i] - nums[i - 1];
        if (tmp > 0 && diff <= 0) || (tmp < 0 && diff >= 0) {
            ret += 1;
            diff = tmp;
        }
    }
    ret
}

/// [53. 最大子数组和](https://leetcode-cn.com/problems/maximum-subarray/)
///
/// 思路一: dp
/// 滚动 以某个数结尾(也可能只有这个数自己)的最大值, 直到最后
/// ```
/// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
///     if nums.is_empty(){
///         return 0;
///     }
///     let mut pre = 0;
///     let mut ans = nums[0];
///     for num in nums{
///         pre = std::cmp::max(num, pre+num);
///         ans = std::cmp::max(ans, pre);
///     }
///     ans
/// }
/// ```
/// 思路二: 贪心
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut result = i32::MIN;
    let mut curr = 0;
    for num in nums {
        curr += num;
        if curr > result {
            result = curr;
        }
        if curr < 0 {
            curr = 0;
        }
    }
    result
}

/// [942. 增减字符串匹配](https://leetcode.cn/problems/di-string-match/)
///
/// 思路:
/// * 遇到`'I'`, 就填充一个最小的, 这样不管后面是多少, 都能符合
/// * 遇到`'D'`, 就填充一个最大的, 这样不管后面是多少, 都能符合
/// * 同时问题规模缩减, 重复上面操作
/// * 最后补一个最小或剩余的最大即可
pub fn di_string_match(s: String) -> Vec<i32> {
    let (mut min, mut max) = (0, s.len() as i32);
    let mut ans = vec![];
    for c in s.chars() {
        if c == 'I' {
            ans.push(min);
            min += 1;
        } else if c == 'D' {
            ans.push(max);
            max -= 1;
        }
    }
    ans.push(min);
    ans
}

/// [55. 跳跃游戏](https://leetcode.cn/problems/jump-game/)
///
/// 从 nums[i] 出发最远到达的距离：
/// `x`在`[nums[i], nums[i]+i]`内的 `nums[x]+x`的最大值
///
/// 这样就扩展了最远距离， 但不是下一轮的起点
///
/// 精简版：
/// ```rust
/// pub fn can_jump(nums: Vec<i32>) -> bool {
///     let mut right_most = 0;
///     for (i, n) in nums.iter().enumerate() {
///         if i <= right_most {
///             right_most = right_most.max(i + *n as usize);
///             if right_most >= nums.len() - 1 { // 终点是最后一个元素, 因此 等于 nums.len() - 1 也可
///                 return true;
///             }
///         }
///         // 如果出现了i > right_most 说明出现了断层
///     }
///     false
/// }
/// ```
///
pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return true;
    }
    let length = nums.len() as i32;

    let mut start = 0;
    let mut end = 0;
    loop {
        let mut step = nums.get(start as usize).copied().unwrap();
        end = end.max(start + step);
        if end >= length - 1 {
            return true;
        }
        if start == end {
            return false;
        }

        let mut tmp = end;
        for i in start..end {
            step = nums.get(i as usize).copied().unwrap();
            tmp = tmp.max(i + step);
        }
        start = end;
        end = tmp;

        if start >= length - 1 {
            return true;
        }
    }
}

/// [646. 最长数对链](https://leetcode.cn/problems/maximum-length-of-pair-chain/)
///
/// 题目保证 **第一个数字总是比第二个数字小**
/// 因此按照第二个数字排序
///
/// 之后取第二个数字最小的， 这样链路可以尽可能的长
///
/// [DP解法](crate::dp::ser::longest_sub::find_longest_chain)
///
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut curr = i32::MIN;
    let mut cnt = 0;

    for pair in pairs {
        if pair[0] > curr {
            curr = pair[1];
            cnt += 1;
        }
    }
    cnt
}

/// [435. 无重叠区间](https://leetcode.cn/problems/non-overlapping-intervals/)
///
/// [DP解法](crate::dp::ser::longest_sub::erase_overlap_intervals)
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut pairs = intervals;
    pairs.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut curr = i32::MIN;
    let mut cnt = 0;

    for pair in pairs.iter() {
        if pair[0] >= curr {
            curr = pair[1];
            cnt += 1;
        }
    }
    pairs.len() as i32 - cnt
}

/// [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/)
///
/// [DP解法](crate::dp::ser::longest_sub::length_of_lis)
///
/// 序列增长的越慢， 最终的长度可能越长
///
///  d[i], 表示长度为 i 的最长上升子序列的末尾元素的最小值
///
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut d = Vec::with_capacity(nums.len() + 1);
    for &num in nums.iter() {
        let last = d.last().copied().unwrap_or(i32::MIN);
        // 如果比结尾大，无疑需要延展一个
        if num > last {
            d.push(num);
            continue;
        }
        // 如果不比结尾大， 找个位置安置, 有可能是替换掉了结尾， 这样也就达到了 减速 的效果
        // 如果不是替换的结尾, 不影响后续延展
        // 替换后会影响后续判断过程
        // 不能直接使用 binary_search， 存在相等元素时， 只能替换最早的那个
        let (mut left, mut right) = (1, d.len());
        while left <= right {
            let mid = left + (right - left) / 2;
            if d[mid - 1] >= num {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        d[left - 1] = num;
    }
    d.len() as i32
}

/// [670. 最大交换](https://leetcode.cn/problems/maximum-swap/)
///
/// 思路: 找到第一个比9小的, 尝试之后有没有比它大的, 找出其中最大的, 交换过来
///
/// 且需要取最低位的
///
pub fn maximum_swap(num: i32) -> i32 {
    let mut num = num;
    let mut part = vec![];
    while num > 0 {
        part.push(num % 10);
        num = num / 10;
    }

    'SWAP: for j in (1..part.len()).rev() {
        let a = part.get(j).copied().unwrap();
        if a < 9 {
            let m = part
                .iter()
                .copied()
                .enumerate()
                .take(j)
                .max_by(|a, b| {
                    if a.1 != b.1 {
                        a.1.cmp(&b.1)
                    } else {
                        if a.0 < b.0 {
                            std::cmp::Ordering::Greater // 小序号优先
                        } else {
                            std::cmp::Ordering::Less
                        }
                    }
                })
                .unwrap();

            if m.1 > a {
                part.swap(j, m.0);
                break 'SWAP;
            }
        }
    }

    let mut result = 0;
    for n in part.into_iter().rev() {
        result = result * 10 + n
    }
    result
}

/// [846. 一手顺子](https://leetcode.cn/problems/hand-of-straights/)
///
/// [Hash解法](crate::mhash::no_class::is_n_straight_hand)
pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    use std::collections::HashMap;
    if (hand.len() as i32) % group_size != 0 {
        // 张数不对
        return false;
    }
    let mut hand = hand;
    hand.sort();

    let mut counter = HashMap::new();
    for &card in hand.iter() {
        *counter.entry(card).or_insert(0) += 1;
    }

    for i in 0..hand.len() {
        let start = hand[i];
        let e = counter.entry(start).or_default();
        if *e == 0 {
            // 跳过, 下一个
            continue;
        }
        *e -= 1;
        for i in 1..group_size {
            // 开始假设枚举
            let e = counter.entry(start + i).or_default();
            if *e == 0 {
                return false;
            }
            *e -= 1;
        }
    }
    true
}

/// [2027. 转换字符串的最少操作次数](https://leetcode.cn/problems/minimum-moves-to-convert-string/)
/// 
/// 只要遇到 X, 三个以内不论什么, 都是至少转换一次
/// 既然不管是啥都得转换, 那就直接跳过
pub fn minimum_moves(s: String) -> i32 {
    let mut sum = 0;

    let bs = s.as_bytes();
    let mut idx = 0;
    while idx < bs.len(){
        if bs[idx] == b'O'{
            idx += 1;
            continue;
        }
        sum += 1;
        idx += 3;
    }

   sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_moves() {
        struct TestCase {
            s: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                s: "XXX",
                expect: 1,
            },
            TestCase {
                s: "XXOX",
                expect: 2,
            },
            TestCase {
                s: "OOOO",
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = minimum_moves(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_is_n_straight_hand() {
        struct TestCase {
            hand: Vec<i32>,
            group_size: i32,
            expect: bool,
        }

        vec![
            TestCase {
                hand: vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
                group_size: 3,
                expect: true,
            },
            TestCase {
                hand: vec![1, 2, 3, 4, 5],
                group_size: 4,
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                hand,
                group_size,
                expect,
            } = testcase;
            let actual = is_n_straight_hand(hand, group_size);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_maximum_swap() {
        struct TestCase {
            name: &'static str,
            num: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                num: 2736,
                expect: 7236,
            },
            TestCase {
                name: "basic 2",
                num: 9973,
                expect: 9973,
            },
            TestCase {
                name: "fix 1",
                num: 98368,
                expect: 98863,
            },
            TestCase {
                name: "fix 2",
                num: 1993,
                expect: 9913,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = maximum_swap(testcase.num);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_length_of_lis() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[10, 9, 2, 5, 3, 7, 101, 18],
                expect: 4,
            },
            TestCase {
                name: "basic 2",
                nums: &[0, 1, 0, 3, 2, 3],
                expect: 4,
            },
            TestCase {
                name: "basic 3",
                nums: &[7, 7, 7, 7, 7, 7, 7],
                expect: 1,
            },
            TestCase {
                name: "fix 1",
                nums: &[10, 9, 2, 5, 3, 4],
                expect: 3,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = length_of_lis(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_erase_overlap_intervals() {
        struct TestCase {
            name: &'static str,
            intervals: &'static [[i32; 2]],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                intervals: &[[1, 2], [2, 3], [3, 4], [1, 3]],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                intervals: &[[1, 2], [1, 2], [1, 2]],
                expect: 2,
            },
            TestCase {
                name: "basic 1",
                intervals: &[[1, 2], [2, 3]],
                expect: 0,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let intervals = testcase.intervals.iter().map(|p| p.to_vec()).collect();
            let actual = erase_overlap_intervals(intervals);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_longest_chain() {
        struct TestCase {
            name: &'static str,
            pair: &'static [(i32, i32)],
            expect: i32,
        }

        vec![TestCase {
            name: "basic 1",
            pair: &[(1, 2), (2, 3), (3, 4)],
            expect: 2,
        }]
        .iter()
        .for_each(|testcase| {
            let pair = testcase.pair.iter().map(|p| vec![p.0, p.1]).collect();
            let actual = find_longest_chain(pair);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_can_jump() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: bool,
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[2, 3, 1, 1, 4],
                expect: true,
            },
            TestCase {
                name: "basic 2",
                nums: &[3, 2, 1, 0, 4],
                expect: false,
            },
            TestCase {
                name: "fix 1",
                nums: &[0],
                expect: true,
            },
            TestCase {
                name: "fix 2",
                nums: &[0, 2, 3],
                expect: false,
            },
            TestCase {
                name: "basic 3",
                nums: &[2, 0, 1],
                expect: true,
            },
            TestCase {
                name: "fix 3",
                nums: &[1, 0, 1, 0],
                expect: false,
            },
            TestCase {
                name: "fix 4",
                nums: &[3, 0, 8, 2, 0, 0, 1],
                expect: true,
            },
            TestCase {
                name: "fix 5",
                nums: &[5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0],
                expect: true,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = can_jump(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_di_string_match() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic",
                s: "IDID",
                expect: &[0, 4, 1, 3, 2],
            },
            TestCase {
                name: "basic 2",
                s: "III",
                expect: &[0, 1, 2, 3],
            },
            TestCase {
                name: "basic 3",
                s: "DDI",
                expect: &[3, 2, 0, 1],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = di_string_match(testcase.s.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_max_sub_array() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[-2, 1, -3, 4, -1, 2, 1, -5, 4],
                expect: 6,
            },
            TestCase {
                name: "basic 2",
                nums: &[1],
                expect: 1,
            },
            TestCase {
                name: "basic 3",
                nums: &[5, 4, -1, 7, 8],
                expect: 23,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = max_sub_array(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_wiggle_max_length() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[1, 7, 4, 9, 2, 5],
                expect: 6,
            },
            TestCase {
                name: "basic 2",
                nums: &[1, 17, 5, 10, 13, 15, 10, 5, 16, 8],
                expect: 7,
            },
            TestCase {
                name: "basic 3",
                nums: &[1, 2, 3, 4, 5, 6, 7, 8, 9],
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = wiggle_max_length(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_content_children() {
        struct TestCase {
            name: &'static str,
            g: &'static [i32],
            s: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                g: &[1, 2, 3],
                s: &[1, 1],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                g: &[1, 2],
                s: &[1, 2, 3],
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = find_content_children(testcase.g.to_vec(), testcase.s.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
