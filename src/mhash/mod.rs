/// 判断两个map是否相等
/// * 特点:
///     * map是无序的
///
/// ## 题目
/// * 简单
///     * [242. 有效的字母异位词](is_anagram)
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

    #[cfg(test)]
    mod tests {
        use super::*;

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
mod just_find {

    
}
