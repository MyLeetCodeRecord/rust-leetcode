/// [468. 验证IP地址](https://leetcode.cn/problems/validate-ip-address/)
pub fn valid_ip_address(query_ip: String) -> String {
    fn is_ipv4(s: &str) -> bool {
        let mut cnt = 0;
        for p in s.split('.') {
            cnt += 1;
            if cnt > 4 {
                return false;
            }

            if p.is_empty() {
                return false;
            } else if p == "0" {
                continue;
            } else if p.starts_with('0') {
                return false;
            } else {
                match p.parse::<i32>() {
                    Ok(0..=255) => {}
                    _ => {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_ipv6(s: &str) -> bool {
        if s.starts_with('0') && !s.starts_with("0:") {
            return false;
        }

        let mut cnt = 0;
        for p in s.split(':') {
            cnt += 1;
            if (cnt > 8) || (p.is_empty() || p.len() > 4) || (i64::from_str_radix(p, 16).is_err()) {
                return false;
            }
        }
        true
    }

    if query_ip.len() <= 4 {
        return "Neither".to_string();
    }
    match query_ip.get(3..4) {
        Some(".") => {
            if is_ipv4(query_ip.as_str()) {
                return "IPv4".to_string();
            }
        }
        _ => {
            if is_ipv6(query_ip.as_str()) {
                return "IPv6".to_string();
            }
        }
    }

    "Neither".to_string()
}

/// [532. 数组中的 k-diff 数对](https://leetcode.cn/problems/k-diff-pairs-in-an-array/)
pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;

    let (mut visited, mut res) = (HashSet::new(), HashSet::new());
    for num in nums {
        if visited.contains(&(num - k)) {
            res.insert(num - k);
        }
        if visited.contains(&(num + k)) {
            res.insert(num);
        }
        visited.insert(num);
    }
    res.len() as i32
}

/// [1592. 重新排列单词间的空格](https://leetcode.cn/problems/rearrange-spaces-between-words/)
pub fn reorder_spaces(text: String) -> String {
    const SPACE: u8 = b' ';

    enum State {
        NotInWord,
        InWord,
    }

    let mut words = vec![];
    let mut space_cnt = 0;
    let mut state = State::NotInWord;

    let mut tmp = vec![];
    for &c in text.as_bytes() {
        if c == SPACE {
            space_cnt += 1;
            match state {
                State::InWord => {
                    words.push(tmp.clone());
                    tmp.clear();
                    state = State::NotInWord;
                }
                State::NotInWord => {}
            }
        } else {
            tmp.push(c);
            match state {
                State::InWord => {}
                State::NotInWord => {
                    state = State::InWord;
                }
            }
        }
    }
    if !tmp.is_empty() {
        words.push(tmp.clone());
        tmp.clear();
    }

    let mut split = 0;
    let mut result = vec![];
    if words.len() > 1 {
        split = space_cnt / (words.len() - 1);
    }

    for word in words {
        result.extend(word);
        if space_cnt >= split {
            result.extend(vec![b' '; split]);
            space_cnt -= split;
        }
    }
    result.extend(vec![b' '; space_cnt]);

    unsafe { String::from_utf8_unchecked(result) }
}

/// [830. 较大分组的位置](https://leetcode.cn/problems/positions-of-large-groups/)
/// ```
/// pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
///     let s = s.as_bytes();
///     if s.len() < 3 {
///         return vec![];
///     }
///
///     let mut ret: Vec<Vec<i32>> = vec![];
///
///     let (mut start, mut cursor) = (0, 1);
///     let mut curr = s[0];
///
///     while cursor < s.len() {
///         if s[cursor] == curr {
///             if cursor - start + 1 >= 3 {
///                 match ret.last_mut() {
///                     Some(last) if last[0] == start as i32 => {
///                         last[1] = cursor as i32;
///                         cursor += 1;
///                         continue;
///                     }
///                     _ => {}
///                 }
///                 ret.push(vec![start as i32, cursor as i32]);
///             }
///         } else {
///             start = cursor;
///             curr = s[cursor];
///         }
///         cursor += 1;
///     }
///
///     ret
/// }
/// ```
pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let s = s.as_bytes();
    let mut ret: Vec<Vec<i32>> = vec![];
    let mut cnt = 1;
    for i in 0..s.len() {
        if i == s.len() - 1 || s[i] != s[i + 1] {
            if cnt >= 3 {
                // 必须是 `i + 1 - cnt`, 不能是 `i - cnt + 1`, 会溢出
                ret.push(vec![(i + 1 - cnt) as i32, i as i32]);
            }
            cnt = 1;
        } else {
            cnt += 1;
        }
    }
    ret
}

/// [831. 隐藏个人信息](https://leetcode.cn/problems/masking-personal-information/)
pub fn mask_pii(s: String) -> String {
    let (mut is_email, mut at_pos) = (false, 0);

    let mut chrs = vec![];
    for &(mut b) in s.as_bytes() {
        if b == b'+' || b == b'-' || b == b'(' || b == b')' || b == b' ' {
            continue;
        }
        if b == b'@' {
            is_email = true;
            at_pos = chrs.len();
        } else if b >= b'A' && b <= b'Z' {
            b = b - b'A' + b'a';
        }
        chrs.push(b);
    }

    let mut ret = vec![];
    if is_email {
        ret.push(chrs[0]);
        ret.extend_from_slice("*****".as_bytes());
        ret.extend_from_slice(&chrs[at_pos - 1..]);
    } else {
        if chrs.len() == 13 {
            ret.extend_from_slice("+***-***-***-".as_bytes());
        } else if chrs.len() == 12 {
            ret.extend_from_slice("+**-***-***-".as_bytes());
        } else if chrs.len() == 11 {
            ret.extend_from_slice("+*-***-***-".as_bytes());
        } else {
            ret.extend_from_slice("***-***-".as_bytes());
        }
        ret.extend_from_slice(&chrs[chrs.len() - 4..]);
    }
    String::from_utf8(ret).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_mask_pii() {
        struct TestCase {
            s: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                s: "LeetCode@LeetCode.com",
                expect: "l*****e@leetcode.com",
            },
            TestCase {
                s: "AB@qq.com",
                expect: "a*****b@qq.com",
            },
            TestCase {
                s: "1(234)567-890",
                expect: "***-***-7890",
            },
            TestCase {
                s: "86-(10)12345678",
                expect: "+**-***-***-5678",
            },
            TestCase{
                s: "+86(88)1513-7-74",
                expect: "+*-***-***-3774"
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = mask_pii(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_large_group_positions() {
        struct TestCase {
            s: &'static str,
            expect: Vec<Vec<i32>>,
        }

        vec![
            // TestCase {
            //     s: "abbxxxxzzy",
            //     expect: vec2![[3, 6]],
            // },
            // TestCase {
            //     s: "abc",
            //     expect: vec2![],
            // },
            // TestCase {
            //     s: "abcdddeeeeaabbbcd",
            //     expect: vec2![[3,5],[6,9],[12,14]],
            // },
            TestCase {
                s: "aaa",
                expect: vec2![[0, 2]],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = large_group_positions(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_reorder_spaces() {
        struct TestCase {
            name: &'static str,
            text: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic 1",
                text: "  this   is  a sentence ",
                expect: "this   is   a   sentence",
            },
            TestCase {
                name: "basic 2",
                text: " practice   makes   perfect",
                expect: "practice   makes   perfect ",
            },
            TestCase {
                name: "basic 3",
                text: "hello   world",
                expect: "hello   world",
            },
            TestCase {
                name: "basic 4",
                text: "  walks  udp package   into  bar a",
                expect: "walks  udp  package  into  bar  a ",
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = reorder_spaces(testcase.text.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_pairs() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            k: i32,
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[3, 1, 4, 1, 5],
                k: 2,
                expect: 2,
            },
            TestCase {
                name: "basic 1",
                nums: &[1, 2, 3, 4, 5],
                k: 1,
                expect: 4,
            },
            TestCase {
                name: "basic 1",
                nums: &[1, 3, 1, 5, 4],
                k: 0,
                expect: 1,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = find_pairs(testcase.nums.to_vec(), testcase.k);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_valid_ip_address() {
        struct TestCase {
            name: &'static str,
            query_ip: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic 1",
                query_ip: "172.16.254.1",
                expect: "IPv4",
            },
            TestCase {
                name: "basic 2",
                query_ip: "2001:0db8:85a3:0:0:8A2E:0370:7334",
                expect: "IPv6",
            },
            TestCase {
                name: "basic 3",
                query_ip: "256.256.256.256",
                expect: "Neither",
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = valid_ip_address(testcase.query_ip.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }
}
