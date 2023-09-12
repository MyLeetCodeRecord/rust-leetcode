//! 区间覆盖
//!
//! * [45. 跳跃游戏 II](jump)
//! * [55. 跳跃游戏](can_jump)
//! * [763. 划分字母区间](partition_labels)
//! * [330. 按要求补齐数组](min_patches)
//! * [1024. 视频拼接](video_stitching)
//! * [1326. 灌溉花园的最少水龙头数目](min_taps)
//!

/// [45. 跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/)
///
/// 思路: 贪心
/// `[2,3,1,2,4,2,3]`
/// * 初始位置是下标 0，从下标 0 出发，最远可到达下标 2
/// * 候选位置有下标1, 下标2, 分别对应步数3, 1
/// * 因此在两步之内, 最远到达 1 + 3, 第一个落点选 下标1 最合适
///
/// * 如果将每次一步视为一个阶段, 那下标0对应的阶段, 最远到下标2
/// * 因此到下标2时, 表示需要切换阶段了.
/// * 切换后到达 1+3, 进入下一阶段
///
/// * 题目保证一定可以到达, 因此可以不做死循环判定
pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut end, mut farthest) = (0, 0);
    let mut jumps = 0;
    for (i, &right) in nums.iter().enumerate().take(nums.len() - 1) {
        farthest = farthest.max(right as usize + i); // 记录在这个阶段最远到哪个位置
        if end == i {
            // 切换阶段
            jumps += 1;
            end = farthest;
        }
    }
    jumps
}

/// [55. 跳跃游戏](https://leetcode.cn/problems/jump-game/)
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut right_most = 0;
    for (i, n) in nums.iter().enumerate() {
        if i <= right_most {
            right_most = right_most.max(i + *n as usize);
            if right_most >= nums.len() - 1 {
                // 终点是最后一个元素, 因此 等于 nums.len() - 1 也可
                return true;
            }
        }
        // 如果出现了i > right_most 说明出现了断层
    }
    false
}

/// [763. 划分字母区间](https://leetcode.cn/problems/partition-labels/)
///
/// 只有英文字母, 用数组代替hashmap
/// 算是 双指针/滑动窗口
pub fn partition_labels(s: String) -> Vec<i32> {
    // 先存下每个字符最后出现的位置
    let mut mark = [0usize; 26];
    for (i, b) in s.as_bytes().iter().enumerate() {
        let idx = (*b - b'a') as usize;
        mark[idx] = i;
    }

    let mut ret = vec![];

    let mut start = -1;
    let mut right = 0;
    // 从头开始遍历
    for i in 0..s.len() {
        // 计算出当前字母最后出现的位置
        let b = s.as_bytes().get(i).copied().unwrap();
        let idx = (b - b'a') as usize;
        let may_right = mark.get(idx).copied().unwrap();
        // 由于保证每个字符只出现在一段内, 因此右边界可能有变化
        right = right.max(may_right);
        // 如果当前就是段的结尾, 就存下长度, 并更新起点.
        // 注意起点其实不包含start
        if right == i {
            ret.push(right as i32 - start);
            start = i as i32;
        }
    }
    ret
}

/// [330. 按要求补齐数组](https://leetcode.cn/problems/patching-array/)
///
/// 区间平移
///
/// 数学:
/// 1. 对于正整数 x，如果区间 `[1,x−1]` 内的所有数字都已经被覆盖，且 x 在数组中，则区间 `[1,2x−1]` 内的所有数字也都被覆盖
/// 2. 上面命题不要求 可逆. 即充分条件即可
///
/// 因此,
/// 1. 对于初始区间, 假设数字 x 缺失，则至少需要在数组中补充一个小于或等于 x 的数，才能覆盖到 x，否则无法覆盖到 x.
/// 2. 如果区间`[1,m−1]`内的所有数字都已经被覆盖, 补充的数m后, `[1,2m−1]`都会被覆盖, 因此下一个需要补充的数, 绝对不小于`2m`
///
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut x = 1i64;
    let n = n as i64; // 防溢出
    let mut cnt = 0;
    let (length, mut index) = (nums.len(), 0);
    while x <= n {
        if index < length && nums[index] <= x as i32 {
            // 能覆盖这个数, 那加上这个数的区间也能覆盖
            // 加上后, 相当于区间平移
            x += nums[index] as i64;
            index += 1;
        } else {
            x *= 2;
            cnt += 1;
        }
    }
    cnt
}

