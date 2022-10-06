//! DFS
//! 

/// [854. 相似度为 K 的字符串](https://leetcode.cn/problems/k-similar-strings/)
/// 
/// - [BFS解法](crate::graph_search::bfs::k_similarity)
/// - DFS解法
/// - [A*启发式解法](crate::graph_search::a_start::k_similarity)
pub fn k_similarity(s1: String, s2: String) -> i32 {
    // (现在, 期望是啥)
    let mut diff = s1
        .as_bytes()
        .iter()
        .zip(s2.as_bytes().iter())
        .filter_map(|(&a, &b)| if a != b { Some((a, b)) } else { None })
        .collect::<Vec<(u8, u8)>>();

    if diff.is_empty() {
        // 没有不同, 直接返回
        // 如果长度是2或3, 也可以直接返回, 不过可以不用特殊处理
        return 0;
    }

    fn dfs(ans: &mut i32, cost: i32, idx: usize, diff: &mut Vec<(u8, u8)>) {
        if cost > *ans {
            // 剪枝1
            return;
        }
        let diff_count = diff.iter().skip(idx).filter(|a| a.0 != a.1).count() as i32;
        if diff_count == 0 {
            // 已经全相同了, 更新结果
            if *ans > cost {
                *ans = cost;
            }
            return;
        }
        let min_swap = (diff_count + 1) / 2;
        if cost + min_swap >= *ans {
            // 剪枝2
            return;
        }
        for j in idx..diff.len() {
            let (curr, expect) = diff[j];
            if curr == expect {
                // 跳过
                continue;
            }
            for k in j + 1..diff.len() {
                let (curr1, expect1) = diff[k];
                if curr1 == expect1 {
                    // 剪枝3, 如果已经相同, 不用再交换
                    continue;
                }
                if curr1 == expect {
                    diff[k].0 = curr; // 交换
                    dfs(ans, cost + 1, j + 1, diff); // 前面j个已经相同
                    diff[k].0 = expect; // 复原
                }
            }
            // 不是为了循环, 而是为了跳过那些已经相同的
            break;
        }
    }
    let mut ans = diff.len() as i32;
    dfs(&mut ans, 0, 0, &mut diff);
    ans
}

#[cfg(test)]
mod tests{
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