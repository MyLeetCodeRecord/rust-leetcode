//! # two pointers 系列
//! ## 题目
//! * 简单
//!     * [876. 链表的中间结点](middle_node)
//!     * [83. 删除排序链表中的重复元素](delete_duplicates)
//!     * [21. 合并两个有序链表](merge_k_lists)   
//! * 中等
//!     * [19. 删除链表的倒数第 N 个结点](remove_nth_from_end)
//! * 困难:
//!     * [23. 合并K个升序链表](merge_k_lists)
//! * 没有rust模版的题:
//!     * [141. 环形链表](https://leetcode-cn.com/problems/linked-list-cycle/)
//!     * [160. 相交链表](https://leetcode-cn.com/problems/intersection-of-two-linked-lists/)

use datastructure::ListNode;

/// [876. 链表的中间结点](https://leetcode-cn.com/problems/middle-of-the-linked-list/)
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut slow_p = &mut head;
    let mut fast_p = &slow_p.clone();
    while fast_p.is_some() {
        let next = &fast_p.as_ref().unwrap().next;
        if next.is_none() {
            break;
        }
        slow_p = &mut slow_p.as_mut().unwrap().next;
        fast_p = &next.as_ref().unwrap().next;
    }
    slow_p.clone()
}

/// [83. 删除排序链表中的重复元素](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/)
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut head = head;
    let mut node = head.as_mut().unwrap();
    let mut duplicate = node.val;

    while let Some(next) = node.next.take() {
        // 先夺
        if next.val == duplicate {
            node.next = next.next;
        } else {
            duplicate = next.val; // 更新下次比较值
            node.next = Some(next); // 放回去
            node = node.next.as_mut().unwrap(); // 慢指针前进
        }
    }
    head
}

/// [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut slow_p = &mut dummy;
    let mut fast_p = &slow_p.clone();

    // 因为加了一个dummy节点, 这里是0..=n
    for _ in 0..=n {
        fast_p = &fast_p.as_ref().unwrap().next;
    }

    while fast_p.is_some() {
        fast_p = &fast_p.as_ref().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
    }

    let remove_p = &mut slow_p.as_mut().unwrap().next;
    slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();

    dummy.unwrap().next
}

/// [21. 合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/)
///
/// 递归写法
/// ```ignore
/// pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
///     if list1.is_none(){
///         return list2;
///     } else if list2.is_none(){
///         return  list1;
///     }
///
///     let (v1, v2) = (
///         list1.as_ref().unwrap().val,
///         list2.as_ref().unwrap().val
///     );
///
///     let mut head;
///     if v1 < v2{
///         head = list1;
///         let next = head.as_mut().unwrap().next.take();
///         head.as_mut().unwrap().next = merge_two_lists(next, list2);
///     } else {
///         head = list2;
///         let next = head.as_mut().unwrap().next.take();
///         head.as_mut().unwrap().next = merge_two_lists(next, list1);
///     }
///
///     head
/// }
/// ```
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list2;
    } else if list2.is_none() {
        return list1;
    }

    let mut head = Some(Box::new(ListNode { val: 0, next: None }));

    let (mut list1, mut list2) = (list1, list2);

    let mut curr = head.as_mut().unwrap();
    while list1.is_some() && list2.is_some() {
        // i32 Copy, 所以无借用
        let (v1, v2) = (list1.as_ref().unwrap().val, list2.as_ref().unwrap().val);

        if v1 < v2 {
            let tmp = list1.as_mut().unwrap().next.take(); // 把list1的next摘下来, 暂存
            curr.next = list1;
            list1 = tmp; // 更新list1, 指针向后一个
        } else {
            curr.next = list2;
            list2 = curr.next.as_mut().unwrap().next.take(); // 或者再取出
                                                             // 注意这里没有像其他语言一样 直接 list2 = list2.next, 因为list2的所有权已经转移
                                                             // 也可以像上面似的, 先用一个tmp承接
        }
        curr = curr.next.as_mut().unwrap(); // 整体指针后移一个
    }

    if list1.is_some() {
        curr.next = list1;
    }
    if list2.is_some() {
        curr.next = list2;
    }

    head.unwrap().next
}

