//! 利用hash加速查找
//! * 特点: 要求 O(1) 时间复杂度
//!
//! ## 题目
//! * 简单
//!     * [953. 验证外星语词典](is_alien_sorted)
//! * 困难
//!

    /// [953. 验证外星语词典](https://leetcode.cn/problems/verifying-an-alien-dictionary/)
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let order_map = order
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect::<HashMap<char, usize>>();

        fn x_before_y(x: &str, y: &str, order_map: &HashMap<char, usize>) -> bool {
            let (x_len, y_len) = (x.len(), y.len());
            for (a, b) in x.chars().zip(y.chars()) {
                if a == b {
                    continue;
                }
                let (ai, bi) = (order_map.get(&a).unwrap(), order_map.get(&b).unwrap());
                match ai.cmp(bi) {
                    std::cmp::Ordering::Less => {
                        return true;
                    }
                    std::cmp::Ordering::Greater => {
                        return false;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }
            x_len <= y_len
        }

        for win in words.windows(2) {
            let (a, b) = (&win[0], &win[1]);
            if !x_before_y(a.as_str(), b.as_str(), &order_map) {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_alien_sorted() {
            struct TestCase {
                name: &'static str,
                words: &'static [&'static str],
                order: &'static str,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    words: &["hello", "leetcode"],
                    order: "hlabcdefgijkmnopqrstuvwxyz",
                    expect: true,
                },
                TestCase {
                    name: "basic 2",
                    words: &["word", "world", "row"],
                    order: "worldabcefghijkmnpqstuvxyz",
                    expect: false,
                },
                TestCase {
                    name: "basic 3",
                    words: &["apple", "app"],
                    order: "abcdefghijklmnopqrstuvwxyz",
                    expect: false,
                },
                TestCase {
                    name: "fix 1",
                    words: &["hello", "hello"],
                    order: "abcdefghijklmnopqrstuvwxyz",
                    expect: true,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let words = testcase.words.iter().map(|s| s.to_string()).collect();
                let actual = is_alien_sorted(words, testcase.order.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }
    }