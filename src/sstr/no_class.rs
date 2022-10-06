/// [859. 亲密字符串](https://leetcode.cn/problems/buddy-strings/)
///
/// 看上去和 [854. 相似度为 K 的字符串](crate::graph_search::dfs::k_similarity) 相似.
///
/// 不过题目要求更简单, 只能且必须交换一次.
///
/// ```
/// pub fn buddy_strings(s: String, goal: String) -> bool {
///     if s.len() != goal.len() {
///         return false;
///     }
///
///     let have_same = {
///         let mut mark = [0; 26];
///         for &b in s.as_bytes() {
///             mark[(b - b'a') as usize] += 1;
///         }
///         mark.iter().filter(|x| **x != 0).any(|x| *x >= 2)
///     };
///     let diff = s
///         .as_bytes()
///         .iter()
///         .zip(goal.as_bytes().iter())
///         .filter_map(|(a, b)| if a != b { Some((*a, *b)) } else { None })
///         .collect::<Vec<(u8, u8)>>();
///     if diff.is_empty() && have_same {
///         return true;
///     } else if diff.len() != 2 {
///         return false;
///     } else {
///         let (actual0, expect0) = diff[0];
///         let (actual1, expect1) = diff[1];
///         return actual0 == expect1 && expect0 == actual1;
///     }
/// }
/// ```
pub fn buddy_strings(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }

    let (sb, gb) = (s.as_bytes(), goal.as_bytes());
    let (mut count0, mut count1) = ([0usize; 26], [0usize; 26]);
    let mut diff_count = 0;
    for (&a, &b) in sb.iter().zip(gb.iter()) {
        count0[(a - b'a') as usize] += 1;
        count1[(b - b'a') as usize] += 1;
        if a != b {
            diff_count += 1;
        }
    }
    let mut have_same = false;
    for (&c0, &c1) in count0.iter().zip(count1.iter()) {
        if c0 != c1 {
            return false; // 词频不相同
        }
        have_same = have_same || (c0 >= 2);
    }
    diff_count == 2 || (diff_count == 0 && have_same)
}

/// [1796. 字符串中第二大的数字](https://leetcode.cn/problems/second-largest-digit-in-a-string/)
pub fn second_highest(s: String) -> i32 {
    let mut heap = [-1; 2];
    s.bytes().into_iter().for_each(|c| {
        if c >= b'0' && c <= b'9' {
            let num = (c - b'0') as i32;
            if heap[1] == -1 {
                heap[1] = num;
            } else if heap[1] == num {
            } else if heap[1] > num {
                if heap[0] == -1 {
                    heap[0] = heap[1];
                    heap[1] = num;
                }
            } else if heap[1] < num {
                if heap[0] == -1 {
                    heap[0] = num;
                } else if heap[0] == num {
                } else if heap[0] < num {
                    heap[1] = heap[0];
                    heap[0] = num;
                } else if heap[0] > num {
                    heap[1] = num;
                }
            }
        }
    });
    if heap[0] != -1 && heap[1] != -1 {
        heap[1]
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_highest() {
        struct TestCase {
            s: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                s: "dfa12321afd",
                expect: 2,
            },
            TestCase {
                s: "abc1111",
                expect: -1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = second_highest(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        })
    }

    #[test]
    fn test_buddy_strings() {
        struct TestCase {
            s: &'static str,
            goal: &'static str,
            expect: bool,
        }

        vec![
            TestCase {
                s: "ab",
                goal: "ba",
                expect: true,
            },
            TestCase {
                s: "ab",
                goal: "ab",
                expect: false,
            },
            TestCase {
                s: "aa",
                goal: "aa",
                expect: true,
            },
            TestCase {
                s: "abab",
                goal: "abab",
                expect: true,
            },
            TestCase {
                s: "aaab",
                goal: "aaab",
                expect: true,
            },
            TestCase {
                s: "ab",
                goal: "babbb",
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, goal, expect } = testcase;
            let actual = buddy_strings(s.to_string(), goal.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
