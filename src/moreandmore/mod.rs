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
//! * 中等
//!     * [376. 摆动序列](wiggle_max_length)
//!     * [55. 跳跃游戏](can_jump)
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
/// 因为给出了最大长度, 因此
/// 每次直接跳跃拉满, 然后记录已经走过的节点
/// 如果发现超过了终点, 也就是能到达,
pub fn can_jump(_nums: Vec<i32>) -> bool {
    // if nums.len() <= 1{
    //     return true;
    // }
    // use std::collections::HashSet;

    // let mut visited = HashSet::new();
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: bool,
        }

        vec![
            // TestCase {
            //     name: "basic",
            //     nums: &[2, 3, 1, 1, 4],
            //     expect: true,
            // },
            // TestCase {
            //     name: "basic 2",
            //     nums: &[3, 2, 1, 0, 4],
            //     expect: false,
            // },
            // TestCase{
            //     name: "fix 1",
            //     nums: &[0],
            //     expect: true
            // },
            // TestCase{
            //     name: "fix 2",
            //     nums: &[0, 2,3],
            //     expect: false
            // },
            // TestCase{
            //     name: "basic 3",
            //     nums: &[2,0,1],
            //     expect: true
            // },
            // TestCase{
            //     name: "fix 3",
            //     nums: &[1,0,1,0],
            //     expect: false
            // },
            TestCase{
                name: "fix 4",
                nums: &[3,0,8,2,0,0,1],
                expect: false
            }
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
