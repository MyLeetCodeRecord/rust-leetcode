//! # 栈相关题目
//! 特点: 先进后出
//!
//! * 简单
//!     * [20. 有效的括号](is_valid)
//!     * [1021. 删除最外层的括号](remove_outer_parentheses)
//! * 中等
//!     * [946. 验证栈序列](validate_stack_sequences)

/// [20. 有效的括号](https://leetcode.cn/problems/valid-parentheses/)
///
/// 不能使用双指针, 因为存在 "()[]{}"也是有效的
pub fn is_valid(s: String) -> bool {
    let bytes = s.as_bytes();
    let mut stack = vec![];
    for b in bytes {
        if b'('.eq(b) || b'['.eq(b) || b'{'.eq(b) {
            stack.push(*b);
            continue;
        }
        let mut mat: bool = false;
        if let Some(last) = stack.last() {
            if (b')'.eq(b) && b'('.eq(last))
                || (b']'.eq(b) && b'['.eq(last))
                || (b'}'.eq(b) && b'{'.eq(last))
            {
                stack.pop();
                mat = true;
            }
        }
        if !mat {
            stack.push(*b);
        }
    }
    stack.is_empty()
}

/// [1021. 删除最外层的括号](https://leetcode.cn/problems/remove-outermost-parentheses/)
pub fn remove_outer_parentheses(s: String) -> String {
    let mut stack = vec![];
    let mut mark = vec![true; s.len()];
    for (idx, chr) in s.chars().enumerate() {
        if chr == '(' {
            stack.push((chr, idx));
        } else {
            let mut last_i = None;
            if let Some(last) = stack.last() {
                if last.0 == '(' {
                    last_i.replace(last.1);
                }
            }
            if let Some(lasti) = last_i {
                stack.pop();
                if stack.is_empty() {
                    *mark.get_mut(idx).unwrap() = false;
                    *mark.get_mut(lasti).unwrap() = false;
                }
            }
        }
    }
    let mut s = s;
    let mut iter = mark.into_iter();
    s.retain(|_| iter.next().unwrap());
    s
}

/// [946. 验证栈序列](https://leetcode.cn/problems/validate-stack-sequences/)
///
/// 模拟入栈和出栈
///
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut popped = popped.iter().peekable();
    let mut stack = Vec::with_capacity(pushed.len());
    for num in pushed.into_iter() {
        stack.push(num);
        while let Some(&peak) = stack.last() {
            if peak != **popped.peek().unwrap() {
                break;
            }

            stack.pop();
            popped.next();
        }
    }
    stack.is_empty()
}

/// [1475. 商品折扣后的最终价格](https://leetcode.cn/problems/final-prices-with-a-special-discount-in-a-shop/)
pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = Vec::with_capacity(prices.len());
    let mut result = prices.clone();

    for (idx, price) in prices.into_iter().enumerate() {
        while let Some((i, l)) = stack.last() {
            if price > *l {
                break;
            }
            *result.get_mut(*i).unwrap() = l - price;
            stack.pop();
        }
        stack.push((idx, price));
    }
    return result;
}

/// [1598. 文件夹操作日志搜集器](https://leetcode.cn/problems/crawler-log-folder/)
pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut cnt = 0;
    for cd in logs {
        if cd == "../" {
            cnt = std::cmp::max(0, cnt - 1);
        } else if cd == "./" {
        } else {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        struct TestCase {
            name: &'static str,
            logs: &'static [&'static str],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                logs: &["d1/", "d2/", "../", "d21/", "./"],
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                logs: &["d1/", "d2/", "./", "d3/", "../", "d31/"],
                expect: 3,
            },
            TestCase {
                name: "basic 3",
                logs: &["d1/", "../", "../", "../"],
                expect: 0,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let logs = testcase.logs.iter().map(|s| s.to_string()).collect();
            let actual = min_operations(logs);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_final_prices() {
        struct TestCase {
            name: &'static str,
            prices: &'static [i32],
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic 1",
                prices: &[8, 4, 6, 2, 3],
                expect: &[4, 2, 4, 2, 3],
            },
            TestCase {
                name: "basic 2",
                prices: &[1, 2, 3, 4, 5],
                expect: &[1, 2, 3, 4, 5],
            },
            TestCase {
                name: "basic 3",
                prices: &[8, 4, 6, 2, 3],
                expect: &[4, 2, 4, 2, 3],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = final_prices(testcase.prices.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_validate_stack_sequences() {
        struct TestCase {
            name: &'static str,
            pushed: &'static [i32],
            popped: &'static [i32],
            expect: bool,
        }

        vec![
            TestCase {
                name: "basic 1",
                pushed: &[1, 2, 3, 4, 5],
                popped: &[4, 5, 3, 2, 1],
                expect: true,
            },
            TestCase {
                name: "basic 2",
                pushed: &[1, 2, 3, 4, 5],
                popped: &[4, 3, 5, 1, 2],
                expect: false,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual =
                validate_stack_sequences(testcase.pushed.to_vec(), testcase.popped.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_remove_outer_parentheses() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic 1",
                s: "(()())(())",
                expect: "()()()",
            },
            TestCase {
                name: "basic 2",
                s: "(()())(())(()(()))",
                expect: "()()()()(())",
            },
            TestCase {
                name: "basic 3",
                s: "()()",
                expect: "",
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = remove_outer_parentheses(testcase.s.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_is_valid() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            expect: bool,
        }

        vec![
            TestCase {
                name: "basic",
                s: "()",
                expect: true,
            },
            TestCase {
                name: "basic 2",
                s: "()[]{}",
                expect: true,
            },
            TestCase {
                name: "basic 3",
                s: "(]",
                expect: false,
            },
            TestCase {
                name: "basic 4",
                s: "([)]",
                expect: false,
            },
            TestCase {
                name: "basic 5",
                s: "{[]}",
                expect: true,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = is_valid(testcase.s.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
