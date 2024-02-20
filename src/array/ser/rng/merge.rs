//! 区间合并
//!
//! * [56. 合并区间](merge)
//! * [57. 插入区间](insert)
//! * [758. 字符串中的加粗单词](bold_words)
//!

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
/// 相对[616. 给字符串添加加粗标签](crate::array::ext::trie::add_bold_tag) 只是将字符范围缩小到了仅英文字母
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

        [TestCase {
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
            }]
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

        [TestCase {
                name: "basic",
                intervals: &[&[1, 3], &[2, 6], &[8, 10], &[15, 18]],
                expect: &[&[1, 6], &[8, 10], &[15, 18]],
            },
            TestCase {
                name: "basic 2",
                intervals: &[&[1, 4], &[4, 5]],
                expect: &[&[1, 5]],
            }]
        .iter()
        .for_each(|testcase| {
            let intervals = testcase.intervals.iter().map(|s| s.to_vec()).collect();
            let actual = merge(intervals);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
