//! 区间相关题目

/// 区间合并
///
/// * [56. 合并区间](merge)
/// * [57. 插入区间](insert)
/// * [758. 字符串中的加粗单词](bold_words)
pub mod merge {

    /// [56. 合并区间](https://leetcode.cn/problems/merge-intervals/)
    ///
    /// 可以利用原来的位置, 双指针
    /// 也可以用额外存储
    ///
    /// 需要先根据起点排序
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a.first().unwrap().cmp(b.first().unwrap()));
        let mut insert_pos = 0;
        for j in 1..intervals.len() {
            if intervals[j][0] <= intervals[insert_pos][1] {
                intervals[insert_pos][1] = std::cmp::max(intervals[insert_pos][1], intervals[j][1]);
            } else {
                insert_pos += 1;
                intervals[insert_pos][0] = intervals[j][0];
                intervals[insert_pos][1] = intervals[j][1];
            }
        }
        intervals.drain(insert_pos + 1..);
        intervals
    }

    /// [57. 插入区间](https://leetcode.cn/problems/insert-interval/)
    ///
    /// 思路1:
    /// 将 `new_interval` 添加到序列中, 然后就变成了[56. 合并区间](merge)
    /// ```ignore
    /// pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    ///     let mut intervals = intervals;
    ///     intervals.push(new_interval);
    ///     merge(intervals)
    /// }
    /// ```
    ///
    /// 但这种需要整体排序, 其实题目已说明, 原本序列有序.
    /// 也可以二分找到插入位置, 插入后合并, 但这种有内存移位.
    ///
    /// 因此稍微有点双指针的策略
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // 用left, right记录当前待插入的边界
        let (mut left, mut right) = (
            new_interval.first().copied().unwrap(),
            new_interval.last().copied().unwrap(),
        );
        let mut ans = vec![];
        //let mut placed = false;

        for inter in intervals.iter() {
            let (a, b) = (
                inter.first().copied().unwrap(),
                inter.last().copied().unwrap(),
            );
            // 如果不能合并
            if a > right || b < left {
                // 找到小的那个, 插入, 大的更新为新待判定
                ans.push(vec![std::cmp::min(a, left), std::cmp::min(b, right)]);
                left = left.max(a);
                right = right.max(b);
            } else {
                // 合并一下
                left = left.min(a);
                right = right.max(b);
            }
        }
        // 把最后一个补充进去
        ans.push(vec![left, right]);
        ans
    }

    /// [758. 字符串中的加粗单词](https://leetcode.cn/problems/bold-words-in-string/)
    ///
    /// 相对[616. 给字符串添加加粗标签](add_bold_tag) 只是将字符范围缩小到了仅英文字母
    /// 由于前面使用的char, 因此可以覆盖
    ///
    /// 除了字典树, 还可以模拟
    pub fn bold_words(words: Vec<String>, s: String) -> String {
        let mut is_bold = vec![false; s.len()];
        for i in 0..s.len() {
            let prefix = s.get(i..).unwrap();
            for word in words.iter() {
                if prefix.starts_with(word.as_str()) {
                    for flag in is_bold
                        .iter_mut()
                        .take(std::cmp::min(i + word.len(), s.len()))
                        .skip(i)
                    {
                        *flag = true;
                    }
                }
            }
        }
        let mut ans = String::new();
        let mut cursor = 0;
        while cursor < s.len() {
            if is_bold[cursor] {
                ans.push_str("<b>");
                let start = cursor;
                while cursor < s.len() && is_bold[cursor] {
                    cursor += 1;
                }
                ans.push_str(s.get(start..cursor).unwrap());
                ans.push_str("</b>");
            } else {
                let start = cursor;
                while cursor < s.len() && !is_bold[cursor] {
                    cursor += 1;
                }
                ans.push_str(s.get(start..cursor).unwrap());
            }
        }
        ans
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bold_words() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                words: &'static [&'static str],
                expect: &'static str,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "aabcd",
                    words: &["ab", "bc"],
                    expect: "a<b>abc</b>d",
                },
                TestCase {
                    name: "basic 2",
                    s: "aabcd",
                    words: &["ab", "cb"],
                    expect: "a<b>ab</b>cd",
                },
            ]
            .iter()
            .for_each(|testcase| {
                let words = testcase.words.iter().map(|s| s.to_string()).collect();
                let actual = bold_words(words, testcase.s.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_insert() {
            struct TestCase {
                name: &'static str,
                intervals: &'static [&'static [i32]],
                new_interval: &'static [i32],
                expect: &'static [&'static [i32]],
            }

            vec![
                TestCase {
                    name: "basic",
                    intervals: &[&[1, 3], &[6, 9]],
                    new_interval: &[2, 5],
                    expect: &[&[1, 5], &[6, 9]],
                },
                TestCase {
                    name: "basic 2",
                    intervals: &[&[1, 2], &[3, 5], &[6, 7], &[8, 10], &[12, 16]],
                    new_interval: &[4, 8],
                    expect: &[&[1, 2], &[3, 10], &[12, 16]],
                },
                TestCase {
                    name: "basic 3",
                    intervals: &[],
                    new_interval: &[5, 7],
                    expect: &[&[5, 7]],
                },
                TestCase {
                    name: "basic 4",
                    intervals: &[&[1, 5]],
                    new_interval: &[2, 3],
                    expect: &[&[1, 5]],
                },
                TestCase {
                    name: "basic 5",
                    intervals: &[&[1, 5]],
                    new_interval: &[2, 7],
                    expect: &[&[1, 7]],
                },
                TestCase {
                    name: "fix 1",
                    intervals: &[&[1, 5]],
                    new_interval: &[6, 8],
                    expect: &[&[1, 5], &[6, 8]],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let intervals = testcase.intervals.iter().map(|s| s.to_vec()).collect();
                let actual = insert(intervals, testcase.new_interval.to_vec());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_merge() {
            struct TestCase {
                name: &'static str,
                intervals: &'static [&'static [i32]],
                expect: &'static [&'static [i32]],
            }

            vec![
                TestCase {
                    name: "basic",
                    intervals: &[&[1, 3], &[2, 6], &[8, 10], &[15, 18]],
                    expect: &[&[1, 6], &[8, 10], &[15, 18]],
                },
                TestCase {
                    name: "basic 2",
                    intervals: &[&[1, 4], &[4, 5]],
                    expect: &[&[1, 5]],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let intervals = testcase.intervals.iter().map(|s| s.to_vec()).collect();
                let actual = merge(intervals);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// 区间覆盖
///
/// * [45. 跳跃游戏 II](jump)
/// * [55. 跳跃游戏](can_jump)
/// * [763. 划分字母区间](partition_labels)
/// * [330. 按要求补齐数组](min_patches)
/// * [1024. 视频拼接](video_stitching)
/// * [1326. 灌溉花园的最少水龙头数目](min_taps)
///
pub mod overlap {
    /// [45. 跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/)
    ///
    /// 思路: 贪心
    /// [2,3,1,2,4,2,3]
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
}

/// # 差分数组
///
/// 特点:
/// * 区间
/// * 与前缀比: 原数组变动
///
/// 步骤:
/// 1. 差分: `diff[0] = origin[0]; diff[i] = origin[i] - origin[i-1]`
/// 2. 操作: `diff[i] += inc; diff[j] -= inc`
/// 3. 还原: `origin[0] = diff[0]; origin[i] = origin[i-1] + diff[i]`
///
/// ## 题目
/// * 简单
/// * 中等
///     * [370. 区间加法](get_modified_array)
///     * [1109. 航班预订统计](corp_flight_bookings)
///     * [1094. 拼车](car_pooling)
/// * 困难
pub mod diff_sub {

    /// [370. 区间加法](https://leetcode-cn.com/problems/range-addition/)
    ///
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0; length as usize];
        let mut diff = vec![0; length as usize];

        for update in updates.iter() {
            let (start, end, inc) = (update[0] as usize, update[1] as usize, update[2]);
            diff[start] += inc;
            if end + 1 < diff.len() {
                diff[end + 1] -= inc;
            }
        }

        // restore
        ret[0] = diff[0];
        for i in 1..ret.len() {
            ret[i] = diff[i] + ret[i - 1];
        }
        ret
    }

    /// [1109. 航班预订统计](https://leetcode-cn.com/problems/corporate-flight-bookings/)
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize];
        let mut diff = vec![0; n as usize];

        for booking in bookings.iter() {
            let (start, end, inc) = (booking[0] as usize - 1, booking[1] as usize - 1, booking[2]);
            diff[start] += inc;
            if end + 1 < diff.len() {
                diff[end + 1] -= inc;
            }
        }

        // restore
        ret[0] = diff[0];
        for i in 1..ret.len() {
            ret[i] = diff[i] + ret[i - 1];
        }
        ret
    }

    /// [1094. 拼车](https://leetcode-cn.com/problems/car-pooling/)
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let end = { trips.iter().map(|trip| trip[2]).max().unwrap() };

        let mut counter = vec![0; end as usize];
        let mut diff = vec![0; end as usize];

        for trip in trips.iter() {
            // [from, to) 到了就下车, 不包含在内
            let (cnt, from, to) = (trip[0], trip[1] as usize, trip[2] as usize - 1);

            diff[from] += cnt;
            if to + 1 < diff.len() {
                diff[to + 1] -= cnt;
            }
        }
        // restore
        counter[0] = diff[0];
        for i in 1..counter.len() {
            counter[i] = diff[i] + counter[i - 1];
        }

        counter.iter().all(|cnt| *cnt <= capacity)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_car_pooling() {
            struct TestCase {
                name: &'static str,
                trips: &'static [&'static [i32]],
                capacity: i32,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    trips: &[&[2, 1, 5], &[3, 3, 7]],
                    capacity: 4,
                    expect: false,
                },
                TestCase {
                    name: "basic 2",
                    trips: &[&[2, 1, 5], &[3, 3, 7]],
                    capacity: 5,
                    expect: true,
                },
                TestCase {
                    name: "fix 1",
                    trips: &[&[2, 1, 5], &[3, 5, 7]],
                    capacity: 3,
                    expect: true,
                },
                TestCase {
                    name: "fix 2",
                    trips: &[&[9, 0, 1], &[3, 3, 7]],
                    capacity: 4,
                    expect: false,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let trips = testcase.trips.iter().map(|x| x.to_vec()).collect();
                let actual = car_pooling(trips, testcase.capacity);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_corp_flight_bookings() {
            struct TestCase {
                name: &'static str,
                bookings: &'static [&'static [i32]],
                n: i32,
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    bookings: &[&[1, 2, 10], &[2, 3, 20], &[2, 5, 25]],
                    n: 5,
                    expect: &[10, 55, 45, 25, 25],
                },
                TestCase {
                    name: "basic 2",
                    bookings: &[&[1, 2, 10], &[2, 2, 15]],
                    n: 2,
                    expect: &[10, 25],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let bookings = testcase.bookings.iter().map(|x| x.to_vec()).collect();
                let actual = corp_flight_bookings(bookings, testcase.n);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_get_modified_array() {
            struct TestCase {
                name: &'static str,
                length: i32,
                updates: &'static [&'static [i32]],
                expect: &'static [i32],
            }

            vec![TestCase {
                name: "basic",
                length: 5,
                updates: &[&[1, 3, 2], &[2, 4, 3], &[0, 2, -2]],
                expect: &[-2, 0, 3, 5, 3],
            }]
            .iter()
            .for_each(|testcase| {
                let updates = testcase.updates.iter().map(|x| x.to_vec()).collect();
                let actual = get_modified_array(testcase.length, updates);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// # 线段树
/// > <https://oi-wiki.org/ds/seg/>
///
/// * [327. 区间和的个数](https://leetcode.cn/problems/count-of-range-sum/)
/// * [715. Range 模块](https://leetcode.cn/problems/range-module/)
mod seg {}
