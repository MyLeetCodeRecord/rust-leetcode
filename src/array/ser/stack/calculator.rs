//! # 计算器专题
//!
//! 又称, 中缀表达式专题
//!
//! ## 题目
//!
//! * 简单
//! * 中等
//!     * [227. 基本计算器 II](calculate2)
//! * 困难
//!     * [224. 基本计算器](calculate)
//!     * [772. 基本计算器 III](calculate3)
//!

/// [224. 基本计算器](https://leetcode.cn/problems/basic-calculator/)
///
/// 思路： 将括号展开，
/// * `-(1+2)` ==> `-1 -2`
/// * `+(1-2)` ==> `1 - 2`
pub fn calculate(s: String) -> i32 {
    let mut ops = vec![];
    ops.push(0 + 1);
    let mut sign = 1;

    let mut ret = 0;
    let mut cursor = 0;
    let stream = s.as_bytes();
    while cursor < stream.len() {
        if stream[cursor] == b' ' {
            cursor += 1;
        } else if stream[cursor] == b'+' {
            sign = ops.last().copied().unwrap();
            cursor += 1;
        } else if stream[cursor] == b'-' {
            sign = 0 - ops.last().copied().unwrap();
            cursor += 1;
        } else if stream[cursor] == b'(' {
            ops.push(sign);
            cursor += 1;
        } else if stream[cursor] == b')' {
            ops.pop();
            cursor += 1;
        } else {
            let mut num = 0;
            while cursor < stream.len() && stream[cursor] >= b'0' && stream[cursor] <= b'9' {
                num = num * 10 + (stream[cursor] - b'0') as i32;
                cursor += 1;
            }
            ret = ret + sign * num;
        }
    }
    ret
}

/// [227. 基本计算器 II](https://leetcode.cn/problems/basic-calculator-ii/)
///
/// 思路: 将 乘除 法先计算出来
///
pub fn calculate2(s: String) -> i32 {
    let mut ops = vec![];
    let mut lits = vec![];
    ops.push(1);

    let stream = s.as_bytes();
    let mut cursor = 0;
    while cursor < stream.len() {
        if stream[cursor] == b' ' {
            cursor += 1;
        } else if stream[cursor] == b'+' {
            ops.push(1);
            cursor += 1;
        } else if stream[cursor] == b'-' {
            ops.push(2);
            cursor += 1;
        } else if stream[cursor] == b'*' {
            ops.push(3);
            cursor += 1;
        } else if stream[cursor] == b'/' {
            ops.push(4);
            cursor += 1;
        } else {
            let mut num = 0;
            while cursor < stream.len() && stream[cursor] >= b'0' && stream[cursor] <= b'9' {
                num = num * 10 + (stream[cursor] - b'0') as i32;
                cursor += 1;
            }

            match ops.pop() {
                None => unreachable!(),
                // 乘除算出来, 其他可以最后统一处理
                Some(3) => {
                    let mut last_num = lits.pop().unwrap();
                    last_num = last_num * num;
                    lits.push(last_num);
                }
                Some(4) => {
                    let mut last_num = lits.pop().unwrap();
                    last_num = last_num / num;
                    lits.push(last_num);
                }
                Some(x) => {
                    lits.push(num);
                    ops.push(x);
                }
            }
        }
    }

    ops.into_iter()
        .zip(lits.into_iter())
        .fold(0, |r, (op, lit)| {
            if op == 1 {
                r + lit
            } else if op == 2 {
                r - lit
            } else {
                unreachable!()
            }
        })
}

/// [772. 基本计算器 III](https://leetcode.cn/problems/basic-calculator-iii/)
pub fn calculate3(s: String) -> i32 {
    fn mul_div(ops: &mut Vec<i32>, lits: &mut Vec<i32>, num: i32) {
        // 合并乘法/除法
        match ops.pop() {
            None => unreachable!(),
            Some(3) => {
                let mut last_num = lits.pop().unwrap();
                last_num = last_num * num;
                lits.push(last_num);
            }
            Some(4) => {
                let mut last_num = lits.pop().unwrap();
                last_num = last_num / num;
                lits.push(last_num);
            }
            Some(x) => {
                lits.push(num);
                ops.push(x); // 再放回
            }
        }
    }

    let mut ops = vec![];
    let mut lits = vec![];
    ops.push(1);

    let stream = s.as_bytes();
    let mut cursor = 0;
    while cursor < stream.len() {
        let peek = stream[cursor];
        if peek == b' ' {
            cursor += 1;
        } else if peek == b'+' {
            ops.push(1);
            cursor += 1;
        } else if peek == b'-' {
            ops.push(2);
            cursor += 1;
        } else if peek == b'*' {
            ops.push(3);
            cursor += 1;
        } else if peek == b'/' {
            ops.push(4);
            cursor += 1;
        } else if peek == b'(' {
            ops.push(0);
            cursor += 1;
        } else if peek == b')' {
            // pop 直到找到对应的 (
            // 先pop出来逆序, 运算完再压回去
            let mut ops_temp = vec![];
            let mut lits_tmp = vec![];
            loop {
                let lit = lits.pop().unwrap();
                let op = ops.pop().unwrap();
                lits_tmp.push(lit);
                if op == 0 {
                    break;
                }
                ops_temp.push(op);
            }
            ops_temp.push(1);
            let num = ops_temp
                .into_iter()
                .rev()
                .zip(lits_tmp.into_iter().rev())
                .fold(0, |r, (op, lit)| if op == 1 { r + lit } else { r - lit });

            mul_div(&mut ops, &mut lits, num);

            cursor += 1;
        } else {
            let mut num = 0;
            while cursor < stream.len() && stream[cursor] >= b'0' && stream[cursor] <= b'9' {
                num = num * 10 + (stream[cursor] - b'0') as i32;
                cursor += 1;
            }

            mul_div(&mut ops, &mut lits, num);
        }
    }
    ops.into_iter().zip(lits.into_iter()).fold(
        0,
        |r, (op, lit)| {
            if op == 1 {
                r + lit
            } else {
                r - lit
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate3() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase {
                s: "1+1",
                expect: 2,
            },
            Testcase {
                s: "6-4/2",
                expect: 4,
            },
            Testcase {
                s: "2*(5+5*2)/3+(6/2+8)",
                expect: 21,
            },
            Testcase {
                s: "(2+6*3+5-(3*14/7+2)*5)+3",
                expect: -12,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = calculate3(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_calculate2() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase {
                s: "3+2*2",
                expect: 7,
            },
            Testcase {
                s: " 3/2 ",
                expect: 1,
            },
            Testcase {
                s: " 3+5 / 2 ",
                expect: 5,
            },
            Testcase {
                s: "1-1+1",
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = calculate2(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_calculate() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase {
                s: "1 + 1",
                expect: 2,
            },
            Testcase {
                s: " 2-1 + 2 ",
                expect: 3,
            },
            Testcase {
                s: "(1+(4+5+2)-3)+(6+8)",
                expect: 23,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = calculate(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
