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
    let mut uf = UnionFind::new(edges.len()+1);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec2;

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
