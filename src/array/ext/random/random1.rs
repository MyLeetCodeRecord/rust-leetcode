//! 特点: 不知总长度的等概率
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [382. 链表随机节点](mod@linked)
//!     * [398. 随机数索引](mod@array)
//!

/// [398. 随机数索引](https://leetcode-cn.com/problems/random-pick-index/)
pub mod array {
    use rand;
    use std::collections::HashMap;

    struct Solution {
        mark: HashMap<i32, Vec<usize>>,
    }
    #[allow(dead_code)]
    impl Solution {
        fn new(nums: Vec<i32>) -> Self {
            let mut mark = HashMap::new();
            for (idx, &num) in nums.iter().enumerate() {
                let entry = mark.entry(num).or_insert(vec![]);
                entry.push(idx);
            }
            Self { mark }
        }

        fn pick(&self, target: i32) -> i32 {
            let x = self.mark.get(&target).unwrap();
            let idx = rand::random::<usize>() % x.len();
            x.get(idx).copied().unwrap() as i32
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pick() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
            }

            [TestCase {
                name: "basic",
                nums: &[1, 2, 3, 3, 3],
            }]
            .iter()
            .for_each(|testcase| {
                let s = Solution::new(testcase.nums.to_vec());
                let actual = s.pick(1);
                assert_eq!(0, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// [382. 链表随机节点](https://leetcode-cn.com/problems/linked-list-random-node/)
pub mod linked {
    use datastructure::ListNode;
    use rand::{thread_rng, Rng};
    use std::cell::RefCell;

    struct Solution {
        list: Option<Box<ListNode>>,
        cnt: RefCell<usize>,
    }

    #[allow(dead_code)]
    impl Solution {
        fn new(head: Option<Box<ListNode>>) -> Self {
            Self {
                list: Some(Box::new(ListNode { val: 0, next: head })),
                cnt: RefCell::new(0),
            }
        }

        fn get_random(&self) -> i32 {
            let old = *self.cnt.borrow();
            *self.cnt.borrow_mut() += 1;
            loop {
                let mut head = &self.list.as_ref().unwrap().next;
                while head.is_some() {
                    let x = thread_rng().gen_range(0..=old);
                    if x == old {
                        return head.as_ref().unwrap().val;
                    }
                    head = &head.as_ref().unwrap().next;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use macros::list;

        #[test]
        fn test_get_random() {
            struct TestCase {
                name: &'static str,
                head: Option<Box<ListNode>>,
                expect: i32,
            }

            vec![TestCase {
                name: "basic",
                head: list![1, 2, 3],
                expect: 1,
            }]
            .into_iter()
            .for_each(|testcase| {
                let TestCase { name, head, expect } = testcase;
                let s = Solution::new(head);
                let actual = s.get_random();
                assert_eq!(expect, actual, "{} failed", name);
            });
        }
    }
}
