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

#[cfg(test)]
mod tests {
    use super::*;

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
