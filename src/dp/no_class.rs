/// [834. 树中距离之和](https://leetcode.cn/problems/sum-of-distances-in-tree/)
/// 标签: 树形DP
///
/// 思路1:
/// 每加入一个边, 就扫描整个图, 更新由这个边联通的节点之间的距离, 复杂度 $O(n^3)$, 然后超时
/// ```
/// pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
///     let n = n as usize;
///     let mut dp = vec![vec![i32::MAX; n]; n];
///     for edge in edges {
///         let (start, end) = (edge[0], edge[1]);
///         let (start, end) = (start as usize, end as usize);
///         dp[start][end] = 1;
///         dp[end][start] = 1;
///         for i in 0..n {
///             if i == start || i == end {
///                 continue;
///             }
///             // 两端并不一定相同, 这个地方会有遗漏, 比如 x -> start -> end -> y 的就更新不到, 导致节点连不起来
///             // start --> end --> i or start --> i
///             let end2i = dp[end][i];
///             let start2i = dp[start][i];
///             dp[start][i] = start2i.min(end2i.checked_add(1).unwrap_or(i32::MAX));
///             dp[i][start] = dp[start][i];
///             // end --> start --> i or end --> i
///             dp[end][i] = end2i.min(start2i.checked_add(1).unwrap_or(i32::MAX));
///             dp[i][end] = dp[end][i];
///
///             for j in 0..n {
///                 if i == j || j == start || j == end {
///                     continue;
///                 }
///                 // i -> start -> end -> y
///                 let (i2start, start2j) = (dp[i][start], dp[start][j]);
///                 let (i2end, end2j) = (dp[i][end], dp[end][j]);
///
///                 dp[i][j] = dp[i][j]
///                     .min(
///                         i2start.checked_add(start2j).unwrap_or(i32::MAX), // i --> start --> j
///                     )
///                     .min(
///                         i2end.checked_add(end2j).unwrap_or(i32::MAX), // i --> end --> j
///                     );
///                 dp[j][i] = dp[i][j];
///             }
///         }
///     }
///     // dbg!(&dp);
///     let mut ans = vec![0; n];
///     for i in 0..n {
///         let mut cnt = 0;
///         for j in 0..n {
///             if i == j {
///                 continue;
///             }
///             cnt += dp[i][j];
///         }
///         ans[i] = cnt;
///     }
///     ans
/// }
/// ```
///
pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    // 通过累加得到根节点0到其他所有节点的距离和以及每个节点作为跟节点时其子树的节点数
    fn post_order(
        ans: &mut Vec<i32>,
        cnt: &mut Vec<i32>,
        graph: &Vec<Vec<usize>>,
        child: usize,
        parent: usize,
    ) {
        for i in 0..graph[child].len() {
            if graph[child][i] != parent {
                post_order(ans, cnt, graph, graph[child][i], child);

                cnt[child] += cnt[graph[child][i]]; // 所有子节点数目汇总
                ans[child] = ans[child] + ans[graph[child][i]] + cnt[graph[child][i]];
            }
        }
    }
    fn pre_order(
        ans: &mut Vec<i32>,
        cnt: &mut Vec<i32>,
        graph: &Vec<Vec<usize>>,
        child: usize,
        parent: usize,
    ) {
        for i in 0..graph[child].len() {
            if parent != graph[child][i] {
                ans[graph[child][i]] =
                    ans[child] - cnt[graph[child][i]] + (ans.len() as i32) - cnt[graph[child][i]];
                pre_order(ans, cnt, graph, graph[child][i], child);
            }
        }
    }

    let n = n as usize;

    let mut ans = vec![0; n];
    let mut cnt = vec![1; n]; // 统计自己, 默认大小为1
    let mut graph = vec![vec![]; n];

    for edge in edges {
        let (start, end) = (edge[0] as usize, edge[1] as usize);
        graph[start].push(end); // 记录 节点A可以到的其他节点
        graph[end].push(start);
    }

    post_order(&mut ans, &mut cnt, &graph, 0, n + 1);
    pre_order(&mut ans, &mut cnt, &graph, 0, n + 1);

    ans
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_sum_of_distances_in_tree() {
        struct TestCase {
            n: i32,
            edges: Vec<Vec<i32>>,
            expect: Vec<i32>,
        }

        vec![
            TestCase {
                n: 6,
                edges: vec2![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]],
                expect: vec![8, 12, 6, 10, 10, 10],
            },
            TestCase {
                n: 1,
                edges: vec2![],
                expect: vec![0],
            },
            TestCase {
                n: 2,
                edges: vec2![[1, 0]],
                expect: vec![1, 1],
            },
            TestCase {
                n: 4,
                edges: vec2![[2, 0], [3, 1], [2, 1]],
                expect: vec![6, 4, 4, 6],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { n, edges, expect } = testcase;
            let actual = sum_of_distances_in_tree(n, edges);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
