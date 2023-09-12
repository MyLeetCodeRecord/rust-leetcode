//! 不知道咋分类的
//! 
//! 题目:
//! * 简单
//!     * [1592. 重新排列单词间的空格](reorder_spaces)
//!     * [830. 较大分组的位置](large_group_positions)
//!     * [1694. 重新格式化电话号码](reformat_number)
//!     * [859. 亲密字符串](buddy_strings)
//!     * [1796. 字符串中第二大的数字](second_highest)
//! * 中等
//!     * [468. 验证IP地址](valid_ip_address)
//!     * [532. 数组中的 k-diff 数对](find_pairs)
//!     * [831. 隐藏个人信息](mask_pii)
//!     * [833. 字符串中的查找与替换](find_replace_string)
//! * 困难

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

    String::from_utf8(result).unwrap()
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

/// [833. 字符串中的查找与替换](https://leetcode.cn/problems/find-and-replace-in-string/)
///
/// 输入`indices`不保证有序, 因此需要整体组合后排序, 否则会产生互相影响.
///
pub fn find_replace_string(
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    let mut ops = vec![];
    for i in 0..indices.len() {
        ops.push((
            indices[i] as usize,
            sources[i].as_str(),
            targets[i].as_str(),
        ));
    }
    ops.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = String::new();

    let mut cursor = 0;
    for (start, source, target) in ops {
        if start > cursor {
            // 如果中间存在不需要替换的, 跳过的, 需要补齐
            ans.push_str(s.get(cursor..start).unwrap());
            cursor = start; // 游标向前, cursor 始终是 s 种的需要判定的起始(字符)
        }
        if s.get(start..).unwrap().starts_with(source) {
            ans.push_str(target);
            cursor += source.len(); // cursor是相对s的, 注意替换的长度
        }
    }
    // 需要补齐剩余
    if cursor < s.len() {
        ans.push_str(s.get(cursor..).unwrap());
    }
    ans
}

/// [1694. 重新格式化电话号码](https://leetcode.cn/problems/reformat-phone-number/)
pub fn reformat_number(number: String) -> String {
    let nums: Vec<u8> = number
        .as_bytes()
        .into_iter()
        .filter_map(|&x| {
            if x >= b'0' && x <= b'9' {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    let mut ans = Vec::new();
    let mut cursor = 0;
    while nums.len() - cursor > 4 {
        ans.extend_from_slice(nums.get(cursor..cursor + 3).unwrap());
        ans.push(b'-');
        cursor += 3;
    }
    match nums.len() - cursor {
        4 => {
            ans.extend_from_slice(nums.get(cursor..cursor + 2).unwrap());
            ans.push(b'-');
            ans.extend_from_slice(nums.get(cursor + 2..).unwrap());
        }
        _ => ans.extend_from_slice(nums.get(cursor..).unwrap()),
    }
    String::from_utf8(ans).unwrap()
}

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
    use crate::vec2;

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

    #[test]
    fn test_reformat_number() {
        struct Testcase {
            number: &'static str,
            expect: &'static str,
        }

        vec![
            Testcase {
                number: "1-23-45 6",
                expect: "123-456",
            },
            Testcase {
                number: "123 4-567",
                expect: "123-45-67",
            },
            Testcase {
                number: "123 4-5678",
                expect: "123-456-78",
            },
            Testcase {
                number: "12",
                expect: "12",
            },
            Testcase {
                number: "--17-5 229 35-39475 ",
                expect: "175-229-353-94-75",
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { number, expect } = testcase;
            let actual = reformat_number(number.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_find_replace_string() {
        struct TestCase {
            s: &'static str,
            indices: Vec<i32>,
            sources: Vec<&'static str>,
            targets: Vec<&'static str>,
            expect: &'static str,
        }

        vec![
            TestCase {
                s: "abcd",
                indices: vec![0, 2],
                sources: vec!["a", "cd"],
                targets: vec!["eee", "ffff"],
                expect: "eeebffff",
            },
            TestCase {
                s: "abcd",
                indices: vec![0, 2],
                sources: vec!["ab", "ec"],
                targets: vec!["eee", "ffff"],
                expect: "eeecd",
            },
            TestCase {
                s: "vmokgggqzp",
                indices: vec![3, 5, 1],
                sources: vec!["kg", "ggq", "mo"],
                targets: vec!["s", "so", "bfr"],
                expect: "vbfrssozp",
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                s,
                indices,
                sources,
                targets,
                expect,
            } = testcase;
            let sources = sources.into_iter().map(str::to_string).collect();
            let targets = targets.into_iter().map(str::to_string).collect();
            let acutal = find_replace_string(s.to_string(), indices, sources, targets);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }

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
            TestCase {
                s: "+86(88)1513-7-74",
                expect: "+*-***-***-3774",
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
            TestCase {
                s: "abbxxxxzzy",
                expect: vec2![[3, 6]],
            },
            TestCase {
                s: "abc",
                expect: vec2![],
            },
            TestCase {
                s: "abcdddeeeeaabbbcd",
                expect: vec2![[3,5],[6,9],[12,14]],
            },
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
