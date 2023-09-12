//! 拓扑排序
//!
//! 特点:
//! 1. 有向无环图(DAG)
//! 2. 有入度为0的节点
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [851. 喧闹和富有](loud_and_rich)
//!     * [207. 课程表](can_finish)
//!     * [210. Course Schedule II](find_order)
//!     * [1462. Course Schedule IV](check_if_prerequisite)
//! * 困难
//!     * [269. 火星词典](alien_order)
//!

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

/// [851. 喧闹和富有](https://leetcode.cn/problems/loud-and-rich/)
/// 解法2: DFS
/// 1. 通过 `graph` 记录谁比 `i` 更富有(直接)
/// 2. 通过DFS逐层深入, 直到最富有
///     * 最富有, 则最终答案就是它自己
/// 3. 相对 `i` 富有的得到答案后, 更新 `i` 对应的答案, 可能的答案有
///     * 自身 `i`
///     * 相对 `i` 富有的节点的答案
/// 4. 由于相对 `i` 富有(直接)的节点可能不止一个, 因此需要都检查一下
/// ```
/// pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
///     let mut graph = vec![vec![]; quiet.len()];
///     for r in richer {
///         let (to, from) = (r[0] as usize, r[1] as usize);
///         graph[from].push(to); // to 是 比 from 富有的
///     }
///
///     fn dfs(ans: &mut Vec<i32>, graph: &Vec<Vec<usize>>, quiet: &Vec<i32>, x: usize) {
///         if ans[x] != -1 {
///             // 已经判出结果的, 不用重复判
///             // 不论结果是啥, 因为dfs后有更新的逻辑
///             // 所以只要有结果, 那就是最终答案
///             return;
///         }
///         ans[x] = x as i32; // 用自身填位先
///         for &y in graph.get(x).unwrap() {
///             dfs(ans, graph, quiet, y);
///             // 比x富有的答案
///             let ans_x = ans[x] as usize;
///             // 比x富有的的y的最终答案,
///             // 由于是先执行的dfs, 因此这里已经拿到相对富有的y的最终答案
///             let ans_y = ans[y] as usize;
///             if quiet[ans_y] < quiet[ans_x] {
///                 ans[x] = ans[y]; // 比x富有的y, 有比当前结果更安静的
///             }
///         }
///     }
///
///     let mut ans = vec![-1; quiet.len()];
///     for i in 0..quiet.len() {
///         // 都尝试一遍, 因为不定哪个最富, 哪个最穷
///         dfs(&mut ans, &graph, &quiet, i);
///     }
///     ans
/// }
/// ```
///
/// 解法3: 拓扑排序 - 精简
/// 1. 通过 `graph` 记录谁比 `i` 更穷(直接)
///     * 穷的那个, 记录下有几个比自己更富有, 即入度
///     * 如果入度为0, 则这个节点为最富有的
/// 2. 将入度为0的节点存入节点, 逐个处理
///     * 最富有的节点, 其答案就是其自身
///     * 计算出相对富有后, 更新比它穷的节点, 可选项有:
///         * 穷节点自身
///         * 富节点的答案
/// 3. 但相对穷的节点, 可能并不止一个相对富的节点,
///     * 其入度减1, 为0之后升级为"最"富有的节点, 存入队列
/// ```
/// pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
///     use std::collections::VecDeque;
///
///     let mut graph = vec![vec![]; quiet.len()];
///     let mut in_deg = vec![0; quiet.len()];
///     for r in richer {
///         let (rich, poor) = (r[0] as usize, r[1] as usize);
///         graph[rich].push(poor); // 记录谁比它穷
///         in_deg[poor] += 1; // 入度为0的, 为最富的
///     }
///
///     let mut ans = (0..quiet.len() as i32).collect::<Vec<i32>>(); // 先用自身填位
///     let mut queue = VecDeque::new();
///     for (idx, &ii) in in_deg.iter().enumerate() {
///         if ii == 0 {
///             queue.push_back(idx);
///         }
///     }
///
///     while !queue.is_empty() {
///         let rich = queue.pop_front().unwrap();
///         // 先计算更富有的, 全局最富有的, 其最终答案就是它自身
///         let ans_rich = ans[rich] as usize;
///         for &poor in graph.get(rich).unwrap() {
///             // 计算出富有的之后, 更新比它穷的
///             let ans_poor = ans[poor] as usize;
///             if quiet[ans_rich] < quiet[ans_poor] {
///                 ans[poor] = ans_rich as i32;
///             }
///             // 从这个 rich 到 这个 poor 已经检查过了
///             // 后面不用在检查了, 因此对poor来说, 入度减1
///             in_deg[poor] -= 1;
///             if in_deg[poor] == 0 {
///                 // 这时poor已经没有入度, 因此升级为最富有的
///                 queue.push_back(poor);
///             }
///         }
///     }
///     ans
/// }
/// ```
/// 解法1: 拓扑排序 - 复杂
/// `[a, b]` 表示 `a` 比 `b`更有钱.
/// 最后求不比`i`钱少的中, 最安静的, 最有钱的答案对应其自身.
///
/// 记`[a, b]`为`b -> a`, 则找到哪个出度为0, 则对应为最有钱的.
/// 处理掉这个最有钱的后, 将`b`的出度减一, 再找到出度为0的, 重复处理.
///
/// 处理的过程: 看是不是之前有比它更有钱的, 如果有, 从中找出最安静的那个.
///
pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet, VecDeque};

    let mut out = vec![0; quiet.len()];
    let mut oout = HashMap::new();
    let mut iin = HashMap::new();
    for rich in richer {
        let (to, from) = (rich[0] as usize, rich[1] as usize);
        out[from] += 1;
        iin.entry(to).or_insert(HashSet::new()).insert(from);
        oout.entry(from).or_insert(HashSet::new()).insert(to);
    }

    let mut queue = VecDeque::new();
    for (idx, &cnt) in out.iter().enumerate() {
        if cnt == 0 {
            queue.push_back(idx); // 这点起始就是最富
        }
    }

    let mut ans = vec![0; quiet.len()];
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let rich = queue.pop_front().unwrap();
            // rich 是当前最富的, 但不一定是全局最富的,
            // 因此检查其指向谁, 然后取下一个节点, 和其ans中的最安静的那个
            let (mut most, mut qu) = (rich, quiet[rich]);

            for &node in oout.get(&rich).unwrap_or(&HashSet::new()) {
                if quiet[node] < qu {
                    most = node;
                    qu = quiet[node];
                }
                let node_ans = ans[node] as usize;
                if quiet[node_ans] < qu {
                    most = node_ans;
                    qu = quiet[node_ans];
                }
            }
            ans[rich] = most as i32;

            // 将指向 rich 的节点, 出度都减1
            for &node in iin.get(&rich).unwrap_or(&HashSet::new()) {
                out[node] -= 1;
                if out[node] == 0 {
                    queue.push_back(node); // 出度为0, 下一轮的最富
                }
            }
        }
    }
    ans
}

