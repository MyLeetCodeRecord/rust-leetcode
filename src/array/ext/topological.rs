/// [269. 火星词典](https://leetcode.cn/problems/alien-dictionary/)
///
/// 只有小写字母, 因此可以用数组代替hash
pub fn alien_order(words: Vec<String>) -> String {
    use std::collections::HashSet;
    struct Node {
        prev: HashSet<usize>,
        next: HashSet<usize>,
    }
    let mut mark: [Option<Node>; 26] = Default::default();

    for win in words.windows(2) {
        let (a, b) = (win[0].as_bytes(), win[1].as_bytes());

        // 有一条是: 同样前缀字符时, 长的在后面
        // 因此以第二个为准, 全量初始化
        for c in b.iter() {
            let ci = (*c - b'a') as usize;
            if mark[ci].is_none() {
                mark[ci].replace(Node {
                    prev: HashSet::new(),
                    next: HashSet::new(),
                });
            }
        }

        // 找不同
        let mut done = false;
        for (x, y) in a.iter().zip(b.iter()) {
            let (xi, yi) = ((*x - b'a') as usize, (*y - b'a') as usize);
            if mark[xi].is_none() {
                mark[xi].replace(Node {
                    prev: HashSet::new(),
                    next: HashSet::new(),
                });
            }
            if mark[yi].is_none() {
                mark[yi].replace(Node {
                    prev: HashSet::new(),
                    next: HashSet::new(),
                });
            }
            if x != y {
                mark.get_mut(xi)
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .next
                    .insert(yi);
                mark.get_mut(yi)
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .prev
                    .insert(xi);
                done = true;
                break;
            }
        }

        if done || a.len() == b.len() {
            // 已经决策出来, 跳过
            // 或者 两个字符都相同,判不出
        } else if a.len() > b.len() {
            // 前面字符都相同, 要求短的在前, 长的在后.
            return "".to_string();
        }
    }

    let mut part: Vec<char> = vec![];
    let mut confirmed = HashSet::new();
    loop {
        let mut found: Option<usize> = None;
        for (i, m) in mark.iter().enumerate() {
            if m.is_none() {
                continue;
            }
            if m.as_ref().unwrap().prev.is_empty() && confirmed.insert(i) {
                found.replace(i);
                break;
            }
        }
        if found.is_none() {
            break;
        }
        let chr = (found.unwrap() as u8 + b'a') as char;
        part.push(chr);

        for part in mark.iter_mut() {
            if part.is_none() {
                continue;
            }
            part.as_mut().unwrap().prev.remove(&found.unwrap());
        }
    }
    part.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alien_order() {
        struct TestCase {
            name: &'static str,
            words: &'static [&'static str],
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic",
                words: &["wrt", "wrf", "er", "ett", "rftt"],
                expect: "wertf",
            },
            TestCase {
                name: "basic 2",
                words: &["z", "x"],
                expect: "zx",
            },
            TestCase {
                name: "basic 3",
                words: &["z", "x", "z"],
                expect: "",
            },
            TestCase {
                name: "fix 1",
                words: &["z", "z"],
                expect: "z",
            },
            TestCase {
                name: "fix 2",
                words: &["ab", "adc"],
                expect: "abcd",
            },
            TestCase {
                name: "fix 3",
                words: &["abc", "ab"],
                expect: "",
            },
        ]
        .iter()
        .for_each(|testcase| {
            let words = testcase.words.iter().map(|s| s.to_string()).collect();
            let actual = alien_order(words);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
