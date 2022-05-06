/// 判断两个map是否相等
/// * 特点:
///     * map是无序的
///
/// ## 题目
/// * 简单
///     * [242. 有效的字母异位词](is_anagram)
///     8 [383. 赎金信](can_construct)
/// * 中等
///     * [49. 字母异位词分组](group_anagrams)
mod just_equal {

    /// [242. 有效的字母异位词](https://leetcode-cn.com/problems/valid-anagram/)
    ///
    /// 题意解释:
    /// `t` 为 `s` 的字母异位词: `s`中包含`t`中所有字母
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() {
            return false;
        }

        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *counter.entry(c).or_insert(0) -= 1;
        }
        !counter.values().any(|&v| v < 0)
    }

    /// [49. 字母异位词分组](https://leetcode-cn.com/problems/group-anagrams/)
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn char_to_num(c: char) -> u32 {
            c as u32 - 'a' as u32
        }

        #[derive(Hash, PartialEq, Eq)]
        struct Mark {
            count: [usize; 26],
        }
        impl Mark {
            fn from_str(s: &str) -> Self {
                let mut cnt_map = [0; 26];
                for c in s.chars() {
                    let idx = char_to_num(c);
                    *cnt_map.get_mut(idx as usize).unwrap() += 1;
                }
                Mark { count: cnt_map }
            }
        }

        use std::collections::HashMap;

        let mut store: HashMap<Mark, Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let mark = Mark::from_str(s.as_str());
            store.entry(mark).or_insert(vec![]).push(s);
        }

        store.values().map(|s| s.clone()).collect()
    }

    /// [383. 赎金信](https://leetcode-cn.com/problems/ransom-note/)
    /// 
    /// 优化: 如果都是小写英文字母, 可以自己用数组替换hashmap
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;

        let mut counter = HashMap::new();
        for c in magazine.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        for c in ransom_note.chars() {
            let entry = counter.entry(c).or_insert(0);
            *entry -= 1;
            if *entry < 0 {
                return false;
            }
        }
        true
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_can_construct() {
            struct TestCase {
                name: &'static str,
                ransom_note: &'static str,
                magazine: &'static str,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    ransom_note: "a",
                    magazine: "b",
                    expect: false,
                },
                TestCase {
                    name: "basic 2",
                    ransom_note: "aa",
                    magazine: "ab",
                    expect: false,
                },
                TestCase {
                    name: "basic 3",
                    ransom_note: "aa",
                    magazine: "aab",
                    expect: true,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = can_construct(
                    testcase.ransom_note.to_string(),
                    testcase.magazine.to_string(),
                );
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_group_anagrams() {
            struct TestCase {
                name: &'static str,
                strs: &'static [&'static str],
                expect: &'static [&'static [&'static str]],
            }
            vec![TestCase {
                name: "basic",
                strs: &["eat", "tea", "tan", "ate", "nat", "bat"],
                expect: &[&["bat"], &["nat", "tan"], &["ate", "eat", "tea"]],
            }]
            .iter()
            .for_each(|testcase| {
                let strs = testcase.strs.iter().map(|s| s.to_string()).collect();
                let mut actual = group_anagrams(strs);
                let mut expect = testcase
                    .expect
                    .iter()
                    .map(|p| p.iter().map(|s| s.to_string()).collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>();
                actual.iter_mut().for_each(|v| v.sort());
                actual.sort();
                expect.iter_mut().for_each(|v| v.sort());
                expect.sort();

                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_is_anagram() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                t: &'static str,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "anagram",
                    t: "nagaram",
                    expect: true,
                },
                TestCase {
                    name: "basic 2",
                    s: "rat",
                    t: "car",
                    expect: false,
                },
                TestCase {
                    name: "fix 1",
                    s: "a",
                    t: "ab",
                    expect: false,
                },
                TestCase {
                    name: "fix 2",
                    s: "ab",
                    t: "a",
                    expect: false,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = is_anagram(testcase.s.to_string(), testcase.t.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// 利用hash加速查找
/// * 特点: 要求 O(1) 时间复杂度
///
mod just_find {}

///
/// 特点:
/// * 去重
/// * 检测是否已经存在(比如无限循环)
///
/// 可以使用set, 也可以根据数据范围, 手动维护hash
///
/// ## 题目
/// * 简单
///     * [349. 两个数组的交集](intersection)
///     * [350. 两个数组的交集 II](intersect)
///     * [1002. 查找共用字符](common_chars)
///     * [202. 快乐数](is_happy)
mod set_and_mark {
    /// [349. 两个数组的交集](https://leetcode-cn.com/problems/intersection-of-two-arrays/)
    ///
    /// 思路1:
    /// 由于题目说数组内元素范围是[0, 1000], 因此可以使用长度1001的数组, 直接用索引做hash
    /// ```
    /// pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    ///     let mut mark0 = vec![0; 1001];
    ///     let mut mark1 = vec![0; 1001];
    ///     for num in nums1 {
    ///         *mark0.get_mut(num as usize).unwrap() += 1;
    ///     }
    ///     for num in nums2 {
    ///         *mark1.get_mut(num as usize).unwrap() -= 1;
    ///     }
    ///     let mut result = vec![];
    ///     for idx in 0..mark0.len() {
    ///         if mark0[idx] * mark1[idx] < 0 {
    ///             result.push(idx as i32);
    ///         }
    ///     }
    ///     result
    /// }
    /// ```
    /// 不过由于元素可能重复出现, 直接使用一个数组去存标记, 有些不方便.
    ///
    /// 思路2: 使用set

    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let nums1_set = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2_set = nums2.into_iter().collect::<HashSet<i32>>();
        nums1_set.intersection(&nums2_set).copied().collect()
    }

    /// [350. 两个数组的交集 II](https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/)
    /// 直接使用set, 会消除内部的重复数据, 不符合要求.
    ///
    /// 思路1:
    /// 使用1001长度的数组维护, 仍然有效
    /// ```
    /// pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    ///     let mut mark0 = vec![0; 1001];
    ///     let mut mark1 = vec![0; 1001];
    ///     for num in nums1 {
    ///         *mark0.get_mut(num as usize).unwrap() += 1;
    ///     }
    ///     for num in nums2 {
    ///         *mark1.get_mut(num as usize).unwrap() += 1;
    ///     }
    ///     let mut result = vec![];
    ///     for idx in 0..mark0.len() {
    ///         let cnt = std::cmp::min(mark0[idx], mark1[idx]);
    ///         for _ in 1..=cnt {
    ///             result.push(idx as i32);
    ///         }
    ///     }
    ///     result
    /// }
    /// ```
    /// 思路2:
    /// 排序后合并数组的方式, 求交集
    /// 这样同时可以处理两个数组过大的问题
    /// 同时可以减少磁盘IO次数
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();

        let (mut a, mut b) = (0usize, 0usize);
        let mut result = vec![];
        while a < nums1.len() && b < nums2.len() {
            let x = nums1.get(a).copied().unwrap();
            let y = nums2.get(b).copied().unwrap();
            if x == y {
                result.push(x);
                a += 1;
                b += 1;
            } else if x < y {
                a += 1;
            } else {
                b += 1;
            }
        }
        result
    }

    /// [1002. 查找共用字符](https://leetcode-cn.com/problems/find-common-characters/)
    ///
    /// 优化的点:
    /// 可以不用全部存储, 只需要留下最小数量即可
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut mark: Vec<[i32; 26]> = vec![];
        for word in words {
            let mut line = [0; 26];
            for c in word.chars() {
                let idx = c as u32 - 'a' as u32;
                *line.get_mut(idx as usize).unwrap() += 1;
            }
            mark.push(line);
        }

        let mut result = vec![];
        for i in 0..26 {
            let mut min = i32::MAX;
            for line in mark.iter() {
                let c = line.get(i).copied().unwrap();
                min = std::cmp::min(min, c);
            }
            for _ in 1..=min {
                result.push(char::from_u32('a' as u32 + i as u32).unwrap());
            }
        }
        result.into_iter().map(|c| c.to_string()).collect()
    }

    /// [202. 快乐数](https://leetcode-cn.com/problems/happy-number/)
    pub fn is_happy(n: i32) -> bool {
        fn next(n: i32) -> i32 {
            let mut n = n;
            let mut digit = vec![];
            while n > 0 {
                digit.push(n % 10);
                n /= 10;
            }
            digit.into_iter().fold(0, |acc, d| acc + d * d)
        }

        use std::collections::HashSet;
        let mut mark = HashSet::new();
        let mut num = n;
        while num != 1 {
            if !mark.insert(num) {
                return false;
            }
            num = next(num);
        }
        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_happy() {
            struct TestCase {
                name: &'static str,
                n: i32,
                expect: bool,
            }
            vec![
                TestCase {
                    name: "basic",
                    n: 19,
                    expect: true,
                },
                TestCase {
                    name: "basic 2",
                    n: 2,
                    expect: false,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = is_happy(testcase.n);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_common_chars() {
            struct TestCase {
                name: &'static str,
                words: &'static [&'static str],
                expect: &'static [&'static str],
            }

            vec![
                TestCase {
                    name: "basic",
                    words: &["bella", "label", "roller"],
                    expect: &["e", "l", "l"],
                },
                TestCase {
                    name: "basic 2",
                    words: &["cool", "lock", "cook"],
                    expect: &["c", "o"],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let words: Vec<String> = testcase.words.iter().map(|s| s.to_string()).collect();
                let mut actual = common_chars(words);
                let mut expect: Vec<String> =
                    testcase.expect.iter().map(|s| s.to_string()).collect();
                actual.sort();
                expect.sort();

                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_intersect() {
            struct TestCase {
                name: &'static str,
                nums1: &'static [i32],
                nums2: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums1: &[1, 2, 2, 1],
                    nums2: &[2, 2],
                    expect: &[2, 2],
                },
                TestCase {
                    name: "basic 2",
                    nums1: &[4, 9, 5],
                    nums2: &[9, 4, 9, 8, 4],
                    expect: &[9, 4],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut actual = intersect(testcase.nums1.to_vec(), testcase.nums2.to_vec());
                actual.sort();
                let mut expect = testcase.expect.to_vec();
                expect.sort();

                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_intersection() {
            use std::collections::HashSet;
            struct TestCase {
                name: &'static str,
                nums1: &'static [i32],
                nums2: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums1: &[1, 2, 2, 1],
                    nums2: &[2, 2],
                    expect: &[2],
                },
                TestCase {
                    name: "basic 2",
                    nums1: &[4, 9, 5],
                    nums2: &[9, 4, 9, 8, 4],
                    expect: &[9, 4],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = intersection(testcase.nums1.to_vec(), testcase.nums2.to_vec());
                let actual_set: HashSet<i32> = HashSet::from_iter(actual.into_iter());
                let expect: HashSet<i32> = HashSet::from_iter(testcase.expect.iter().copied());

                assert_eq!(expect, actual_set, "{} failed", testcase.name);
            });
        }
    }
}
