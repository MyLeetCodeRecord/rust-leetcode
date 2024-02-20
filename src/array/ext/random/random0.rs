//! 特点: 概率, 知道总长度
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [528. 按权重随机选择](mod@array)
//!     * [380. O(1) 时间插入、删除和获取随机元素](mod@set)

/// [528. 按权重随机选择](https://leetcode-cn.com/problems/random-pick-with-weight/)
///
/// 综合题: 前缀和 + 二分查找
///
/// 步骤:
/// 1. 根据权重数组`w`, 生成前缀和数组`preSum`
/// 2. 生成一个取值在 `preSum` 之内的随机数, 用二分搜索算法寻找大于等于这个随机数的最小元素索引
/// 3. 最后对这个索引减一(因为前缀和数组有一位索引偏移), 就可以作为权重数组的索引, 即最终答案
///
/// 上述步骤在实现上存在一些细节差别.
///
/// 实现1:
/// ```ignore
/// struct Solution {
///     prefix_sum: Vec<i32>,
/// }
///
/// impl Solution {
///     fn new(mut w: Vec<i32>) -> Self {
///         for i in 1..w.len() {
///             w[i] += w[i - 1];
///         }
///         Self { prefix_sum: w }
///     }
///
///     fn pick_index(&self) -> i32 {
///         use rand::{thread_rng, Rng};
///         let x = thread_rng().gen_range(1..self.prefix_sum.last().unwrap() + 1);
///         match self.prefix_sum.binary_search(&x) {
///             Ok(i) => i as i32,
///             Err(i) => i as i32,
///         }
///     }
/// }
/// ```
/// * 这个实现, 前缀和没有补前缀0, 长度没有变化,
/// * random范围是`[1, last]`, 起点不是0
///     * 比如第一个数权重3, 则随机到 `1, 2, 3`时就代表取第一个, 返回结果0
///     * 此时前缀和的第一个为3, 使用 `binary_search` 方法时, 会返回下面三种结果
///         * `Err(0)`, `Err(0)` 和 `Ok(0)`
///     * 因此不需要 减去1
///
/// 实现2:
/// * 有前缀补0
/// * 整体索引加1, 因此结果需要减一
///
/// ```ignore
/// struct Solution {
///     pre_sum: Vec<i32>,
/// }
/// impl Solution {
///     fn new(w: Vec<i32>) -> Self {
///         let mut pre = Vec::with_capacity(w.len() + 1);
///         let mut curr = 0;
///         pre.push(curr);
///         for _w in w {
///             curr += _w;
///             pre.push(curr);
///         }
///         Self { pre_sum: pre }
///     }
///     fn pick_index(&self) -> i32 {
///         use rand::{thread_rng, Rng};
///         let x = thread_rng().gen_range(1..=self.pre_sum.last().copied().unwrap());
///         match self.pre_sum.binary_search(&x) {
///             Ok(i) => i as i32- 1,
///             Err(i) => i as i32 - 1,
///         }
///     }
/// }
/// ```
///
/// 实现3:
/// 上面两种实现, 都依赖了 [Rand](https://crates.io/crates/rand) 这个外部 *crate*
/// 但是 leetcode 上支持的版本是`0.7.2`, 而随机函数`gen_range`的签名发生变化.
/// 其实题目只需要保证 **概率+权重**即可. 可以不用真随机
///
/// 思路
/// 1. 对每个下标被选中次数作计数
/// 2. 下标出现次数的比例应该与下标权重比例一致
/// 3. 每次选下标时, 选择选中次数/权重最小的下标. (选中时更新被选中计数并放回队列)
///
/// 但这样有一个问题, 就是 `pick_index` 的函数入参变为了`&mut self`
/// 为了保持签名不变, 可以用 [std::cell::RefCell].
///
pub mod array {
    use std::cell::RefCell;
    use std::cmp::{Ordering, Reverse};
    use std::collections::BinaryHeap;

    struct Solution {
        heap: RefCell<BinaryHeap<Reverse<Probability>>>,
    }

    #[allow(dead_code)]
    impl Solution {
        fn new(w: Vec<i32>) -> Self {
            let mut heap = BinaryHeap::with_capacity(w.len());
            for (i, &x) in w.iter().enumerate() {
                heap.push(Reverse(Probability {
                    weight: x,
                    count: 0,
                    idx: i,
                }));
            }
            Solution {
                heap: RefCell::new(heap),
            }
        }

        fn pick_index(&self) -> i32 {
            let mut x = { self.heap.borrow_mut().pop().unwrap().0 };
            x.count += 1;
            let ret = x.idx;
            self.heap.borrow_mut().push(Reverse(x));
            ret as i32
        }
    }