/// [207. 课程表](https://leetcode.cn/problems/course-schedule/)
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;

    let mut count = num_courses as usize;

    let mut graph = vec![vec![]; count];
    let mut in_deg = vec![0; count];
    for pre in prerequisites {
        let (want, require) = (pre[0], pre[1]);
        in_deg[want as usize] += 1;
        graph[require as usize].push(want as usize);
    }

    let mut queue = VecDeque::new();
    for (idx, &iin) in in_deg.iter().enumerate() {
        if iin == 0 {
            queue.push_back(idx);
        }
    }

    while !queue.is_empty() {
        let require = queue.pop_front().unwrap();
        count -= 1;
        for &want in graph.get(require).unwrap() {
            in_deg[want] -= 1;
            if in_deg[want] == 0 {
                queue.push_back(want);
            }
        }
    }
    count == 0
}

/// [210. Course Schedule II](https://leetcode.cn/problems/course-schedule-ii/)
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let count = num_courses as usize;

    let mut in_deg = vec![0; count];
    let mut graph = vec![vec![]; count];
    for pre in prerequisites {
        let (want, require) = (pre[0], pre[1]);
        in_deg[want as usize] += 1;
        graph[require as usize].push(want as usize);
    }

    let mut queue = VecDeque::new();
    for (idx, &iin) in in_deg.iter().enumerate() {
        if iin == 0 {
            queue.push_back(idx);
        }
    }

    let mut result = vec![];
    while !queue.is_empty() {
        let require = queue.pop_front().unwrap();
        result.push(require as i32);
        for &want in graph.get(require).unwrap() {
            in_deg[want] -= 1;
            if in_deg[want] == 0 {
                queue.push_back(want);
            }
        }
    }
    if result.len() == count {
        return result;
    }
    vec![]
}

