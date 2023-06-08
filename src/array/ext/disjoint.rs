//! # Disjoint-set data structure
//!
//! wiki: [Disjoint-set data structure](https://en.wikipedia.org/wiki/Disjoint-set_data_structure)
//!
//! 效果: 将数据分组
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [547. 省份数量](find_circle_num)
//!     * [684. 冗余连接](find_redundant_connection)
//!     * [990. 等式方程的可满足性](equations_possible)
//!     * [841. 钥匙和房间](can_visit_all_rooms)
//! * 困难
//!     * [839. 相似字符串组](num_similar_groups)
//!

struct UnionFind {
    count: usize,                           // 连通分量的个数, 总共分成了几组
    parent: std::cell::RefCell<Vec<usize>>, // 记录每个节点的父节点，父节点为自身的是根节点
    size: Vec<usize>,                       // 记录每个连通分量的大小
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            count: size,
            parent: std::cell::RefCell::new((0..size).collect()),
            size: vec![1; size],
        }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn find(&self, p: usize) -> usize {
        let mut root = p;
        while root != self.parent.borrow()[root] {
            root = self.parent.borrow()[root];
        }
        let mut p = p;
        while p != root {
            let next = self.parent.borrow()[p];
            self.parent.borrow_mut()[p] = root;
            p = next;
        }
        root
    }

    pub fn is_connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
    pub fn connect(&mut self, p: usize, q: usize) {
        let (p_root, q_root) = (self.find(p), self.find(q));
        if p_root == q_root {
            return;
        }
        if self.size[p_root] < self.size[q_root] {
            self.parent.borrow_mut()[p_root] = q_root;
            self.size[q_root] += self.size[p_root];
        } else {
            self.parent.borrow_mut()[q_root] = p_root;
            self.size[p_root] += self.size[q_root];
        }
        self.count -= 1;
    }
}

/// [547. 省份数量](https://leetcode.cn/problems/number-of-provinces/)
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut uf = UnionFind::new(is_connected.len());
    is_connected.into_iter().enumerate().for_each(|(i, line)| {
        line.into_iter().enumerate().for_each(|(j, x)| {
            if x == 1 {
                uf.connect(i, j);
            }
        })
    });
    uf.count() as i32
}

/// [684. 冗余连接](https://leetcode.cn/problems/redundant-connection/)
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut uf = UnionFind::new(edges.len() + 1);
    let mut curr_count = uf.count();
    let mut last_edge = vec![];

    for edge in edges {
        if let &[p, q] = edge.as_slice() {
            uf.connect(p as usize, q as usize);
            if uf.count() == curr_count {
                last_edge = edge.clone();
            }
            curr_count = uf.count;
        }
    }
    last_edge
}

/// [990. 等式方程的可满足性](https://leetcode.cn/problems/satisfiability-of-equality-equations/)
pub fn equations_possible(equations: Vec<String>) -> bool {
    let mut uf = UnionFind::new(26);

    let mut not_equal = vec![]; // 暂存

    for e in equations {
        let s = e.as_bytes();
        let (a, b, eq) = (s[0] - b'a', s[3] - b'a', s[1] == b'=');
        if eq {
            uf.connect(a as usize, b as usize);
        } else {
            not_equal.push((a as usize, b as usize));
        }
    }

    for (a, b) in not_equal {
        if uf.is_connected(a, b) {
            return false;
        }
    }
    true
}

/// [839. 相似字符串组](https://leetcode.cn/problems/similar-string-groups/)
pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    fn is_similar(s1: &str, s2: &str) -> bool {
        s1.chars().zip(s2.chars()).filter(|(a, b)| !a.eq(b)).count() <= 2
    }
    let mut uf = UnionFind::new(strs.len());

    for i in 0..strs.len() {
        let s1 = strs.get(i).unwrap();
        for j in i + 1..strs.len() {
            if uf.is_connected(i, j) {
                continue;
            }

            let s2 = strs.get(j).unwrap();
            if is_similar(s1, s2) {
                uf.connect(i, j);
            }
        }
    }
    uf.count() as i32
}

