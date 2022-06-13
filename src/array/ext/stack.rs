/// 简单的压栈出栈操作: 先进先出
///
/// * 简单
///     * [20. 有效的括号](is_valid)
///     * [1021. 删除最外层的括号](remove_outer_parentheses)
pub mod simple {
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
                if (b')'.eq(b) && b'('.eq(last)) || (b']'.eq(b) && b'['.eq(last)) || (b'}'.eq(b) && b'{'.eq(last)) {
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

    #[cfg(test)]
    mod tests {
        use super::*;

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
}
