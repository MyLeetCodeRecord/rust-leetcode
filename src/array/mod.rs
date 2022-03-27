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
/// * 中等:
///     * [167. 两数之和 II - 输入有序数组](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/)
///     * [5. 最长回文子串](https://leetcode-cn.com/problems/longest-palindromic-substring/)
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
        let(mut left, mut right) = (1, nums.len());
        while left <= right {
            let l = nums.get(left-1).copied().unwrap();
            let r = nums.get(right-1).copied().unwrap();

            if l.abs() > r.abs(){
                ret.push(l*l);
                left += 1;
            } else {
                ret.push(r*r);
                right -= 1;
            }
        }
        ret.reverse();
        ret
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
        fn test_sorted_squares(){
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase{
                    name: "basic",
                    nums: &[-4,-1,0,3,10],
                    expect: &[0,1,9,16,100]
                },
                TestCase{
                    name: "basic 2",
                    nums: &[-7,-3,2,3,11],
                    expect: &[4,9,9,49,121]
                },
                TestCase{
                    name: "fix 1",
                    nums: &[1],
                    expect: &[1]
                }
            ].iter().for_each(|testcase| {
                let actual = sorted_squares(testcase.nums.to_vec());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }
    }
}
