/// # 二分查找
/// 特点:
/// 1. 数组(连续内存, 或可用索引随机访问) + 有序
/// 2. 时间复杂度O(logn)
///
/// 看到要求O(logn)的, 多和二分有关
///
/// ## 基础 - 区间选择 - 写法
///
/// > 没有重复元素
///
/// 之所以有区间的写法问题, 是因为口语上的左边一半右边一半, 没有描述清边界问题.
/// 但其实把握住: 终止条件为 **搜索区间内没有元素** 这一条就可以了.
///
/// 升序数组, 查找
/// * 左闭右闭: 初始化用 `left, right = 0, len(nums)-1`
///     * `left`, `right`都是索引值
///     * `left`, `right`包含在搜索区间,
///         * 因此如果判定 `target < nums[mid]`, 则`right = mid -1`, 不能`right = mid` 否则区间多一个
///         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
///         * 终止条件为`while left <= right` 有等号
///     * 注意:
///         * 由于存在`right = mid -1`, 因此存在`right`小于0的可能(就一个元素时)
///         * 终止条件为 **搜索区间内没有元素**, 即`left > right`;
///             * `left == right`时还包含一个元素
/// * 左闭右开: 初始化用 `left, right = 0, len(nums)`
///     * `left`为索引, `right`为右边界
///     * `left`在搜索区间, `right`不在, `right-1`在
///         * 因此如果判定 `target < nums[mid]`, 则`right = mid`, 不能`right = mid-1` 否则区间少一个
///         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
///         * 终止条件为`while left < right`
///     * 注意:
///         * 终止条件为 **搜索区间内没有元素**,
///         * `right-left`为区间内元素的个数, `left==right`时就没有元素了
/// * 左开右开: 初始化用 `left, right = 1, len(nums)`
///     * `left`, `right`为第*x*个中的*x*
///     * `left`, `right`的含义不再是索引, 而是字面上的第几个
///         * 和*左闭右闭*相似, 等效于同时加1
///         * 因此如果判定 `target < nums[mid]`, 则`right = mid -1`, 表示不再检查第`mid`个
///         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
///         * 终止条件为`while left <= right` 有等号
///     * 注意:
///         * 由于字面意义的顺序比索引大1, 取值时注意减去
///         * `right`不存在比0小的可能
/// * 左开右闭: 没啥意义, 不考虑
///
/// ## 扩展 - 重复
///
/// > 有重复元素, 查找第一个/最后一个出现的位置
///
/// 与基础的区别在于找到目标元素后, 不是立刻`return`, 而是更改边界,
///
/// 升序数组, 寻找第一个出现位置:
/// * 左闭右闭: 初始化用 `left, right = 0, len(nums)-1`
///     * `left`, `right`在搜索区间,
///         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
///         * `target < nums[mid]`, 则`right = mid-1` 与基础二分相同
///     * 在`target == num[mid]`的处理上有两种方式,
///         * 表示第一次出现至少不在`mid`右边, 令 `right = mid`, `mid`可能就是最早出现的那个, 需要保留为候选
///             * 这时, 不在是*搜索区间*, 而是*候选区间*, 变为至多保留一个目标值,
///             * 循环变为`while left < right` 保证至少保留一个
///             * 同时需要在`right = left + 1`时需要手动判定跳出, 否则死循环
///             * 使用`left`值即可(此时`left`==`right`)
///         * `right = mid - 1`
///             * 如果`mid`为第一个, 则`[0, mid-1]`一定都比`nums[mid]`小, 最终`left == mid`跳出循环
///             * 循环使用`while left <= right` 保证每一个都搜索到
///             * 使用`left`值即可
///     * 注意:
///         * `right = mid - 1` 存在 -1的可能
///         * 将`target < nums[mid]` 和 `target == nums[mid]`合并, 统一`right = mid-1`比较方便
///
///  * 左闭右开: 初始化用 `left, right = 0, len(nums)`
///     * `left`在搜索区间, `right`不在搜索区间, `right-1`在
///         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
///         * `target < nums[mid]`, 则`right = mid` 与基础二分相同
///     * `target == num[mid]`的处理上有两种方式,
///         * 令`right = mid+1`, 第一次出现至少不在`mid`右边, 因为可能第一次出现就是`mid`, 保留作为候选
///             * 此时候选区间定义变化, 不再是搜索区间,
///             * 循环条件变为`while left < right-1`, 保证至少保留一个
///             * 同时需要在`right = left + 2`时需要手动判定跳出, 否则死循环
///             * 使用`right-1`作为可能值, 可能负数
///         * 令`right = mid`
///             * 如果`mid`为第一个, 则`[0, mid)`一定都比`nums[mid]`小, 最终`left == mid`跳出循环
///             * 循环条件为 `while left < right`, 保证每一个都搜索到
///             * 使用`left`值即可
///     * 注意:
///         * `while left < right-1` 可转化为`while left+1 < right`, 否则可能会有负数
///         * 将`target < nums[mid]` 和 `target == nums[mid]`合并, 统一`right = mid`比较方便
/// * 左开右开: 初始化用 `left, right = 1, len(nums)`
///     * `left`, `right`的含义不再是索引, 而是字面上的第几个
///         * 和*左闭右闭*相似, 等效于同时加1
///         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
///         * `target < nums[mid]`, 则`right = mid-1` 与基础二分相同
///     * `target == num[mid]`的处理上有两种方式
///
///
/// 总结起来,
/// * 还是维持**搜索空间** 比较好处理.
/// * *左开右开* 对于空数组有奇效
/// * `left` `right`含义相同(同为索引, 或同为第x个)时,
///     * `while` 包含`left == right`的情况, 其他不包含
///     * 切换`right`时需要, `right = mid -1`
///
/// ## 扩展 - 单调函数
///
/// 前面 对`nums[mid]`和`target`判大小, 其实用数学描述即为
/// $$
/// f(i) = nums_{i} >= target
///      = \begin{cases}
///            False, & i < target_{first} ,\\
///            True,  & i \ge target_{first}
///        \end{cases}
/// $$
///
/// 即在左区间为`0`, 右区间为`1`, 这样就构成了一个 **二分**.
///
/// 因此利用该特性, 结合上面的边界移动, 即可完整一般形式的 二分搜索.
///
/// ## 题目链接
///
/// * 简单:
///     * [704. 二分查找](https://leetcode-cn.com/problems/binary-search/)
///     * [35. 搜索插入位置](https://leetcode-cn.com/problems/search-insert-position/)
/// * 中等:
///     * [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
///     * [875. 爱吃香蕉的珂珂](https://leetcode-cn.com/problems/koko-eating-bananas/)
///     * [1011. 在 D 天内送达包裹的能力](https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days/)
///
pub mod binary_search {