/// [23. 合并K个升序链表](https://leetcode-cn.com/problems/merge-k-sorted-lists/)
///
/// 思路1: 使用小顶堆
/// ```ignore
/// pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
///     use std::cmp::Ordering;
///     #[derive(Debug, PartialEq, Eq)]
///     struct Wrapper {
///         node: Option<Box<ListNode>>,
///     }
///     impl Wrapper {
///         fn reverse_cmp(&self, other: &Self) -> Ordering {
///             // 这里将大小反转, 实现 core::cmp::Reverse 效果
///             // 这样在 BinaryHeap 中可以偷懒
///             match self
///                 .node
///                 .as_ref()
///                 .unwrap()
///                 .val
///                 .cmp(&other.node.as_ref().unwrap().val)
///             {
///                 Ordering::Greater => Ordering::Less,
///                 Ordering::Less => Ordering::Greater,
///                 Ordering::Equal => Ordering::Equal,
///             }
///         }
///     }
///
///     impl PartialOrd for Wrapper {
///         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///             Some(self.reverse_cmp(other))
///         }
///     }
///
///     impl Ord for Wrapper {
///         fn cmp(&self, other: &Self) -> Ordering {
///             self.reverse_cmp(other)
///         }
///     }
///
///     if lists.is_empty() {
///         return None;
///     }
///
///     use std::collections::BinaryHeap; // 默认是大顶堆
///     let mut heap = BinaryHeap::new();
///
///     lists.into_iter().for_each(|node| {
///         if node.is_some() {
///             heap.push(Wrapper { node });
///         }
///     });
///
///     let mut head = Some(Box::new(ListNode { val: 0, next: None }));
///     let mut curr = head.as_mut().unwrap();
///
///     while !heap.is_empty() {
///         let mut wrap = heap.pop().unwrap();
///         if wrap.node.is_none() {
///             continue;
///         }
///         let next = wrap.node.as_mut().unwrap().next.take();
///         curr.next = wrap.node;
///         if next.is_some() {
///             heap.push(Wrapper { node: next });
///         }
///
///         curr = curr.next.as_mut().unwrap();
///     }
///
///     head.unwrap().next
/// }
/// ```
/// 缺点就是代码太多， 还需要加一层wrapper才能存入heap
///
/// 思路2: 两两合并
/// ```ignore
/// pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
///     lists.into_iter().fold(None, |r, list|{
///         merge_two_lists(r, list)
///     })
/// }
/// ```
///
/// 思路3: 既然能两两合并， 那分治也是可以的
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    while lists.len() > 1 {
        lists = lists
            .chunks(2)
            .into_iter()
            .map(|x| {
                if x.len() >= 2 {
                    merge_two_lists(x[0].clone(), x[1].clone())
                } else if x.len() >= 1 {
                    x.split_first().unwrap().0.clone()
                } else {
                    unreachable!()
                }
            })
            .collect();
    }

    lists.pop().unwrap_or(None)
}
#[cfg(test)]
mod test {
    use super::*;
    use macros::list;

    #[test]
    fn test_merge_k_lists() {
        struct TestCase {
            lists: Vec<Option<Box<ListNode>>>,
            expect: Option<Box<ListNode>>,
        }

        vec![
            TestCase {
                lists: vec![list!([1, 4, 5]), list!([1, 3, 4]), list!([2, 6])],
                expect: list!([1, 1, 2, 3, 4, 4, 5, 6]),
            },
            TestCase {
                lists: vec![],
                expect: list!([]),
            },
            TestCase {
                lists: vec![list!([])],
                expect: list!([]),
            },
            TestCase {
                lists: vec![list![], list![]],
                expect: list![],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { lists, expect } = testcase;
            let actual = merge_k_lists(lists);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_merge_two_lists() {
        struct TestCase {
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
            expect: Option<Box<ListNode>>,
        }

        vec![
            TestCase {
                list1: list!([1, 2, 4]),
                list2: list!([1, 3, 4]),
                expect: list!([1, 1, 2, 3, 4, 4]),
            },
            TestCase {
                list1: list!([]),
                list2: list!([]),
                expect: list!([]),
            },
            TestCase {
                list1: list!([]),
                list2: list!([0]),
                expect: list!([0]),
            },
            TestCase {
                list1: list!([0]),
                list2: list!([]),
                expect: list!([0]),
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                list1,
                list2,
                expect,
            } = testcase;
            let actual = merge_two_lists(list1, list2);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_remove_nth_from_end() {
        struct TestCase {
            head: Option<Box<ListNode>>,
            n: i32,
            expect: Option<Box<ListNode>>,
        }

        vec![
            TestCase {
                head: list!([1, 2, 3, 4, 5]),
                n: 2,
                expect: list![1, 2, 3, 5],
            },
            TestCase {
                head: list![1],
                n: 1,
                expect: list![],
            },
            TestCase {
                head: list![1, 2],
                n: 1,
                expect: list![1],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { head, n, expect } = testcase;
            let actual = remove_nth_from_end(head, n);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_delete_duplicates() {
        struct TestCase {
            head: Option<Box<ListNode>>,
            expect: Option<Box<ListNode>>,
        }

        vec![
            TestCase {
                head: list!([1, 1, 2, 3, 3]),
                expect: list!([1, 2, 3]),
            },
            TestCase {
                head: list!([]),
                expect: list!([]),
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { head, expect } = testcase;
            let actual = delete_duplicates(head);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_middle_node() {
        struct TestCase {
            head: Option<Box<ListNode>>,
            expect: Option<Box<ListNode>>,
        }

        vec![
            TestCase {
                head: list!([1, 2, 3, 4, 5]),
                expect: list!([3, 4, 5]),
            },
            TestCase {
                head: list!([1, 2, 3, 4, 5, 6]),
                expect: list!([4, 5, 6]),
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { head, expect } = testcase;
            let actual = middle_node(head);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }
}
