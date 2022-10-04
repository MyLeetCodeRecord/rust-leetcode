/// [846. 一手顺子](https://leetcode.cn/problems/hand-of-straights/)
///
/// 重点: 连续, 因此可以直接+1, 尝试找后一个
///
/// [贪心解法]()
pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    if (hand.len() as i32) % group_size != 0 {
        // 张数不对
        return false;
    }
    let mut counter = HashMap::new();
    let mut pq = BinaryHeap::new();
    for card in hand {
        *counter.entry(card).or_insert(0) += 1;
        pq.push(Reverse(card));
    }

    while !pq.is_empty() {
        let start = pq.pop().unwrap().0;
        let entry = counter.entry(start).or_default();
        if *entry == 0 {
            // 起点已经被消耗了, 重新选一个
            continue;
        }
        *entry -= 1; //消耗一个
        for i in 1..group_size {
            // 因为连续, 所以可以枚举
            let e = counter.entry(start + i).or_default();
            if *e == 0 {
                // 凑不出连续
                return false;
            }
            *e -= 1;
        }
    }
    true
}

#[test]
fn test_is_n_straight_hand() {
    struct TestCase {
        hand: Vec<i32>,
        group_size: i32,
        expect: bool,
    }

    vec![
        TestCase {
            hand: vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            group_size: 3,
            expect: true,
        },
        TestCase {
            hand: vec![1, 2, 3, 4, 5],
            group_size: 4,
            expect: false,
        },
    ]
    .into_iter()
    .enumerate()
    .for_each(|(idx, testcase)| {
        let TestCase {
            hand,
            group_size,
            expect,
        } = testcase;
        let actual = is_n_straight_hand(hand, group_size);
        assert_eq!(expect, actual, "case {} failed", idx);
    });
}
