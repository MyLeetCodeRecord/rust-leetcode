//! A* 启发式搜索
//!
//! 经典blog: <https://www.redblobgames.com/pathfinding/a-star/introduction.html>
//!
//! - 最小路径, 要求heuristic小于实际值
//! - 否则就会退化为Greedy Best-First Search, 不保证最小路径
//!

/// [854. 相似度为 K 的字符串](https://leetcode.cn/problems/k-similar-strings/)
///
/// - [BFS解法](crate::graph_search::bfs::k_similarity)
/// - [DFS解法](crate::graph_search::dfs::k_similarity)
/// - A* 启发式解法
pub fn k_similarity(s1: String, s2: String) -> i32 {
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct State {
        cost: usize,         // 已经交换了多少次
        position: usize,     // 当前的位置(从左向右, 从0开始)
        curr: Vec<(u8, u8)>, // 当前的状态, 由于不是dfs, 需要保存状态快照
    }

    impl State {
        fn diff_count(&self) -> usize {
            self.curr
                .iter()
                .skip(self.position)
                .filter(|a| a.0 != a.1)
                .count()
        }
        fn heuristic(&self) -> usize {
            // 直接使用 diff_count 并不一定得到 最小交换
            // 启发式搜索能取到最小cost的条件是 heuristic 比实际值小, 不包含等于
            (self.diff_count() + 1) / 2
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            // cost小的优先, position大的优先
            let (h0, h1) = (self.heuristic(), other.heuristic());
            (self.cost + h0)
                .cmp(&(other.cost + h1))
                .reverse()
                .then_with(|| self.position.cmp(&other.position))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let diff = s1
        .as_bytes()
        .iter()
        .zip(s2.as_bytes())
        .filter_map(|(a, b)| if a != b { Some((*a, *b)) } else { None })
        .collect::<Vec<(u8, u8)>>();

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();

    visited.insert(diff.clone());
    pq.push(State {
        cost: 0,
        position: 0,
        curr: diff,
    });

    while !pq.is_empty() {
        let State {
            cost,
            mut position,
            mut curr,
        } = pq.pop().unwrap();

        while position < curr.len() {
            let (actual, expect) = curr[position];
            if actual == expect {
                position += 1;
            } else {
                break;
            }
        }
        // 已经处理到结尾了, 由于是cost小顶堆, 因此这就是最小结果
        if position >= curr.len() {
            return cost as i32;
        }
        // 使 position 位置的字符变成期望的
        // 从其后面交换一个过来
        // 取最小的那个继续, 不过也可以将所有的状态压入pq, 通过优先队列处理最小最大
        let (actual, expect) = curr[position];
        for j in position + 1..curr.len() {
            let (actual1, expect1) = curr[j];
            if actual1 == expect1 {
                // 已经相同了, 就不再交换出去了
                continue;
            } else if actual1 == expect {
                curr[position].0 = actual1;
                curr[j].0 = actual;
                if visited.insert(curr.clone()) {
                    // 去重
                    pq.push(State {
                        cost: cost + 1,
                        position: position + 1,
                        curr: curr.clone(),
                    });
                }
                // 换回来
                curr[position].0 = actual;
                curr[j].0 = actual1;
            }
        }
    }
    unreachable!("题目保证必然可以交换得到, 因此前面的A*能保证已经返回")
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Testcase {
                s1: "aabbccddee",
                s2: "dcacbedbae",
                expect: 5,
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
}
