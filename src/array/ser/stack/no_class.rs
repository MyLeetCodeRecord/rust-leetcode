//! 不好归类的
//! 
//! 题目:
//! * 简单
//! * 中等
//!     * [856. 括号的分数](score_of_parentheses)
//! * 困难

/// [856. 括号的分数](https://leetcode.cn/problems/score-of-parentheses/)
pub fn score_of_parentheses(s: String) -> i32 {
    let mut score = vec![0];
    for &b in s.as_bytes() {
        match b {
            b'(' => {
                score.push(0);
            }
            b')' => {
                let curr = score.pop().unwrap();
                let prev = score.pop().unwrap_or(0);
                score.push(prev + std::cmp::max(curr * 2, 1));
            }
            _ => {
                unreachable!()
            }
        }
    }
    score.first().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_parentheses() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase { s: "()", expect: 1 },
            Testcase {
                s: "(())",
                expect: 2,
            },
            Testcase {
                s: "()()",
                expect: 2,
            },
            Testcase {
                s: "(()(()))",
                expect: 6,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = score_of_parentheses(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
