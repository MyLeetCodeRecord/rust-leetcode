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
    for num in nums{
        if visited.contains(&(num-k)){
            res.insert(num-k);
        }
        if visited.contains(&(num+k)){
            res.insert(num);
        }
        visited.insert(num);
    }
    res.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

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