    /// [704. 二分查找](https://leetcode-cn.com/problems/binary-search/)
    ///
    /// `rust`的限制: `usize` 大于等于0, 对`0usize` 减一会溢出, 直接变为`usize::MAX`
    ///  因此*左闭右闭*不适用.
    ///  
    #[rustfmt::skip]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (1, nums.len());
        while left <= right {
            let mid = (left + right) / 2;
            match target.cmp(nums.get(mid-1).unwrap()) {
                Ordering::Equal   => { return mid as i32 -1; }
                Ordering::Greater => { left = mid + 1;       }
                Ordering::Less    => { right = mid - 1;      }
            }
        }
        -1
    }

    /// [35. 搜索插入位置](https://leetcode-cn.com/problems/search-insert-position/)
    #[rustfmt::skip]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (1, nums.len());
        while left <= right {
            let mid = (left + right) / 2;
            match target.cmp(nums.get(mid-1).unwrap()) {
                Ordering::Equal   => { return mid as i32 -1; }
                Ordering::Greater => { left = mid + 1;       }
                Ordering::Less    => { right = mid - 1;      }
            }
        }
        left as i32 -1
    }

    /// [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        if nums.is_empty() {
            return vec![-1, -1];
        }

        let first_find = |nums: &[i32], target: i32| -> i32 {
            let (mut left, mut right) = (0, nums.len());
            while left < right {
                let mid = (right + left) / 2;
                match target.cmp(nums.get(mid).unwrap()) {
                    Ordering::Greater => {
                        left = mid + 1;
                    }
                    _ => {
                        right = mid;
                    }
                }
            }
            if let Some(&num) = nums.get(left) {
                if num == target {
                    return left as i32;
                }
            }
            -1
        };

        let last_find = |nums: &[i32], target: i32| -> i32 {
            let (mut left, mut right) = (1, nums.len());
            while left <= right {
                let mid = (right + left) / 2;
                match target.cmp(nums.get(mid - 1).unwrap()) {
                    Ordering::Less => {
                        right = mid - 1;
                    }
                    _ => {
                        left = mid + 1;
                    }
                }
            }
            if right == 0 {
                return -1;
            }

            if let Some(&num) = nums.get(right - 1) {
                if num == target {
                    return right as i32 - 1;
                }
            }
            -1
        };

        vec![first_find(&nums, target), last_find(&nums, target)]
    }

    /// [875. 爱吃香蕉的珂珂](https://leetcode-cn.com/problems/koko-eating-bananas/)
    ///
    /// * h 必然大于 堆数, 否则不可能吃完. 题目保证
    /// * 至少需要 堆数 个小时
    ///
    /// 题目是在求, 能达到这个效果的最小值, 即左边界
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let possible = |piles: &[i32], each: i32| -> bool {
            piles
                .iter()
                .map(|&p| {
                    if p % each == 0 {
                        p / each
                    } else {
                        p / each + 1 // 不足的需要额外吃一次
                    }
                })
                .sum::<i32>()
                <= h
        };

        let (mut left, mut right) = (1, piles.iter().max().copied().unwrap());

        while left <= right {
            let mid = (left + right) / 2;
            if possible(&piles, mid) {
                // 用时少, 一次吃太多, 少吃一点
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    /// [1011. 在 D 天内送达包裹的能力](https://leetcode-cn.com/problems/capacity-to-ship-packages-within-d-days/)
    ///
    /// * 单次最少容量, 也得能把包裹里面那个最终那个放上, 否则上不去.
    /// * 按顺序一次上传.
    /// * 最多一次全装上
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let possible = |weights: &[i32], each: i32| -> bool {
            let mut sum = 1; // 不管有多少, 一次运多少, 至少占用一次
            let mut remain = each;
            let mut curr = 0;
            while curr < weights.len() {
                let weight = weights.get(curr).copied().unwrap();
                if remain >= weight {
                    remain -= weight;
                    curr += 1;
                } else {
                    sum += 1;
                    remain = each;
                }
            }
            sum <= days
        };

        let (mut left, mut right) = (
            weights.iter().max().copied().unwrap(),
            weights.iter().sum::<i32>(),
        );
        while left <= right {
            let mid = (left + right) / 2;
            if possible(&weights, mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_search() {
            struct TestCase {
                name: &'static str,
                nums: Vec<i32>,
                target: i32,
                expect: i32,
            }
            vec![
                TestCase {
                    name: "basic",
                    nums: vec![-1, 0, 3, 5, 9, 12],
                    target: 9,
                    expect: 4,
                },
                TestCase {
                    name: "basic 2",
                    nums: vec![-1, 0, 3, 5, 9, 12],
                    target: 2,
                    expect: -1,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = search(testcase.nums.clone(), testcase.target);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_search_insert() {
            struct TestCase {
                name: &'static str,
                nums: Vec<i32>,
                target: i32,
                expect: i32,
            }
            vec![
                TestCase {
                    name: "basic",
                    nums: vec![1, 3, 5, 6],
                    target: 5,
                    expect: 2,
                },
                TestCase {
                    name: "basic 2",
                    nums: vec![1, 3, 5, 6],
                    target: 2,
                    expect: 1,
                },
                TestCase {
                    name: "basic 3",
                    nums: vec![1, 3, 5, 6],
                    target: 7,
                    expect: 4,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = search_insert(testcase.nums.clone(), testcase.target);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_search_range() {
            struct TestCase {
                name: &'static str,
                nums: Vec<i32>,
                target: i32,
                expect: Vec<i32>,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: vec![5, 7, 7, 8, 8, 10],
                    target: 8,
                    expect: vec![3, 4],
                },
                TestCase {
                    name: "basic 2",
                    nums: vec![5, 7, 7, 8, 8, 10],
                    target: 6,
                    expect: vec![-1, -1],
                },
                TestCase {
                    name: "basic 3",
                    nums: vec![],
                    target: 0,
                    expect: vec![-1, -1],
                },
                TestCase {
                    name: "fix 1",
                    nums: vec![1],
                    target: 0,
                    expect: vec![-1, -1],
                },
                TestCase {
                    name: "fix 2",
                    nums: vec![2, 2],
                    target: 3,
                    expect: vec![-1, -1],
                },
                TestCase {
                    name: "fix 3",
                    nums: vec![1, 4],
                    target: 4,
                    expect: vec![1, 1],
                },
                TestCase {
                    name: "fix 4",
                    nums: vec![2, 2],
                    target: 2,
                    expect: vec![0, 1],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = search_range(testcase.nums.clone(), testcase.target);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_min_eating_speed() {
            struct TestCase {
                name: &'static str,
                piles: Vec<i32>,
                h: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    piles: vec![3, 6, 7, 11],
                    h: 8,
                    expect: 4,
                },
                TestCase {
                    name: "basic 2",
                    piles: vec![30, 11, 23, 4, 20],
                    h: 5,
                    expect: 30,
                },
                TestCase {
                    name: "basic 3",
                    piles: vec![30, 11, 23, 4, 20],
                    h: 6,
                    expect: 23,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = min_eating_speed(testcase.piles.clone(), testcase.h);
                assert_eq!(actual, testcase.expect, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_ship_within_days() {
            struct TestCase {
                name: &'static str,
                weights: Vec<i32>,
                days: i32,
                expect: i32,
            }
            vec![
                TestCase {
                    name: "basic",
                    weights: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                    days: 5,
                    expect: 15,
                },
                TestCase {
                    name: "basic 2",
                    weights: vec![3, 2, 2, 4, 1, 4],
                    days: 3,
                    expect: 6,
                },
                TestCase {
                    name: "basic 3",
                    weights: vec![1, 2, 3, 1, 1],
                    days: 4,
                    expect: 3,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = ship_within_days(testcase.weights.clone(), testcase.days);
                assert_eq!(actual, testcase.expect, "{} failed", testcase.name);
            })
        }
    }
}

/// # 双指针
/// > swap 不就是双指针吗
/// 特点: 就地; 相对
///
/// ## 概念
/// 双指针主要有 **快慢指针**, **左右指针**.
/// * 快慢指针: 两个指针同向而行，一快一慢
/// * 左右指针: 两个指针相向而行或者相背而行
///
/// 落到具体数据结构中
/// * 数组中用索引代替指针
/// * 单链表只有快慢指针
///
/// 其他形式的变种
/// * 滑动窗口
/// * 二分 可以视为左右指针
/// * 归并排序
///
/// ## 题目链接
/// * 简单:
///     * [27. 移除元素](https://leetcode-cn.com/problems/remove-element/)
///     * [26. 删除有序数组中的重复项](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/)
///     * [83. 删除排序链表中的重复元素](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/)
///     * [283. 移动零](https://leetcode-cn.com/problems/move-zeroes/)
///     * [344. 反转字符串](https://leetcode-cn.com/problems/reverse-string/)
///     * [977. 有序数组的平方](https://leetcode-cn.com/problems/squares-of-a-sorted-array/)
///     * [21. 合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/)
///     * [876. 链表的中间结点](https://leetcode-cn.com/problems/middle-of-the-linked-list/)
/// * 中等:
///     * [167. 两数之和 II - 输入有序数组](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)
///     * [5. 最长回文子串](https://leetcode-cn.com/problems/longest-palindromic-substring/)
///     * [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)
/// * 困难:
///     * [23. 合并K个升序链表](https://leetcode-cn.com/problems/merge-k-sorted-lists/)
/// * 没有rust模版的题:
///     * [141. 环形链表](https://leetcode-cn.com/problems/linked-list-cycle/)
///     * [160. 相交链表](https://leetcode-cn.com/problems/intersection-of-two-linked-lists/)
///
pub mod two_pointers {

    /// [27. 移除元素](https://leetcode-cn.com/problems/remove-element/)
    /// 索引 usize 可能溢出
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut from, mut end) = (0, nums.len() - 1);
        while from <= end {
            if val.eq(nums.get(end).unwrap()) {
                if let Some(x) = end.checked_sub(1) {
                    // 防止溢出
                    end = x;
                } else {
                    break;
                }
                continue;
            }
            if val.eq(nums.get(from).unwrap()) {
                nums.swap(from, end);
                end -= 1;
            }
            from += 1;
        }
        from as i32
    }

    /// [26. 删除有序数组中的重复项](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/)
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let (mut slow, mut fast) = (1, 1);
        let mut curr = nums.first().copied().unwrap();

        while let Some(&x) = nums.get(fast) {
            if x == curr {
                fast += 1;
                continue;
            }

            *nums.get_mut(slow).unwrap() = x;
            slow += 1;
            fast += 1;
            curr = x;
        }
        slow as i32
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
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

    /// [283. 移动零](https://leetcode-cn.com/problems/move-zeroes/)
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = {
            let mut idx = nums.len();
            // 找到第一个0, 作为起始插入点
            for (i, num) in nums.iter().enumerate() {
                if *num == 0 {
                    idx = i;
                    break;
                }
            }
            idx
        };
        let mut fast = slow;

        while slow < nums.len() && fast < nums.len() {
            let x = nums.get(fast).copied().unwrap();
            if x != 0 {
                *nums.get_mut(slow).unwrap() = x;
                slow += 1;
            }
            fast += 1;
        }
        // 将结尾置为0
        // while slow < nums.len(){
        //     *nums.get_mut(slow).unwrap() = 0;
        //     slow += 1;
        // }
        nums[slow..].fill(0);
    }

    /// [167. 两数之和 II - 输入有序数组](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let (mut left, mut right) = (1, numbers.len());
        while left < right {
            let l = numbers.get(left - 1).copied().unwrap();
            let r = numbers.get(right - 1).copied().unwrap();
            match target.cmp(&(l + r)) {
                Ordering::Equal => break,
                Ordering::Greater => left += 1,
                Ordering::Less => right -= 1,
            }
        }
        return vec![left as i32, right as i32];
    }

    /// [344. 反转字符串](https://leetcode-cn.com/problems/reverse-string/)
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    /// [5. 最长回文子串](https://leetcode-cn.com/problems/longest-palindromic-substring/)
    pub fn longest_palindrome(s: String) -> String {
        let palindrome = |s: &[char], l: usize, r: usize| -> String {
            let (mut l, mut r) = (l as isize, r as isize);
            while l >= 0
                && (r as usize) < s.len()
                && s.get(l as usize).unwrap().eq(s.get(r as usize).unwrap())
            {
                l -= 1; // 可能-1
                r += 1;
            }

            s[(l + 1) as usize..r as usize].into_iter().collect()
        };

        let chars: Vec<char> = s.chars().collect();

        let mut ret = "".to_string();
        for i in 0..chars.len() {
            let s1 = palindrome(&chars, i, i);
            let s2 = palindrome(&chars, i, i + 1);

            if s1.len() > ret.len() {
                ret = s1;
            }
            if s2.len() > ret.len() {
                ret = s2;
            }
        }
        ret
    }

    /// [977. 有序数组的平方](https://leetcode-cn.com/problems/squares-of-a-sorted-array/)
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums.len());
        let (mut left, mut right) = (1, nums.len());
        while left <= right {
            let l = nums.get(left - 1).copied().unwrap();
            let r = nums.get(right - 1).copied().unwrap();

            if l.abs() > r.abs() {
                ret.push(l * l);
                left += 1;
            } else {
                ret.push(r * r);
                right -= 1;
            }
        }
        ret.reverse();
        ret
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::cmp::Ordering;
        impl ListNode {
            fn reverse_cmp(&self, other: &Self) -> Ordering {
                // 这里将大小反转, 实现 core::cmp::Reverse 效果
                // 这样在 BinaryHeap 中可以偷懒
                match self.val.cmp(&other.val) {
                    Ordering::Greater => Ordering::Less,
                    Ordering::Less => Ordering::Greater,
                    Ordering::Equal => Ordering::Equal,
                }
            }
        }

        impl PartialOrd for ListNode {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.reverse_cmp(other))
            }
        }

        impl Ord for ListNode {
            fn cmp(&self, other: &Self) -> Ordering {
                self.reverse_cmp(other)
            }
        }

        if lists.is_empty() {
            return None;
        }

        use std::collections::BinaryHeap; // 默认是大顶堆

        let mut heap = BinaryHeap::from_iter(lists.into_iter());

        let mut head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut curr = head.as_mut().unwrap();

        while !heap.is_empty() {
            let mut node = heap.pop().unwrap();
            if node.is_none() {
                continue;
            }
            let next = node.as_mut().unwrap().next.take();
            curr.next = node;
            if next.is_some() {
                heap.push(next);
            }

            curr = curr.next.as_mut().unwrap();
        }

        head.unwrap().next
    }

    /// [19. 删除链表的倒数第 N 个结点](https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/)
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &slow_p.clone();

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_remove_element() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                val: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[3, 2, 2, 3],
                    val: 3,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[0, 1, 2, 2, 3, 0, 4, 2],
                    val: 2,
                },
                TestCase {
                    name: "fix 1",
                    nums: &[1],
                    val: 1,
                },
                TestCase {
                    name: "empty",
                    nums: &[],
                    val: 0,
                },
            ]
            .iter()
            .for_each(|testcase| {
                use std::collections::HashSet;

                let expect = {
                    let mut tmp = testcase.nums.iter().map(|x| *x).collect::<HashSet<i32>>();
                    tmp.remove(&testcase.val);
                    tmp
                };
                let expect_length = testcase.nums.iter().fold(0, |acc, &x| {
                    if x != testcase.val {
                        return acc + 1;
                    }
                    return acc;
                });

                let mut tmp = testcase.nums.to_vec();
                let length = remove_element(&mut tmp, testcase.val) as usize;

                assert_eq!(expect_length, length, "{} length not match", testcase.name);

                let actual = tmp[..length].iter().map(|x| *x).collect::<HashSet<i32>>();
                assert_eq!(expect, actual, "{} result not match", testcase.name);
            })
        }

        #[test]
        fn test_remove_duplicates() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[1, 1, 2],
                    expect: &[1, 2],
                },
                TestCase {
                    name: "basic 2",
                    nums: &[0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                    expect: &[0, 1, 2, 3, 4],
                },
                TestCase {
                    name: "basic 3",
                    nums: &[1],
                    expect: &[1],
                },
                TestCase {
                    name: "fix 1",
                    nums: &[1, 2],
                    expect: &[1, 2],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut tmp = testcase.nums.to_vec();
                let actual = remove_duplicates(&mut tmp) as usize;
                assert_eq!(testcase.expect.len(), actual, "{} failed", testcase.name);
                assert_eq!(testcase.expect, &tmp[..actual], "{} failed", testcase.name);
            });
        }

        fn build_list_from_slice(s: &[i32]) -> Option<Box<ListNode>> {
            if s.is_empty() {
                return None;
            }
            let head = Box::new(ListNode {
                val: s.first().copied().unwrap(),
                next: build_list_from_slice(&s[1..]),
            });
            Some(head)
        }

        #[test]
        fn test_delete_duplicates() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[1, 1, 2, 3, 3],
                    expect: &[1, 2, 3],
                },
                TestCase {
                    name: "cov 1",
                    nums: &[],
                    expect: &[],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let head = build_list_from_slice(testcase.nums);
                let actual = delete_duplicates(head);
                let expect = build_list_from_slice(testcase.expect);
                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_move_zeroes() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[0, 1, 0, 3, 12],
                    expect: &[1, 3, 12, 0, 0],
                },
                TestCase {
                    name: "basic 2",
                    nums: &[0],
                    expect: &[0],
                },
                TestCase {
                    name: "cov 1",
                    nums: &[1, 2, 0, 3, 4, 5],
                    expect: &[1, 2, 3, 4, 5, 0],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut tmp = testcase.nums.to_vec();
                move_zeroes(&mut tmp);
                assert_eq!(testcase.expect, tmp, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_two_sum() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                target: i32,
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[2, 7, 11, 15],
                    target: 9,
                    expect: &[1, 2],
                },
                TestCase {
                    name: "basic 2",
                    nums: &[2, 3, 4],
                    target: 6,
                    expect: &[1, 3],
                },
                TestCase {
                    name: "basic 3",
                    nums: &[-1, 0],
                    target: -1,
                    expect: &[1, 2],
                },
                TestCase {
                    name: "cov 1",
                    nums: &[1, 2, 3, 4, 5, 6],
                    target: 10,
                    expect: &[4, 6],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = two_sum(testcase.nums.to_vec(), testcase.target);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_reverse_string() {
            struct TestCase {
                name: &'static str,
                s: &'static [char],
                expect: &'static [char],
            }

            vec![
                TestCase {
                    name: "basic",
                    s: &['h', 'e', 'l', 'l', 'o'],
                    expect: &['o', 'l', 'l', 'e', 'h'],
                },
                TestCase {
                    name: "basic 2",
                    s: &['H', 'a', 'n', 'n', 'a', 'h'],
                    expect: &['h', 'a', 'n', 'n', 'a', 'H'],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut tmp = testcase.s.to_vec();
                reverse_string(&mut tmp);
                assert_eq!(testcase.expect, tmp, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_longest_palindrome() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                expect: &'static str,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "babad",
                    expect: "bab",
                },
                TestCase {
                    name: "basic 2",
                    s: "cbbd",
                    expect: "bb",
                },
                TestCase {
                    name: "fix 1",
                    s: "a",
                    expect: "a",
                },
            ]
            .iter()
            .for_each(|testcae| {
                let actual = longest_palindrome(testcae.s.to_string());
                assert_eq!(testcae.expect, actual, "{} failed", testcae.name);
            });
        }

        #[test]
        fn test_sorted_squares() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[-4, -1, 0, 3, 10],
                    expect: &[0, 1, 9, 16, 100],
                },
                TestCase {
                    name: "basic 2",
                    nums: &[-7, -3, 2, 3, 11],
                    expect: &[4, 9, 9, 49, 121],
                },
                TestCase {
                    name: "fix 1",
                    nums: &[1],
                    expect: &[1],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = sorted_squares(testcase.nums.to_vec());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_merge_two_lists() {
            struct TestCase {
                name: &'static str,
                list1: &'static [i32],
                list2: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    list1: &[1, 2, 4],
                    list2: &[1, 3, 4],
                    expect: &[1, 1, 2, 3, 4, 4],
                },
                TestCase {
                    name: "basic 2",
                    list1: &[],
                    list2: &[],
                    expect: &[],
                },
                TestCase {
                    name: "basic 3",
                    list1: &[],
                    list2: &[0],
                    expect: &[0],
                },
                TestCase {
                    name: "cov 1",
                    list1: &[0],
                    list2: &[],
                    expect: &[0],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let list1 = build_list_from_slice(testcase.list1);
                let list2 = build_list_from_slice(testcase.list2);
                let expect = build_list_from_slice(testcase.expect);
                let actual = merge_two_lists(list1, list2);
                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_merge_k_lists() {
            struct TestCase {
                name: &'static str,
                lists: &'static [&'static [i32]],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    lists: &[&[1, 4, 5], &[1, 3, 4], &[2, 6]],
                    expect: &[1, 1, 2, 3, 4, 4, 5, 6],
                },
                TestCase {
                    name: "basic 2",
                    lists: &[],
                    expect: &[],
                },
                TestCase {
                    name: "basic 3",
                    lists: &[&[]],
                    expect: &[],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let lists: Vec<Option<Box<ListNode>>> = testcase
                    .lists
                    .iter()
                    .map(|l| build_list_from_slice(*l))
                    .collect();
                let expect = build_list_from_slice(testcase.expect);
                let actual = merge_k_lists(lists);
                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_remove_nth_from_end() {
            struct TestCase {
                name: &'static str,
                head: &'static [i32],
                n: i32,
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    head: &[1, 2, 3, 4, 5],
                    n: 2,
                    expect: &[1, 2, 3, 5],
                },
                TestCase {
                    name: "basic 2",
                    head: &[1],
                    n: 1,
                    expect: &[],
                },
                TestCase {
                    name: "basic 3",
                    head: &[1, 2],
                    n: 1,
                    expect: &[1],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let head = build_list_from_slice(testcase.head);
                let expect = build_list_from_slice(testcase.expect);
                let actual = remove_nth_from_end(head, testcase.n);
                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_middle_node() {
            struct TestCase {
                name: &'static str,
                head: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    head: &[1, 2, 3, 4, 5],
                    expect: &[3, 4, 5],
                },
                TestCase {
                    name: "basic 2",
                    head: &[1, 2, 3, 4, 5, 6],
                    expect: &[4, 5, 6],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let head = build_list_from_slice(testcase.head);
                let expect = build_list_from_slice(testcase.expect);
                let actual = middle_node(head);
                assert_eq!(expect, actual, "{} failed", testcase.name);
            });
        }
    }
}

/// # 滑动窗口
///
/// 特点:
/// * 连续
/// * two pointers 的扩展
///
/// 主要点: 什么时机动哪个边界.
///
/// ## 题目链接
/// * 中等
///     * [2024. 考试的最大困扰度](https://leetcode-cn.com/problems/maximize-the-confusion-of-an-exam/)
///     * [1004. 最大连续1的个数 III](https://leetcode-cn.com/problems/max-consecutive-ones-iii/)
///     * [209. 长度最小的子数组](https://leetcode-cn.com/problems/minimum-size-subarray-sum/)
///     * [567. 字符串的排列](https://leetcode-cn.com/problems/permutation-in-string/)
///     * [3. 无重复字符的最长子串](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/)
///     * [904. 水果成篮](https://leetcode-cn.com/problems/fruit-into-baskets/)
/// * 困难
///     * [76. 最小覆盖子串](https://leetcode-cn.com/problems/minimum-window-substring/)
/// 
/// ## 总结
/// * 快慢指针的扩展, 即都是向一个方向跑的
/// * 题目都是要求连续的xxx, 常见的比如连续子串, 连续子数组, 即能组成一个窗口
/// * 核心在于什么时机移动哪个边界
///     * 一般右边界O(N)逐次移动, 即 一个 `for right in 0..lenggh`
///     * 左边界根据窗口的定义条件移动探索目标解
///     * 有时窗口大小是固定的
pub mod windows {

    /// [2024. 考试的最大困扰度](https://leetcode-cn.com/problems/maximize-the-confusion-of-an-exam/)
    /// 题目描述简化:
    /// 已知字符串中只有 `T` 和 `F`两种字符, 替换其中的k个字符, 使连续的 `T`或`F`最长
    ///
    /// 维持窗口内至多有k个非想要的字符;
    /// 如果不到k个, 就扩张右边界
    /// 如果多于k个, 就收缩左边界
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let bytes = answer_key.as_bytes();

        let max_replace = |ser: &[u8], target: u8| -> usize {
            let mut max_consecutive = 0;
            let mut other_cnt = 0;

            let mut left = 0;

            for right in 0..ser.len() {
                let r = ser.get(right).unwrap();
                if !target.eq(r) {
                    other_cnt += 1;
                }
                while other_cnt > k {
                    let l = ser.get(left).unwrap();
                    if !target.eq(l) {
                        other_cnt -= 1;
                    }
                    left += 1;
                }
                max_consecutive = max_consecutive.max(right - left + 1);
            }
            max_consecutive
        };

        max_replace(bytes, b'T').max(max_replace(bytes, b'F')) as i32
    }

    /// [1004. 最大连续1的个数 III](https://leetcode-cn.com/problems/max-consecutive-ones-iii/)
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let longest = |ser: &[i32], target: i32| -> usize {
            let mut max_consecutive = 0;
            let mut other_cnt = 0;

            let mut left = 0;
            for right in 0..ser.len() {
                let i = ser.get(right).unwrap();
                if !target.eq(i) {
                    other_cnt += 1;
                }
                while other_cnt > k {
                    let l = ser.get(left).unwrap();
                    if !target.eq(l) {
                        other_cnt -= 1;
                    }
                    left += 1;
                }
                max_consecutive = max_consecutive.max(right - left + 1); // 同为索引, 因此需要+1
            }
            max_consecutive
        };

        longest(&nums, 1) as i32
    }

    /// [209. 长度最小的子数组](https://leetcode-cn.com/problems/minimum-size-subarray-sum/)
    ///
    /// 如果窗口内和不够, 就扩展右边界
    /// 如果大于等于, 就尝试收缩左边界, 以便求出最小长度
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len: Option<usize> = None;

        let mut left = 0;
        let mut curr_sum = 0;
        for right in 0..nums.len() {
            let r = nums.get(right).unwrap();
            curr_sum += *r;
            while curr_sum >= target {
                if min_len.is_none() {
                    min_len.replace(right - left + 1);
                }
                min_len = min_len.min(Some(right - left + 1));

                let l = nums.get(left).unwrap();
                curr_sum -= *l;
                left += 1;
            }
        }
        min_len.unwrap_or(0) as i32
    }

    /// [76. 最小覆盖子串](https://leetcode-cn.com/problems/minimum-window-substring/)
    ///
    /// 如果窗口内的覆盖不了, 就扩展右边界
    /// 如果窗口内的能覆盖, 就尝试收缩左边界, 求最小长度
    ///
    /// ```ignore
    /// pub fn min_window(s: String, t: String) -> String {
    ///     use std::cmp::Ordering;
    ///     use std::collections::HashMap;
    ///
    ///     let can_cover = |target: &HashMap<u8, i32>, range: &HashMap<u8, i32>| -> bool{
    ///         for (k, v) in target.iter(){
    ///             let x = range.get(k);
    ///             if x.is_none(){
    ///                 return false;
    ///             }
    ///             match v.cmp(x.unwrap()){
    ///                 Ordering::Greater => return false,
    ///                 _ => {}
    ///             }
    ///         }
    ///         return true;
    ///     };
    ///
    ///     let t_cnt = {
    ///         let mut tmp = HashMap::with_capacity(52);
    ///         t.bytes().for_each(|c|{
    ///             *tmp.entry(c).or_insert(0) += 1;
    ///         });
    ///         tmp
    ///     };
    ///
    ///     let s_cnt = {
    ///         let mut tmp = HashMap::with_capacity(52);
    ///         s.bytes().for_each(|c|{
    ///             *tmp.entry(c).or_insert(0) += 1;
    ///         });
    ///         tmp
    ///     };
    ///
    ///     if !can_cover(&t_cnt, &s_cnt){
    ///         return "".to_string();
    ///     }
    ///
    ///     let bytes = s.as_bytes();
    ///     
    ///     let mut left= 0;
    ///     let (mut start, mut length) = (0, usize::MAX);
    ///
    ///     let mut counter = HashMap::with_capacity(52);
    ///
    ///     for right in 0..bytes.len(){
    ///         let c = bytes.get(right).unwrap();
    ///         *counter.entry(*c).or_insert(0) += 1;
    ///
    ///         while can_cover(&t_cnt, &counter){
    ///             if right - left + 1 < length{
    ///                 start = left;
    ///                 length = right - left + 1;
    ///             }
    ///             let l = bytes.get(left).unwrap();
    ///             *counter.entry(*l).or_default() -= 1;
    ///             left += 1;
    ///         }
    ///     }
    ///
    ///     String::from_utf8(bytes[start..start+length].to_vec()).unwrap()
    /// }
    /// ```
    ///
    /// * 由于字母计数不会突变, 因此在 判定是否覆盖时, 可以不用全量扫描
    /// * 由于全英文字母, 因此可以用 26 * 2的数组表示替换hash表
    ///     * 不过大小写范围不同, 还得处理索引, 不如hash方便
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;

        let need = {
            let mut tmp = HashMap::with_capacity(52);
            t.bytes().for_each(|c| {
                *tmp.entry(c).or_insert(0) += 1;
            });
            tmp
        };

        let bytes = s.as_bytes();

        let (mut start, mut length): (usize, Option<usize>) = (0, None);

        let mut window_cnt = HashMap::with_capacity(52);
        let mut valid_char_cnt = 0;

        let mut left = 0;
        for right in 0..bytes.len() {
            let rc = bytes.get(right).unwrap();

            if let Some(cnt) = need.get(rc) {
                // 不需要的字符, 不统计
                let entry = window_cnt.entry(*rc).or_insert(0);
                *entry += 1;
                if entry == cnt {
                    // 又一个字符达到要求
                    valid_char_cnt += 1;
                }
            }

            while valid_char_cnt == need.len() {
                let ll = length.unwrap_or(usize::MAX);
                if right - left + 1 < ll {
                    start = left;
                    length.replace(right - left + 1);
                }

                let lc = bytes.get(left).unwrap();
                left += 1;

                if let Some(cnt) = need.get(lc) {
                    // 不需要的字符, 不统计
                    let entry = window_cnt.entry(*lc).or_insert(0);
                    if entry == cnt {
                        // 一个达标字符被删掉一个
                        valid_char_cnt -= 1;
                    }
                    *entry -= 1;
                }
            }
        }
        if length.is_none() {
            return "".to_string();
        }

        String::from_utf8(bytes[start..start + length.unwrap()].to_vec()).unwrap()
    }

    /// [567. 字符串的排列](https://leetcode-cn.com/problems/permutation-in-string/)
    ///
    /// 由于是子串, 长度必然要相等, 因此固定窗口大小为 s1 的长度.
    ///
    /// 与 76 基本相似, 只是窗口大小固定
    /// 因此调整窗口边界增加一个大小限制
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        use std::collections::HashMap;
        let need: HashMap<u8, i32> = {
            let mut tmp = HashMap::with_capacity(26);
            s1.bytes().for_each(|b| {
                *tmp.entry(b).or_default() += 1;
            });
            tmp
        };

        let bytes = s2.as_bytes();
        let length = s1.as_bytes().len();

        let mut valid = 0;
        let mut window_cnt: HashMap<u8, i32> = HashMap::with_capacity(26);

        let mut left = 0;
        for right in 0..bytes.len() {
            let rc = bytes.get(right).unwrap();
            if let Some(nc) = need.get(rc) {
                let entry = window_cnt.entry(*rc).or_default();
                *entry += 1;
                if entry == nc {
                    valid += 1;
                }
            }

            while right - left + 1 >= length {
                if valid == need.len() {
                    return true;
                }
                let lc = bytes.get(left).unwrap();
                left += 1;

                if let Some(nc) = need.get(lc) {
                    let entry = window_cnt.entry(*lc).or_default();
                    if entry == nc {
                        valid -= 1;
                    }
                    *entry -= 1;
                }
            }
        }

        false
    }

    /// [438. 找到字符串中所有字母异位词](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)
    ///
    /// 与 567 基本相似, 只是需要记录起点位置
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        use std::collections::HashMap;
        let need = {
            let mut tmp = HashMap::with_capacity(26);
            p.bytes().for_each(|c| {
                *tmp.entry(c).or_insert(0) += 1;
            });
            tmp
        };

        let bytes = s.as_bytes();
        let window_size = p.as_bytes().len();

        let mut valid = 0;
        let mut window_cnt = HashMap::with_capacity(0);

        let mut ret = vec![];

        let mut left = 0;
        for right in 0..bytes.len() {
            let rc = bytes.get(right).unwrap();
            if let Some(nc) = need.get(rc) {
                let entry = window_cnt.entry(*rc).or_insert(0);
                *entry += 1;
                if entry == nc {
                    valid += 1;
                }
            }

            while right - left + 1 >= window_size {
                if valid == need.len() {
                    ret.push(left as i32);
                }
                let lc = bytes.get(left).unwrap();
                left += 1;
                if let Some(nc) = need.get(lc) {
                    let entry = window_cnt.entry(*lc).or_insert(0);
                    if entry == nc {
                        valid -= 1;
                    }
                    *entry -= 1;
                }
            }
        }

        ret
    }

    /// [3. 无重复字符的最长子串](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/)
    ///
    /// 窗口内没有重复字符时, 就扩张右边界, 并记录结果
    /// 窗口内有重复字符时, 就收缩左边界
    ///
    /// ~~由于窗口内字母常态时仅一个, 可以不用保存计数,~~
    /// * 可以保存谁破坏了规则, 即谁重复了
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let bytes = s.as_bytes();

        let mut max_length = 0;

        let mut who: Option<u8> = None;
        let mut cnt = HashMap::with_capacity(26);

        let mut left = 0;
        for right in 0..bytes.len() {
            let rc = bytes.get(right).unwrap();

            let entry = cnt.entry(*rc).or_insert(0);
            *entry += 1;

            if *entry >= 2 {
                who.replace(*rc);
            } else {
                max_length = max_length.max(right - left + 1);
            }

            // 没有像其他语言一样直接 cnt[rc] > 1
            // 也没有用上面的entry, 语言限制 ??
            while who.is_some() {
                let lc = bytes.get(left).unwrap();
                left += 1;

                let entry = cnt.entry(*lc).or_insert(0);
                *entry -= 1;

                if who.unwrap() == *lc {
                    who.take();
                }
            }
        }

        max_length as i32
    }

    /// [904. 水果成篮](https://leetcode-cn.com/problems/fruit-into-baskets/)
    ///
    /// * 保持窗口内 只有 f1, f2 两种水果
    /// * 如果 符合, 继续扩张右边界, 并计算将结果
    /// * 如果出现不符合, 找到窗口内 离f3最远那个的最后出现位置, 将left重置到 其最后出现位置 + 1
    ///     * 因此需要保存每个水果种类最后出现的问题
    ///
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut f1, mut f2): (Option<(i32, usize)>, Option<(i32, usize)>) = (None, None);

        let mut max_length = 0;

        let mut left = 0;
        for right in 0..fruits.len() {
            let f3_calss = fruits.get(right).unwrap();

            if f1.is_none() {
                f1.replace((*f3_calss, right));
            } else if *f3_calss == f1.unwrap().0 {
                f1.replace((*f3_calss, right));
            } else if f2.is_none() {
                f2.replace((*f3_calss, right));
            } else if *f3_calss == f2.unwrap().0 {
                f2.replace((*f3_calss, right));
            } else {
                let prev_class = fruits.get(right - 1).unwrap();
                if *prev_class == f1.unwrap().0 {
                    left = f2.unwrap().1 + 1;
                    f2.replace((*f3_calss, right));
                } else {
                    left = f1.unwrap().1 + 1;
                    f1.replace((*f3_calss, right));
                }
            }

            max_length = max_length.max(right - left + 1);
        }

        max_length as i32
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_total_fruit() {
            struct TestCase {
                name: &'static str,
                fruits: &'static [i32],
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    fruits: &[1, 2, 1],
                    expect: 3,
                },
                TestCase {
                    name: "basic 2",
                    fruits: &[0, 1, 2, 2],
                    expect: 3,
                },
                TestCase {
                    name: "basic 3",
                    fruits: &[1, 2, 3, 2, 2],
                    expect: 4,
                },
                TestCase {
                    name: "basic 4",
                    fruits: &[3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4],
                    expect: 5,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = total_fruit(testcase.fruits.to_vec());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_length_of_longest_substring() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "abcabcbb",
                    expect: 3,
                },
                TestCase {
                    name: "basic 2",
                    s: "bbbbb",
                    expect: 1,
                },
                TestCase {
                    name: "basic 3",
                    s: "pwwkew",
                    expect: 3,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = length_of_longest_substring(testcase.s.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_find_anagrams() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                p: &'static str,
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "cbaebabacd",
                    p: "abc",
                    expect: &[0, 6],
                },
                TestCase {
                    name: "basic 2",
                    s: "abab",
                    p: "ab",
                    expect: &[0, 1, 2],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = find_anagrams(testcase.s.to_string(), testcase.p.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_check_inclusion() {
            struct TestCase {
                name: &'static str,
                s1: &'static str,
                s2: &'static str,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    s1: "ab",
                    s2: "eidbaooo",
                    expect: true,
                },
                TestCase {
                    name: "basic 2",
                    s1: "ab",
                    s2: "eidboaoo",
                    expect: false,
                },
                TestCase {
                    name: "fix 1",
                    s1: "hello",
                    s2: "ooolleoooleh",
                    expect: false,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = check_inclusion(testcase.s1.to_string(), testcase.s2.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_min_window() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                t: &'static str,
                expect: &'static str,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "ADOBECODEBANC",
                    t: "ABC",
                    expect: "BANC",
                },
                TestCase {
                    name: "basic 2",
                    s: "a",
                    t: "a",
                    expect: "a",
                },
                TestCase {
                    name: "basic 3",
                    s: "a",
                    t: "aa",
                    expect: "",
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = min_window(testcase.s.to_string(), testcase.t.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_min_sub_array_len() {
            struct TestCase {
                name: &'static str,
                target: i32,
                nums: &'static [i32],
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    target: 7,
                    nums: &[2, 3, 1, 2, 4, 3],
                    expect: 2,
                },
                TestCase {
                    name: "basic 2",
                    target: 4,
                    nums: &[1, 4, 4],
                    expect: 1,
                },
                TestCase {
                    name: "basic 3",
                    target: 11,
                    nums: &[1, 1, 1, 1, 1, 1, 1, 1],
                    expect: 0,
                },
                TestCase {
                    name: "fix 1",
                    target: 11,
                    nums: &[1, 2, 3, 4, 5],
                    expect: 3,
                },
                TestCase {
                    name: "fix 2",
                    target: 15,
                    nums: &[1, 2, 3, 4, 5],
                    expect: 5,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let nums = testcase.nums.to_vec();
                let actual = min_sub_array_len(testcase.target, nums);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_longest_ones() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0],
                    k: 2,
                    expect: 6,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                    k: 3,
                    expect: 10,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let nums = testcase.nums.to_vec();
                let actual = longest_ones(nums, testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name)
            });
        }

        #[test]
        fn test_max_consecutive_answers() {
            struct TestCase {
                name: &'static str,
                answer: &'static str,
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    answer: "TTFF",
                    k: 2,
                    expect: 4,
                },
                TestCase {
                    name: "basic 2",
                    answer: "TFFT",
                    k: 1,
                    expect: 3,
                },
                TestCase {
                    name: "basic 3",
                    answer: "TTFTTFTT",
                    k: 1,
                    expect: 5,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let answer = testcase.answer.to_string();
                let actual = max_consecutive_answers(answer, testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }
    }
}
