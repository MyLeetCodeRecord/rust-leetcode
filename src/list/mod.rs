//! 链表题目
//! 
//! 由于rust写链表是真的难受, 因此将链表相关的题目汇总到这里.
//! 当然也会右很多其他范式的题目, 比如[two pointers](crate::array::ser::two_pointers).
//! 这种题目, 会在文档部分补上连接, 关联起来.

use datastructure::ListNode;

/// [203. 移除链表元素](https://leetcode.cn/problems/remove-linked-list-elements/)
///
/// 不使用 dummy 节点
/// ```ignore
/// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
///     // 保留头指针用于返回
///     let mut head = head;
///     // 创建游标， "引用"自身这个变量要可变， 可赋值。 同时是对原有节点的 `可变借用`， 以便对节点进行操作
///     let mut curr = &mut head;
///     // 这里没有使用 `while let Some(con)` 的写法, 因为这里不论是`Some(con)`还是`Some(ref con)`,借用的变量就难以处理了, 会影响下面是否可变
///     while curr.is_some() {
///         // 这时不需要对curr内部的值做变化, 所以使用 `as_ref` 足以
///         if curr.as_ref()
///              .unwrap()
///              .val == val
///         {
///             // curr 是对一个option的可变引用,
///             // 由于val相等, 需要丢弃curr, 因此deref, in-place修改, 对应上面`&mut head;`的可变引用(否则不能就地修改)
///             // 前面节点还是指向这个节点, 只是节点的内容被替换了
///             *curr = curr.as_mut()
///                       .unwrap()
///                       .next
///                       .take();
///         } else {
///             // val不等, 游标后移
///             // 改变 curr 的值, 对应上面的 `let mut curr`
///             // 还是对option的可变引用, 因此需要`&mut`的标注
///             curr = &mut curr.as_mut()
///                         .unwrap()
///                         .next;
///         }
///     }
///     head
/// }
/// ```
/// 
/// 递归写法
/// ```ignore
/// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
///     if head.is_none(){
///         return None;
///     }
///     let mut head = head;
///     let h = head.as_mut().unwrap();
///     if h.val == val{
///         return remove_elements(h.next.take(), val);
///     } else {
///         h.next = remove_elements(h.next.take(), val);
///     }
///     return head;
/// }
/// ```
///
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };
    let mut curr = &mut dummy;
    while curr.next.is_some() {
        let mut nxt = curr.next.take().unwrap(); // 先抢占， 不行再还回去
        if nxt.val == val {
            curr.next = nxt.next.take();
        } else {
            curr.next = Some(nxt);
            curr = curr.next.as_mut().unwrap();
        }
    }
    dummy.next
}

#[cfg(test)]
mod test{
    use super::*;
    use macros::list;

    #[test]
    fn test_remove_elements(){
        struct Testcase{
            head: Option<Box<ListNode>>,
            val: i32,
            expect: Option<Box<ListNode>>
        }

        vec![
            Testcase{
                head: list!([1,2,6,3,4,5,6]),
                val: 6,
                expect: list!([1,2,3,4,5]),
            },
            Testcase{
                head: list!([]),
                val: 1,
                expect: list!([])
            },
            Testcase{
                head: list!([7, 7, 7, 7]),
                val: 7,
                expect: list!([])
            }
        ].into_iter().enumerate().for_each(|(idx, testcase)|{
            let Testcase{head, val, expect} = testcase;
            let acutal = remove_elements(head, val);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }
}

pub mod ser;