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
//! * 困难
//!     * [630. 课程表 III](schedule_course)
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
/// 从 `nums[i]` 出发最远到达的距离：
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
///  `d[i]`, 表示长度为 `i` 的最长上升子序列的末尾元素的最小值
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
        num /= 10;
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
                    } else if a.0 < b.0 {
                        std::cmp::Ordering::Greater // 小序号优先
                    } else {
                        std::cmp::Ordering::Less
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
    while idx < bs.len() {
        if bs[idx] == b'O' {
            idx += 1;
            continue;
        }
        sum += 1;
        idx += 3;
    }

    sum
}

/// [853. 车队](https://leetcode.cn/problems/car-fleet/)
///
/// 可以追上去，并与前车 以相同的速度 紧接着行驶
///
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let target = target as f64;
    let mut ps = position
        .into_iter()
        .zip(speed)
        .collect::<Vec<(i32, i32)>>();
    ps.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    let mut cursor = 0;
    let mut ans = 0;
    while cursor < ps.len() {
        let time = {
            // f64精度够吗?
            let div = target - ps[cursor].0 as f64;
            let step = ps[cursor].1 as f64;
            div / step
        };
        let mut end = cursor + 1;
        while end < ps.len() {
            let tmp = ps[end];
            if tmp.0 as f64 + tmp.1 as f64 * time >= target {
                end += 1;
            } else {
                break;
            }
        }
        ans += 1;
        cursor = end;
    }
    ans
}

/// [860. 柠檬水找零](https://leetcode.cn/problems/lemonade-change/)
///
/// - 只有10, 20 需要找零, 5不用找零
/// - 能先用大额就用大额
/// - 10可以用于20的找零, 20不能用于找零, 因此20不用存
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut changes = [0; 2]; // 5, 10
    for &bill in bills.iter() {
        match bill {
            5 => {
                changes[0] += 1;
            }
            10 => {
                if changes[0] == 0 {
                    return false;
                }
                changes[0] -= 1;
                changes[1] += 1;
            }
            20 => {
                if changes[1] > 0 && changes[0] > 0 {
                    changes[1] -= 1;
                    changes[0] -= 1;
                } else if changes[0] >= 3 {
                    changes[0] -= 3;
                } else {
                    return false;
                }
                // 20参与不了找零, 因此可以不用存计数
                // changes[2] += 1;
            }
            _ => unreachable!(),
        }
    }
    true
}

/// [861. 翻转矩阵后的得分](https://leetcode.cn/problems/score-after-flipping-matrix/)
///
/// 1. 由于二进制高位在左边, 因此每一行的第一个数必须是1
/// 2. 之后每列的1的个数必须大于0的个数, 否则翻转
pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut ans = 0;
    for row in grid.iter_mut() {
        if row[0] == 0 {
            // 翻转
            row.iter_mut().for_each(|x| *x ^= 1);
        }
    }
    for col in 0..grid[0].len() {
        let mut count = 0;
        for row in 0..grid.len() {
            if grid[row][col] == 1 {
                count += 1;
            }
        }
        if count <= grid.len() / 2 {
            // 翻转
            for row in 0..grid.len() {
                grid[row][col] ^= 1;
            }
        }
    }
    for row in grid.iter() {
        let mut sum = 0;
        for &x in row.iter() {
            sum = sum << 1 | x;
        }
        ans += sum;
    }
    ans
}

/// [630. Course Schedule III](https://leetcode.cn/problems/course-schedule-iii/)
/// 
/// 思路:
/// 1. 先学 deadline 早的, 总是好的
pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;

    courses.sort_unstable_by(|a, b|{
       a[1].cmp(&b[1])
    });

    // 用来存当前选的课程的耗时, 大顶堆
    let mut heap = BinaryHeap::new();
    // 总时间
    let mut total = 0;
    for course in courses {
        let (duration, deadline) = (course[0], course[1]);
        
        if total + duration <= deadline{
            // 如果能学完, 就学
            // 更新总时间
            total += duration;
            heap.push(duration);
            continue;
        } else {
            // 学不完, 看下最耗时那个, 能不能换掉
            if let Some(max_duration) = heap.peek(){
                if max_duration > &duration{
                    // 如果前面有个耗时更长的, 换掉
                    // 万一换掉也不够deadline呢? 
                    // 因为前面已经排好序了, 耗时更长的, deadline也更早
                    // max_duration > duration, 
                    // 所以 total - max_duration + duration < total <= max_deadline <= deadline
                    // 所以可以完成
                    total = total - max_duration + duration;
                    heap.pop();
                    heap.push(duration);
                }
            }
        }
    }

    heap.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_schedule_course() {
        struct TestCase {
            courses: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            TestCase {
                courses: vec2![[100, 200], [200, 1300], [1000, 1250], [2000, 3200]],
                expect: 3,
            },
            TestCase {
                courses: vec2![[1, 2]],
                expect: 1,
            },
            TestCase {
                courses: vec2![[3, 2], [4, 3]],
                expect: 0,
            },
            TestCase{
                courses: vec2![[5,5],[4,6],[2,6]],
                expect: 2,
            },
            TestCase{
                courses: vec2![[7,17],[3,12],[10,20],[9,10],[5,20],[10,19],[4,18]],
                expect: 4
            }
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { courses, expect })| {
            let actual = schedule_course(courses);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_matrix_score() {
        struct TestCase {
            grid: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            TestCase {
                grid: vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]],
                expect: 39,
            },
            TestCase {
                grid: vec![vec![0]],
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { grid, expect })| {
            let actual = matrix_score(grid);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_lemonade_change() {
        struct TestCase {
            bills: Vec<i32>,
            expect: bool,
        }

        vec![
            TestCase {
                bills: vec![5, 5, 5, 10, 20],
                expect: true,
            },
            TestCase {
                bills: vec![5, 5, 10],
                expect: true,
            },
            TestCase {
                bills: vec![10, 10],
                expect: false,
            },
            TestCase {
                bills: vec![5, 5, 10, 10, 20],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { bills, expect } = testcase;
            let actual = lemonade_change(bills);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

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
    fn test_car_fleet() {
        struct TestCase {
            target: i32,
            position: Vec<i32>,
            speed: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                target: 12,
                position: vec![10, 8, 0, 5, 3],
                speed: vec![2, 4, 1, 1, 3],
                expect: 3,
            },
            TestCase {
                target: 10,
                position: vec![3],
                speed: vec![3],
                expect: 1,
            },
            TestCase {
                target: 100,
                position: vec![0, 2, 4],
                speed: vec![4, 2, 1],
                expect: 1,
            },
            TestCase {
                target: 17,
                position: vec![8, 12, 16, 11, 7],
                speed: vec![6, 9, 10, 9, 7],
                expect: 4,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                target,
                position,
                speed,
                expect,
            } = testcase;
            let actual = car_fleet(target, position, speed);
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
        .iter()
        .for_each(|testcase| {
            let actual = find_content_children(testcase.g.to_vec(), testcase.s.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