/// [1024. 视频拼接](https://leetcode.cn/problems/video-stitching/)
///
/// [Lexicographical comparison](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html#lexicographical-comparison)
///
/// 用最少的片段, 覆盖整个区间.
/// 和 [55. 跳跃游戏](can_jump) 一样, 只是这次用区间描述了步长
pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
    // use std::convert::TryFrom;
    let mut clips = clips;
    // 小的在前, 长的在前
    clips.sort();
    // vec 自身的Ord实现就够, 不用自己写sort
    // clips.sort_by(|a, b|{
    //     let [s0, e0] = <[i32; 2]>::try_from(a.as_slice()).ok().unwrap();
    //     let [s1, e1] = <[i32; 2]>::try_from(b.as_slice()).ok().unwrap();
    //     match s0.cmp(&s1){
    //         std::cmp::Ordering::Equal => {
    //             e0.cmp(&e1)
    //         },
    //         a @_ => a
    //     }
    // });
    let mut curr_right = 0;
    let mut cnt = 1; // 初始 [0, x] 段默认加个1, 处理起来方便一点

    let mut max_right = 0;
    for clip in clips {
        let [start, end] = <[i32; 2]>::try_from(clip.as_slice()).ok().unwrap();
        if start <= curr_right {
            // 如果start 处于 [0, curr_right]内, 则更新本阶段的最大值
            // max_right = max_right.max(end);
        } else {
            // 如果start 不处于 [0, curr_right]内, 则需要切换阶段
            // curr_right 变为 max_right
            // 然后重新检查
            curr_right = max_right;
            if start > curr_right {
                // 断链
                return -1;
            }
            cnt += 1;
        }
        max_right = max_right.max(end);
        if max_right >= time {
            // 可能超过了需要的范围, 提前跳出
            break;
        }
    }
    if max_right < time {
        -1
    } else {
        cnt
    }
}

/// [1326. 灌溉花园的最少水龙头数目](https://leetcode.cn/problems/minimum-number-of-taps-to-open-to-water-a-garden/)
///
/// 和 [1024. 视频拼接](video_stitching) 基本一样, 只是区间是用 `[i -  ranges[i], i + ranges[i]]` 给出
/// 区间起点可能为负数, 终点超过范围, 因此需要裁剪一下
///
pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let mut clips = ranges
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let (mut start, mut end) = (i as i32 - x, i as i32 + x);
            start = start.max(0); // 负数部分忽略
            end = end.min(n); // 超出的部分忽略
            (start, end)
        })
        .collect::<Vec<(i32, i32)>>();
    clips.sort();

    let mut cnt = 1;
    let mut curr_right = 0;

    let mut max_right = 0;
    for (start, end) in clips {
        if start > curr_right {
            curr_right = max_right;
            cnt += 1;
            if start > curr_right {
                return -1;
            }
        }
        max_right = max_right.max(end);
        if max_right >= n {
            break;
        }
    }
    if max_right < n {
        -1
    } else {
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_taps() {
        struct TestCase {
            name: &'static str,
            n: i32,
            ranges: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                n: 5,
                ranges: &[3, 4, 1, 1, 0, 0],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                n: 5,
                ranges: &[3, 4, 1, 1, 0, 0],
                expect: 1,
            },
            TestCase {
                name: "fix 1",
                n: 8,
                ranges: &[4, 0, 0, 0, 0, 0, 0, 0, 4],
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = min_taps(testcase.n, testcase.ranges.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_video_stitching() {
        struct TestCase {
            name: &'static str,
            clips: &'static [&'static [i32]],
            time: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                clips: &[&[0, 2], &[4, 6], &[8, 10], &[1, 9], &[1, 5], &[5, 9]],
                time: 10,
                expect: 3,
            },
            TestCase {
                name: "basic 2",
                clips: &[&[0, 1], &[1, 2]],
                time: 5,
                expect: -1,
            },
            TestCase {
                name: "basic 3",
                clips: &[
                    &[0, 1],
                    &[6, 8],
                    &[0, 2],
                    &[5, 6],
                    &[0, 4],
                    &[0, 3],
                    &[6, 7],
                    &[1, 3],
                    &[4, 7],
                    &[1, 4],
                    &[2, 5],
                    &[2, 6],
                    &[3, 4],
                    &[4, 5],
                    &[5, 7],
                    &[6, 9],
                ],
                time: 9,
                expect: 3,
            },
            TestCase {
                name: "fix 1",
                clips: &[
                    &[5, 7],
                    &[1, 8],
                    &[0, 0],
                    &[2, 3],
                    &[4, 5],
                    &[0, 6],
                    &[5, 10],
                    &[7, 10],
                ],
                time: 5,
                expect: 1,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let clips = testcase.clips.iter().map(|c| c.to_vec()).collect();
            let actual = video_stitching(clips, testcase.time);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_min_patches() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            n: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[1, 3],
                n: 6,
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                nums: &[1, 5, 10],
                n: 20,
                expect: 2,
            },
            TestCase {
                name: "basic 3",
                nums: &[1, 2, 2],
                n: 5,
                expect: 0,
            },
            TestCase {
                name: "fix 1",
                nums: &[1, 2, 31, 33],
                n: 2147483647,
                expect: 28,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = min_patches(testcase.nums.to_vec(), testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_partition_labels() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic 1",
                s: "ababcbacadefegdehijhklij",
                expect: &[9, 7, 8],
            },
            TestCase {
                name: "basic 2",
                s: "eccbbbbdec",
                expect: &[10],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = partition_labels(testcase.s.to_string());
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
                name: "basic 1",
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
                nums: &[2, 0],
                expect: true,
            },
            TestCase {
                name: "fix 2",
                nums: &[2, 5, 0, 0],
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
    fn test_jump() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[2, 3, 1, 1, 4],
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                nums: &[2, 3, 0, 1, 4],
                expect: 2,
            },
            TestCase {
                name: "fix 1",
                nums: &[2, 1],
                expect: 1,
            },
            TestCase {
                name: "fix 2",
                nums: &[3, 2, 1],
                expect: 1,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = jump(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