    #[derive(Eq)]
    struct Probability {
        weight: i32,
        count: i32,
        idx: usize,
    }

    impl std::cmp::Ord for Probability {
        fn cmp(&self, other: &Probability) -> Ordering {
            // 选中次数/权重, 分数转化为乘法比较
            let this = self.count * other.weight;
            let that = other.count * self.weight;
            this.cmp(&that)
        }
    }

    impl std::cmp::PartialOrd<Probability> for Probability {
        fn partial_cmp(&self, other: &Probability) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::cmp::PartialEq<Probability> for Probability {
        fn eq(&self, that: &Probability) -> bool {
            self.count * that.weight == that.count * self.weight
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pick_index() {
            struct TestCase {
                name: &'static str,
                w: &'static [i32],
                expect: i32,
            }

            [TestCase {
                name: "basic",
                w: &[1],
                expect: 0,
            }]
            .iter()
            .for_each(|testcase| {
                let s = Solution::new(testcase.w.to_vec());
                let actual = s.pick_index();
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// [380. O(1) 时间插入、删除和获取随机元素](https://leetcode-cn.com/problems/insert-delete-getrandom-o1/)
///
/// 关键点
/// * 查找的 0(1) 必然是哈希
/// * 访问的 O(1) 是哈希或者连续存储(支持随机访问)
pub mod set {
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct RandomizedSet {
        vals: RefCell<Vec<i32>>,
        val_to_index: RefCell<HashMap<i32, usize>>,
    }

    #[allow(dead_code)]
    impl RandomizedSet {
        fn new() -> Self {
            Self {
                vals: RefCell::new(Vec::new()),
                val_to_index: RefCell::new(HashMap::new()),
            }
        }

        fn insert(&self, val: i32) -> bool {
            if self.val_to_index.borrow().contains_key(&val) {
                return false;
            }
            self.val_to_index
                .borrow_mut()
                .insert(val, self.vals.borrow().len());
            self.vals.borrow_mut().push(val);
            true
        }

        fn remove(&self, val: i32) -> bool {
            if !self.val_to_index.borrow().contains_key(&val) {
                return false;
            }
            let idx_to_remove = self.val_to_index.borrow().get(&val).copied().unwrap();
            let idx_last = self.vals.borrow().len() - 1;
            let last_val = self.vals.borrow().last().copied().unwrap();

            self.vals.borrow_mut().swap(idx_to_remove, idx_last);
            *self
                .val_to_index
                .borrow_mut()
                .entry(last_val)
                .or_insert(idx_to_remove) = idx_to_remove;

            self.vals.borrow_mut().pop();
            self.val_to_index.borrow_mut().remove(&val);

            true
        }

        fn get_random(&self) -> i32 {
            let idx: usize = rand::random::<usize>();
            let length = self.vals.borrow().len();
            self.vals.borrow().get(idx % length).copied().unwrap()
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_randomized_set() {
            enum Op {
                Insert((i32, bool)),
                Remove((i32, bool)),
                Get(i32),
            }

            struct TestCase {
                name: &'static str,
                queries: &'static [Op],
            }

            [TestCase {
                name: "basic",
                queries: &[
                    Op::Insert((1, true)),
                    Op::Remove((2, false)),
                    Op::Insert((2, true)),
                    Op::Get(0),
                    Op::Remove((1, true)),
                    Op::Insert((2, false)),
                    Op::Get(0),
                ],
            }]
            .iter()
            .for_each(|testcase| {
                let set = RandomizedSet::new();
                for (idx, op) in testcase.queries.iter().enumerate() {
                    match *op {
                        Op::Insert((val, expect)) => {
                            let actual = set.insert(val);
                            assert_eq!(expect, actual, "{}-{} failed", testcase.name, idx);
                        }
                        Op::Remove((val, expect)) => {
                            let actual = set.remove(val);
                            assert_eq!(expect, actual, "{}-{} failed", testcase.name, idx);
                        }
                        Op::Get(_) => {
                            let _ = set.get_random();
                        }
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore]
    fn test_show_binary_search() {
        let v = [1, 2, 3, 4, 5, 6, 9];
        println!("1: {}", v.binary_search(&1).unwrap());
        println!("5: {}", v.binary_search(&5).unwrap());
        println!("9: {}", v.binary_search(&9).unwrap());

        println!("0: {}", v.binary_search(&0).unwrap_err());
        println!("8: {}", v.binary_search(&8).unwrap_err());
        println!("10: {}", v.binary_search(&10).unwrap_err());
    }
}
