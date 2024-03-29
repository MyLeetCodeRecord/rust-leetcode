//! BFS
//!
//! 题目:
//! * 简单
//! * 中等
//!     * [847. 访问所有节点的最短路径](shortest_path_length)
//! * 困难

/// [847. 访问所有节点的最短路径](https://leetcode.cn/problems/shortest-path-visiting-all-nodes/)
///
/// 1. 从不同点出发, 最终的路径长度可能不同,  因为要求 __访问所有节点的最短路径的长度__ 中的最小值, 因此总共需要进行 `n` 组路径计算,
/// 2. 而经过一个节点, 可以有 $2^n$ 种可能(节点可以重复使用),  为防止 "往回走", 可以用 `n` 位二进制标识 _走法_.
/// 3. 根据graph, 可以得到所有从节点 `i` 出发的相邻节点, 检查能到的下一个节点,
///     - 如果经过这个节点的 _走法_ 已经枚举过了, 则忽略.
///     - 如果没有, 则继续延展, 这时距离+1
///
pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let n = graph.len();

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; 1 << n]; n];

    for i in 0..n {
        // 从i节点开始搜索, 总过枚举n的起点
        queue.push_back((i, 1 << i, 0));
        visited[i][1 << i] = true; // 走法 1<<i 存一下
    }

    let mut ans = 0;
    while !queue.is_empty() {
        let (i, mask, dist) = queue.pop_front().unwrap();
        if mask == (1 << n) - 1 {
            // 全为1, 表示当前节点访问了所有其他节点
            // 同时由于是 bfs, 因此先找到的必然最小, 即为结果
            ans = dist;
            break;
        }
        // 相邻节点
        for e in graph.get(i).unwrap() {
            let e = *e as usize;
            // 将mask的第e位值为1, 更新状态
            let mask_e = mask | (1 << e);
            // 经过e的, 走法为 mask_e
            if !visited[e][mask_e] {
                // 下一步, 从e继续, 当前状态更新至 mask_e, 路径+1
                queue.push_back((e, mask_e, dist + 1));
                visited[e][mask_e] = true;
            }
        }
    }
    ans
}

/// [854. 相似度为 K 的字符串](https://leetcode.cn/problems/k-similar-strings/)
/// 
/// - BFS解法
/// - [DFS解法](crate::graph_search::dfs::k_similarity)
/// - [A*启发式解法](crate::graph_search::a_start::k_similarity)
pub fn k_similarity(s1: String, s2: String) -> i32 {
    use std::collections::{HashSet, VecDeque};

    let n = s1.as_bytes().len();
    let s2b = s2.as_bytes();

    let mut step = 0;
    let mut queue = VecDeque::new();
    // 当前状态, 以及前面多少字符已经相同
    queue.push_back((s1.as_bytes().to_vec(), 0));

    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let (mut s, mut i) = queue.pop_front().unwrap();
            if s.eq(s2b) {
                // break 两层也可以, 不过直接返回方便
                return step;
            }
            // 前面idx已经相同
            while i < n && s[i] == s2b[i] {
                // 相同, 可以不用交换, 跳过
                i += 1;
            }
            // 这时i不相同, 从i+1开始找, 和目标s2b[i]相同的, 且s[j] != s2b[j]
            // 即需要过滤处于正确位置的, 否则会平白增加交换次数
            for j in i + 1..n {
                if s[j] == s2b[i] && s[j] != s2b[j] {
                    s.swap(i, j);
                    if visited.insert(s.clone()) {
                        queue.push_back((s.clone(), i + 1));
                    }
                    s.swap(i, j); // 回溯?
                }
            }
        }
        step += 1;
    }
    unreachable!("题目保证必然可以交换得到, 因此前面的BFS能保证已经返回")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_k_similarity() {
        struct Testcase {
            s1: &'static str,
            s2: &'static str,
            expect: i32,
        }

        vec![
            Testcase {
                s1: "ab",
                s2: "ba",
                expect: 1,
            },
            Testcase {
                s1: "abc",
                s2: "bca",
                expect: 2,
            },
            Testcase {
                s1: "bccaba",
                s2: "abacbc",
                expect: 3,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s1, s2, expect } = testcase;
            let actual = k_similarity(s1.to_string(), s2.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_shortest_path_length() {
        struct Testcase {
            graph: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                graph: vec2![[1, 2, 3], [0], [0], [0]],
                expect: 4,
            },
            Testcase {
                graph: vec2![[1], [0, 2, 4], [1, 3, 4], [2], [1, 2]],
                expect: 4,
            },
            Testcase {
                graph: vec2![
                    [2, 3, 5, 7],
                    [2, 3, 7],
                    [0, 1],
                    [0, 1],
                    [7],
                    [0],
                    [10],
                    [9, 10, 0, 1, 4],
                    [9],
                    [7, 8],
                    [7, 6]
                ],
                expect: 14,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { graph, expect } = testcase;
            let actual = shortest_path_length(graph);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