/// [1462. Course Schedule IV](https://leetcode.cn/problems/course-schedule-iv/description/)
pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    use std::collections::VecDeque;

    let mut graph = vec![vec![]; num_courses as usize];
    let mut in_degrees = vec![0; num_courses as usize];
    for pre in prerequisites {
        let (require, want) = (pre[0], pre[1]);
        in_degrees[want as usize] += 1;
        graph[require as usize].push(want as usize);
    }

    let mut queue = VecDeque::new();
    for (cls, &in_degree) in in_degrees.iter().enumerate() {
        if in_degree == 0 {
            queue.push_back(cls);
        }
    }

    let mut query_map = vec![vec![0; num_courses as usize]; num_courses as usize];
    while !queue.is_empty() {
        let require = queue.pop_front().unwrap();
        for &want in graph.get(require).unwrap() {
            // [x][y] == 1, 表示 x 是 y 的前置
            query_map[require][want] = 1;
            for col in 0..query_map.len() {
                // 将require的前置, 也加入到want的前置中
                query_map[col][want] = query_map[col][want] | query_map[col][require];
            }

            in_degrees[want] -= 1;
            if in_degrees[want] == 0 {
                queue.push_back(want);
            }
        }
    }
    queries
        .into_iter()
        .map(|q| query_map[q[0] as usize][q[1] as usize] == 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_check_if_prerequisite() {
        struct TestCase {
            num_courses: i32,
            prerequisites: Vec<Vec<i32>>,
            queries: Vec<Vec<i32>>,
            expect: Vec<bool>,
        }

        vec![
            TestCase {
                num_courses: 2,
                prerequisites: vec2![[1, 0]],
                queries: vec2![[0, 1], [1, 0]],
                expect: vec![false, true],
            },
            TestCase {
                num_courses: 2,
                prerequisites: vec![],
                queries: vec2![[1, 0], [0, 1]],
                expect: vec![false, false],
            },
            TestCase {
                num_courses: 3,
                prerequisites: vec2![[1, 2], [1, 0], [2, 0]],
                queries: vec2![[1, 0], [1, 2]],
                expect: vec![true, true],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                TestCase {
                    num_courses,
                    prerequisites,
                    queries,
                    expect,
                },
            )| {
                let actual = check_if_prerequisite(num_courses, prerequisites, queries);
                assert_eq!(expect, actual, "case {} failed", idx);
            },
        )
    }

    #[test]
    fn test_find_order() {
        struct TestCase {
            num_courses: i32,
            prerequisites: Vec<Vec<i32>>,
            expect: Vec<Vec<i32>>,
        }

        vec![
            TestCase {
                num_courses: 2,
                prerequisites: vec2![[1, 0]],
                expect: vec2![[0, 1]],
            },
            TestCase {
                num_courses: 4,
                prerequisites: vec2![[1, 0], [2, 0], [3, 1], [3, 2]],
                expect: vec2![[0, 2, 1, 3], [0, 1, 2, 3]],
            },
            TestCase {
                num_courses: 1,
                prerequisites: vec![],
                expect: vec2![[0]],
            },
            TestCase {
                num_courses: 3,
                prerequisites: vec2![[1, 0], [1, 2], [0, 1]],
                expect: vec![],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(
            |(
                idx,
                TestCase {
                    num_courses,
                    prerequisites,
                    expect,
                },
            )| {
                let actual = find_order(num_courses, prerequisites);
                if expect.is_empty() {
                    assert!(actual.is_empty(), "case {} failed", idx);
                    return;
                }
                assert!(
                    expect.into_iter().any(|x| x == actual),
                    "case {} failed",
                    idx
                );
            },
        )
    }

    #[test]
    fn test_can_finish() {
        struct TestCase {
            num_courses: i32,
            prerequisites: Vec<Vec<i32>>,
            expect: bool,
        }

        vec![
            TestCase {
                num_courses: 2,
                prerequisites: vec2![[1, 0]],
                expect: true,
            },
            TestCase {
                num_courses: 2,
                prerequisites: vec2![[1, 0], [0, 1]],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                num_courses,
                prerequisites,
                expect,
            } = testcase;
            let actual = can_finish(num_courses, prerequisites);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_loud_and_rich() {
        struct TestCase {
            richer: Vec<Vec<i32>>,
            quiet: Vec<i32>,
            expect: Vec<i32>,
        }

        vec![
            TestCase {
                richer: vec2![[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]],
                quiet: vec![3, 2, 5, 4, 6, 1, 7, 0],
                expect: vec![5, 5, 2, 5, 4, 5, 6, 7],
            },
            TestCase {
                richer: vec2![],
                quiet: vec![0],
                expect: vec![0],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                richer,
                quiet,
                expect,
            } = testcase;
            let actual = loud_and_rich(richer, quiet);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

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