/// [841. 钥匙和房间](https://leetcode.cn/problems/keys-and-rooms/)
///
/// 关键信息: 除 `0` 号房间外的其余所有房间都被锁住
///
/// 解法1: bfs
/// ```
/// pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
///     use std::collections::HashSet;
///     use std::collections::VecDeque;
///
///     let mut queue = VecDeque::new();
///     let mut visited = HashSet::new();
///
///     queue.push_back(0);
///     visited.insert(0);
///     while let Some(n) = queue.pop_front() {
///         let room = rooms.get(n).unwrap();
///         for &r in room.iter(){
///             if !visited.insert(r) {
///                 continue;
///             }
///             queue.push_back(r as usize)
///         }
///     }
///     visited.len() == rooms.len()
/// }
/// ```
///
/// 解法二: 并查集
///
/// 由于 `HashSet` 这里也只是用了 `insert` 和 `contain` 两个方法, 因此换成 并查集判定也是OK的.
/// 甚至内存表现上, 并查集更优.
pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;
    let mut uf = UnionFind::new(rooms.len());

    let mut queue = VecDeque::new();

    queue.push_back(0);
    while let Some(curr) = queue.pop_front() {
        let room = rooms.get(curr).unwrap();
        for &r in room.iter() {
            if uf.is_connected(curr, r as usize) {
                continue;
            }
            uf.connect(curr, r as usize);
            queue.push_back(r as usize);
        }
    }
    uf.count() == 1
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_can_visit_all_rooms() {
        struct Testcase {
            rooms: Vec<Vec<i32>>,
            expect: bool,
        }

        vec![
            Testcase {
                rooms: vec2![[1], [2], [3], []],
                expect: true,
            },
            Testcase {
                rooms: vec2![[1, 3], [3, 0, 1], [2], [0]],
                expect: false,
            },
            Testcase {
                rooms: vec2![[1], [], [0, 3], [1]],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { rooms, expect } = testcase;
            let actual = can_visit_all_rooms(rooms);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_num_similar_groups() {
        struct TestCase {
            strs: Vec<&'static str>,
            expect: i32,
        }

        vec![
            TestCase {
                strs: vec!["tars", "rats", "arts", "star"],
                expect: 2,
            },
            TestCase {
                strs: vec!["omv", "ovm"],
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { strs, expect } = testcase;
            let strs = strs.into_iter().map(str::to_string).collect();
            let actual = num_similar_groups(strs);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_equations_possible() {
        struct TestCase {
            equations: Vec<&'static str>,
            expect: bool,
        }

        vec![
            TestCase {
                equations: vec!["a==b", "b!=a"],
                expect: false,
            },
            TestCase {
                equations: vec!["b==a", "a==b"],
                expect: true,
            },
            TestCase {
                equations: vec!["a==b", "b==c", "a==c"],
                expect: true,
            },
            TestCase {
                equations: vec!["a==b", "b!=c", "c==a"],
                expect: false,
            },
            TestCase {
                equations: vec!["c==c", "b==d", "x!=z"],
                expect: true,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { equations, expect } = testcase;
            let equations = equations.into_iter().map(str::to_string).collect();
            let actual = equations_possible(equations);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_find_redundant_connection() {
        struct Testcase {
            edges: Vec<Vec<i32>>,
            expect: Vec<i32>,
        }

        vec![
            Testcase {
                edges: vec2![[1, 2], [1, 3], [2, 3]],
                expect: vec![2, 3],
            },
            Testcase {
                edges: vec2![[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]],
                expect: vec![1, 4],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { edges, expect } = testcase;
            let actual = find_redundant_connection(edges);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_find_circle_num() {
        struct Testcase {
            is_connected: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                is_connected: vec2![[1, 1, 0], [1, 1, 0], [0, 0, 1]],
                expect: 2,
            },
            Testcase {
                is_connected: vec2![[1, 0, 0], [0, 1, 0], [0, 0, 1]],
                expect: 3,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase {
                is_connected,
                expect,
            } = testcase;
            let actual = find_circle_num(is_connected);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
