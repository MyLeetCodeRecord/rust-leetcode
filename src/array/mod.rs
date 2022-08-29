//! 虽然mod名为`array`, 但并不只是数组相关题目
//! 线性表相关的都会涉及
//! 比如数组(连续存储), 简单链表, 字符串(连续存储)

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
///     * [704. 二分查找](search)
///     * [35. 搜索插入位置](search_insert)
/// * 中等:
///     * [34. 在排序数组中查找元素的第一个和最后一个位置](search_range)
///     * [875. 爱吃香蕉的珂珂](min_eating_speed)
///     * [1011. 在 D 天内送达包裹的能力](ship_within_days)
///     * [2226. 每个小孩最多能分到多少糖果](maximum_candies)
///     * [436. 寻找右区间](find_right_interval)
///     * [33. 搜索旋转排序数组](search_2)
/// * 困难
///     * [668. 乘法表中第k小的数](find_kth_number)
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

    /// [2226. 每个小孩最多能分到多少糖果](https://leetcode-cn.com/problems/maximum-candies-allocated-to-k-children/)
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        const MAX_CANDY: i64 = 10000000;
        if candies.iter().fold(0i64, |acc, x| acc + *x as i64) < k {
            return 0;
        }
        let (mut left, mut right) = (1i64, MAX_CANDY);

        while left <= right {
            let mid = (left + right) / 2;
            let _k = candies.iter().map(|x| *x as i64 / mid).sum::<i64>();
            if _k >= k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left as i32 - 1
    }

    /// [668. 乘法表中第k小的数](https://leetcode.cn/problems/kth-smallest-number-in-multiplication-table/)
    ///
    /// tips:
    ///     1. (1<<32) - 1  > 30000 * 30000, 因此可以不用i64
    ///
    /// 解释:
    /// 假设乘法表中数x, 求x在什么位置(第几个).
    ///
    /// 记m行, n列.
    ///
    /// x/n 就表示在x之前有多少个完整的行. 因此起点计数即为  x/n * n
    /// 再补齐不满一行的.
    /// 还是前面的式子, 可知前面 x/n 行是满的, 从行号 x/n + 1 开始有不满一行的, 直到m
    /// 每一行的数量为 x/行号
    #[rustfmt::skip]
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut l, mut r) = (1, m * n);

        while l <= r {
            let mid = (l + r) / 2; // mid不一定在乘法表中

            let mut cnt = mid / n * n; // 整行的
            for i in (mid / n + 1)..=m {
                cnt += mid / i; // 不满整行的
            }

            // 由于mid不一定在乘法表中, 因此cnt有重复的可能, 因此属于有重复数组中找边界.
            match cnt.cmp(&k) {
                std::cmp::Ordering::Equal   => { r = mid - 1; },
                std::cmp::Ordering::Less    => { l = mid + 1; },
                std::cmp::Ordering::Greater => { r = mid - 1; },
            }
        }

        l
    }

    /// [436. 寻找右区间](https://leetcode.cn/problems/find-right-interval/)
    ///
    /// 最小起始位置
    ///
    /// 也就是找边界, 不是单独位置
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        // if intervals.len() <= 1 {
        //     return vec![-1];
        // }

        fn find_pos(rng: &[(i32, usize)], target: i32) -> Option<usize> {
            let (mut l, mut r) = (1, rng.len());
            while l <= r {
                let mid = (l + r) / 2;
                let p = rng.get(mid - 1).unwrap();
                match target.cmp(&p.0) {
                    std::cmp::Ordering::Less => {
                        r = mid - 1;
                    }
                    std::cmp::Ordering::Equal => {
                        r = mid - 1; // 发现第一个大于等于的, 因此把r向前提
                    }
                    std::cmp::Ordering::Greater => {
                        l = mid + 1;
                    }
                }
            }
            if l > rng.len() {
                None
            } else {
                Some(rng.get(l - 1).unwrap().1)
            }
        }

        let mut start = intervals
            .iter()
            .enumerate()
            .map(|(idx, rng)| (rng.first().copied().unwrap(), idx))
            .collect::<Vec<(i32, usize)>>();
        start.sort_by(|a, b| a.0.cmp(&b.0));

        let mut ret = vec![];
        for inter in intervals.iter() {
            let end = inter.last().copied().unwrap();
            if let Some(pos) = find_pos(&start, end) {
                ret.push(pos as i32);
            } else {
                ret.push(-1);
            }
        }
        ret
    }

    /// [33. 搜索旋转排序数组](https://leetcode.cn/problems/search-in-rotated-sorted-array/)
    /// 分情况讨论
    pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (1, nums.len());
        while left <= right {
            let mid = (left + right) / 2;
            let (l, m, r) = (
                nums.get(left - 1).copied().unwrap(),
                nums.get(mid - 1).copied().unwrap(),
                nums.get(right - 1).copied().unwrap(),
            );
            match target.cmp(&m) {
                std::cmp::Ordering::Equal => {
                    return mid as i32 - 1;
                }
                std::cmp::Ordering::Greater => {
                    if l > r {
                        if m > l {
                            left = mid + 1;
                        } else if target > r {
                            right = mid - 1;
                        } else {
                            left = mid + 1;
                        }
                    } else {
                        left = mid + 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    if l > r {
                        if m > l {
                            if target < l {
                                left = mid + 1;
                            } else {
                                right = mid - 1;
                            }
                        } else {
                            right = mid - 1;
                        }
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }
        -1
    }

    /// [172. 阶乘后的零](https://leetcode.cn/problems/factorial-trailing-zeroes/)
    ///
    /// 0 只会由 2 * 5 产生, 数量为 min{count(2), count(5)}
    ///
    /// 由于 `n/5 < n/2`, 因此 min{count(2), count(5)} ==> count(5)
    ///
    /// 1..15 其实有 3 个可以做因子的5, 一个来自5, 一个来自 10, 一个来自15
    /// 1..25 其实有 5+1个可以做因子的5, 除了1..15的三个, 还有一个来自20 , 两个来自25
    ///
    /// 同样, 125可以贡献三个, 525可以贡献四个.
    ///
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut cnt = 0;
        let mut n = n;

        loop {
            if n == 0 {
                break;
            }
            n = n / 5;
            cnt += n;
        }
        cnt
    }

    /// [793. 阶乘函数后 K 个零](https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function/)
    ///
    /// 相对与[172. 阶乘后的零](trailing_zeroes), *172*是给定阶乘求结尾有几个0, *793*是给定数量, 求有多少个阶乘
    /// 
    /// 由 *172*可知, 一段范围内的阶乘, 0的数量不变, 即存在相等, 求边界
    ///
    /// 题目 k 的范围是`[0, 10**9]`, 对应的阶乘范围很大, 可以检测 `i32::MAX` 的0的数量大于 10**9, 因此可以作为右边界
    /// 但是实测, 用 `i32::MAX` 会超时. 
    /// 
    /// 通过 *172* 可以得到, 右边界可以用 5*k 
    /// 
    pub fn preimage_size_fzf(k: i32) -> i32 {

        fn left_bound(k: i32) -> i32{
            let (mut left, mut right) = (0, 5*k);
            while left <= right{
                let mid = left + (right-left)/2;
                let count = trailing_zeroes(mid);
                if count >= k{
                    right = mid-1;
                } else {
                    left = mid+1;
                }
            }
            return right+1;
        }

        left_bound(k+1) - left_bound(k)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_preimage_size_fzf() {
            struct TestCase {
                name: &'static str,
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    k: 0,
                    expect: 5,
                },
                TestCase {
                    name: "basic 2",
                    k: 5,
                    expect: 0,
                },
                TestCase {
                    name: "basic 3",
                    k: 3,
                    expect: 5,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = preimage_size_fzf(testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_trailing_zeroes() {
            struct TestCase {
                name: &'static str,
                n: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic 1",
                    n: 3,
                    expect: 0,
                },
                TestCase {
                    name: "basic 2",
                    n: 5,
                    expect: 1,
                },
                TestCase {
                    name: "basic 3",
                    n: 0,
                    expect: 0,
                },
                TestCase {
                    name: "fix 1",
                    n: 10,
                    expect: 2,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = trailing_zeroes(testcase.n);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_search_2() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                target: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[4, 5, 6, 7, 0, 1, 2],
                    target: 0,
                    expect: 4,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[4, 5, 6, 7, 0, 1, 2],
                    target: 3,
                    expect: -1,
                },
                TestCase {
                    name: "basic 3",
                    nums: &[1],
                    target: 0,
                    expect: -1,
                },
                TestCase {
                    name: "fix 1",
                    nums: &[3, 5, 1],
                    target: 3,
                    expect: 0,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = search_2(testcase.nums.to_vec(), testcase.target);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_find_right_interval() {
            struct TestCase {
                name: &'static str,
                intervals: &'static [&'static [i32]],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    intervals: &[&[1, 2]],
                    expect: &[-1],
                },
                TestCase {
                    name: "basic 2",
                    intervals: &[&[3, 4], &[2, 3], &[1, 2]],
                    expect: &[-1, 0, 1],
                },
                TestCase {
                    name: "basic 3",
                    intervals: &[&[1, 4], &[2, 3], &[3, 4]],
                    expect: &[-1, 2, -1],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let intervals = testcase.intervals.iter().map(|x| x.to_vec()).collect();
                let actual = find_right_interval(intervals);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_find_kth_number() {
            struct TestCase {
                name: &'static str,
                m: i32,
                n: i32,
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    m: 3,
                    n: 3,
                    k: 5,
                    expect: 3,
                },
                TestCase {
                    name: "basic 2",
                    m: 2,
                    n: 3,
                    k: 6,
                    expect: 6,
                },
                TestCase {
                    name: "fix 1",
                    m: 1,
                    n: 3,
                    k: 2,
                    expect: 2,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = find_kth_number(testcase.m, testcase.n, testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_maximum_candies() {
            struct TestCase {
                name: &'static str,
                candies: &'static [i32],
                k: i64,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    candies: &[5, 8, 6],
                    k: 3,
                    expect: 5,
                },
                TestCase {
                    name: "basic 2",
                    candies: &[2, 5],
                    k: 11,
                    expect: 0,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = maximum_candies(testcase.candies.to_vec(), testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

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
///     * [27. 移除元素](remove_element)
///     * [26. 删除有序数组中的重复项](remove_duplicates)
///     * [83. 删除排序链表中的重复元素](delete_duplicates)
///     * [283. 移动零](move_zeroes)
///     * [344. 反转字符串](reverse_string)
///     * [977. 有序数组的平方](sorted_squares)
///     * [21. 合并两个有序链表](merge_k_lists)
///     * [876. 链表的中间结点](middle_node)
///     * [541. 反转字符串 II](reverse_str)
///     * [剑指 Offer 05. 替换空格](replace_space)
///     * [88. 合并两个有序数组](merge)
/// * 中等:
///     * [167. 两数之和 II - 输入有序数组](two_sum)
///     * [5. 最长回文子串](longest_palindrome)
///     * [19. 删除链表的倒数第 N 个结点](remove_nth_from_end)
///     * [870. 优势洗牌](advantage_count)
///     * [186. 翻转字符串里的单词 II](reverse_words)
///     * [151. 颠倒字符串中的单词](reverse_words_1)
///     * [面试题 17.11. 单词距离](find_closest)
///     * [1089. 复写零](duplicate_zeros)
/// * 困难:
///     * [23. 合并K个升序链表](merge_k_lists)
/// * 没有rust模版的题:
///     * [141. 环形链表](https://leetcode-cn.com/problems/linked-list-cycle/)
///     * [160. 相交链表](https://leetcode-cn.com/problems/intersection-of-two-linked-lists/)
///
pub mod two_pointers {

    /// [27. 移除元素](https://leetcode-cn.com/problems/remove-element/)
    /// 索引 usize 可能溢出
    #[allow(clippy::ptr_arg)]
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
    #[allow(clippy::ptr_arg)]
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
    #[allow(clippy::ptr_arg)]
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
        vec![left as i32, right as i32]
    }

    /// [344. 反转字符串](https://leetcode-cn.com/problems/reverse-string/)
    #[allow(clippy::ptr_arg)]
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

            s[(l + 1) as usize..r as usize].iter().collect()
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

    /// [870. 优势洗牌](https://leetcode-cn.com/problems/advantage-shuffle/)
    ///
    /// 田忌赛马
    ///
    /// 齐王的马顺序不能变, 因此需要记录位置之后再排序. 可以使用优先队列, 也可以直接排
    /// ```
    /// pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    ///     use std::cmp::Ordering;
    ///     use std::collections::BinaryHeap;
    ///
    ///     struct Element(usize, i32);
    ///     impl std::cmp::Ord for Element {
    ///         fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    ///             self.1.cmp(&other.1)
    ///         }
    ///     }
    ///     impl std::cmp::PartialOrd for Element {
    ///         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    ///             Some(self.cmp(other))
    ///         }
    ///     }
    ///     impl std::cmp::PartialEq for Element {
    ///         fn eq(&self, other: &Self) -> bool {
    ///             self.1.eq(&other.1)
    ///         }
    ///     }
    ///     impl std::cmp::Eq for Element {}
    ///
    ///     let mut nums2: BinaryHeap<Element> = nums2
    ///         .into_iter()
    ///         .enumerate()
    ///         .map(|(idx, num)| Element(idx, num))
    ///         .collect();
    ///
    ///     let mut nums1 = nums1;
    ///     nums1.sort();
    ///
    ///     let mut result = nums1.clone();
    ///
    ///     let (mut left, mut right) = (1, nums1.len());
    ///     while !nums2.is_empty() {
    ///         let Element(index, val) = nums2.pop().unwrap();
    ///         match val.cmp(nums1.get(right - 1).unwrap()) {
    ///             Ordering::Greater => {
    ///                 // 用次等马对它的好马
    ///                 *result.get_mut(index).unwrap() = nums1.get(left - 1).copied().unwrap();
    ///                 left += 1;
    ///             }
    ///             Ordering::Less => {
    ///                 // 能胜过就胜过
    ///                 *result.get_mut(index).unwrap() = nums1.get(right - 1).copied().unwrap();
    ///                 right -= 1;
    ///             }
    ///             Ordering::Equal => {
    ///                 // 同等战力, 用次的替换, 保存实力
    ///                 *result.get_mut(index).unwrap() = nums1.get(left - 1).copied().unwrap();
    ///                 left += 1;
    ///             }
    ///         }
    ///     }
    ///
    ///     result
    /// }
    /// ```
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::cmp::Ordering;

        let mut nums1 = nums1;
        nums1.sort();
        let mut nums2 = nums2.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
        nums2.sort_by(|a, b| a.1.cmp(&b.1));

        let mut result = nums1.clone();

        let (mut left, mut right) = (1, nums1.len());
        while !nums2.is_empty() {
            let (index, val) = nums2.pop().unwrap();
            match val.cmp(nums1.get(right - 1).unwrap()) {
                Ordering::Greater => {
                    // 用次等马对它的好马
                    *result.get_mut(index).unwrap() = nums1.get(left - 1).copied().unwrap();
                    left += 1;
                }
                Ordering::Less => {
                    // 能胜过就胜过
                    *result.get_mut(index).unwrap() = nums1.get(right - 1).copied().unwrap();
                    right -= 1;
                }
                Ordering::Equal => {
                    // 同等战力, 用次的替换, 保存实力
                    *result.get_mut(index).unwrap() = nums1.get(left - 1).copied().unwrap();
                    left += 1;
                }
            }
        }

        result
    }

    /// [541. 反转字符串 II](https://leetcode-cn.com/problems/reverse-string-ii/)
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s;
        let k = k as usize;
        let bytes = unsafe { s.as_mut_vec() };

        let mut start = 0;
        while start < bytes.len() {
            let mut x = start;
            let mut y = start + k - 1;

            if start + k >= bytes.len() {
                y = bytes.len() - 1;
            }
            while x < y {
                bytes.swap(x, y);
                x += 1;
                y -= 1;
            }
            start += 2 * k;
        }
        s
    }

    /// [剑指 Offer 05. 替换空格](https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/)
    pub fn replace_space(s: String) -> String {
        let mut s = s;
        let bytes = unsafe { s.as_mut_vec() };
        let mut old = bytes.len();

        let space_cnt = bytes.iter().filter(|b| b' '.eq(b)).count();

        bytes.extend(vec![0; space_cnt * 2]);
        let mut new = bytes.len();

        while old > 0 {
            let b = bytes.get(old - 1).copied().unwrap();
            if b != b' ' {
                *bytes.get_mut(new - 1).unwrap() = b;
                new -= 1;
            } else {
                *bytes.get_mut(new - 1).unwrap() = b'0';
                new -= 1;
                *bytes.get_mut(new - 1).unwrap() = b'2';
                new -= 1;
                *bytes.get_mut(new - 1).unwrap() = b'%';
                new -= 1;
            }
            old -= 1;
        }
        s
    }

    /// [186. 翻转字符串里的单词 II](https://leetcode-cn.com/problems/reverse-words-in-a-string-ii/)
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();

        let mut start = 0;
        while start < s.len() {
            let mut x = start;
            let mut y = start;
            while let Some(c) = s.get(y) {
                if *c == ' ' {
                    break;
                }
                y += 1;
            }

            start = y + 1;

            y -= 1;
            while x < y {
                s.swap(x, y);
                x += 1;
                y -= 1;
            }
        }
    }

    /// [151. 颠倒字符串中的单词](https://leetcode-cn.com/problems/reverse-words-in-a-string/)
    ///
    /// 思路1: 字符串操作, 分割拼接
    /// ```
    /// pub fn reverse_words(s: String) -> String {
    ///     s.split_ascii_whitespace()
    ///         .filter(|ss| !ss.eq(&" "))
    ///         .rev()
    ///         .map(|ss| ss.to_string())
    ///         .collect::<Vec<String>>()
    ///         .join(" ")
    /// }
    /// ```
    /// 但是这样做, 需要O(N)的空间
    /// 思路2: 双指针, 原地操作.
    /// 只是在T186的基础上, 加了删除空格
    pub fn reverse_words_1(s: String) -> String {
        let mut s = s;
        let bytes = unsafe { s.as_mut_vec() };
        bytes.reverse();

        // clean space
        let mut insert_pos = 0;
        let mut need_space = false;
        for i in 0..bytes.len() {
            let curr = bytes.get(i).copied().unwrap();
            if curr != b' ' {
                *bytes.get_mut(insert_pos).unwrap() = curr;
                need_space = true;
                insert_pos += 1;
            } else if need_space {
                *bytes.get_mut(insert_pos).unwrap() = curr;
                insert_pos += 1;
                need_space = false;
            }
        }
        if let Some(c) = bytes.get(insert_pos - 1) {
            if *c == b' ' {
                insert_pos -= 1;
            }
        }
        bytes.truncate(insert_pos);

        let mut start = 0;
        while start < bytes.len() {
            let mut x = start;
            let mut y = start;
            while let Some(c) = bytes.get(y) {
                if *c == b' ' {
                    break;
                }
                y += 1;
            }

            start = y + 1;

            y -= 1;
            while x < y {
                bytes.swap(x, y);
                x += 1;
                y -= 1;
            }
        }
        s
    }

    /// [88. 合并两个有序数组](https://leetcode.cn/problems/merge-sorted-array/)
    ///
    /// nums1 有后缀填充的0
    /// 如果正向(从小到大)的合并, 由于插入, 需要有shift操作.
    /// 因此需要从大到小的merge
    #[rustfmt::skip]
    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut insert_position = (nums1.len() - 1) as isize;
        let (mut p0, mut p1) = (m as usize, n as usize);
        while insert_position >= 0 {
            let x = { if p0 == 0 { i32::MIN } else { nums1.get(p0 - 1).copied().unwrap() } };
            let y = { if p1 == 0 { i32::MIN } else { nums2.get(p1 - 1).copied().unwrap() } };

            nums1[insert_position as usize] = std::cmp::max(x, y);
            insert_position -= 1;
            match x.cmp(&y) {
                std::cmp::Ordering::Greater => { p0 -= 1; }
                _ => { p1 -= 1; }
            }
        }
    }

    /// [面试题 17.11. 单词距离](https://leetcode.cn/problems/find-closest-lcci/)
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        enum WhichWord {
            X,
            Y,
        }
        // 优化点: 可以不用全量记录位置, 直接双指针
        let mark = words
            .iter()
            .enumerate()
            .filter_map(|(idx, s)| {
                if word1.eq(s) {
                    Some((WhichWord::X, idx))
                } else if word2.eq(s) {
                    Some((WhichWord::Y, idx))
                } else {
                    None
                }
            })
            .collect::<Vec<(WhichWord, usize)>>();

        let (mut last_x, mut last_y): (Option<i32>, Option<i32>) = (None, None);
        let mut ret = i32::MAX;
        for (class, pos) in mark.iter() {
            let pos = *pos as i32;
            match class {
                WhichWord::X => {
                    if let Some(y) = last_y {
                        ret = std::cmp::min(ret, (pos - y).abs());
                    }
                    last_x.replace(pos);
                }
                WhichWord::Y => {
                    if let Some(x) = last_x {
                        ret = std::cmp::min(ret, (pos - x).abs());
                    }
                    last_y.replace(pos);
                }
            }
        }
        ret
    }

    /// [1089. 复写零](https://leetcode.cn/problems/duplicate-zeros/)
    ///
    /// 先确认原数组中哪个位置变成了新数组的结尾
    /// 有三种情况
    /// 1. 以一个非0数据结尾, 也就是加上这一个, 长度达到
    /// 2. 以一个原始0结尾, 也就是加上一个0, 长度达到,
    /// 3. 以一个补充0结尾, 也就是加上两个0, 长度达到
    ///
    /// 对于情况2, 由于每次0补两个, 因此长度会超过一个
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut top = 0usize;
        let mut last = 0;
        for (idx, &num) in arr.iter().enumerate() {
            top += 1;
            if num == 0 {
                top += 1;
            }
            if top >= arr.len() {
                last = idx;
                break;
            }
        }

        let mut end = arr.len(); // 填充位置, 为防止溢出, 这里取 索引+1
        if top > arr.len() {
            // 结尾是0, 命中情况2
            *arr.last_mut().unwrap() = 0;
            end -= 1;
            last -= 1; // 消耗掉一个
        }

        for cur in (0..=last).rev() {
            let num = arr.get(cur).copied().unwrap();
            if num == 0 {
                *arr.get_mut(end - 1).unwrap() = 0;
                *arr.get_mut(end - 2).unwrap() = 0;
                end -= 2;
                //end = end.checked_sub(2).unwrap_or(0);
            } else {
                *arr.get_mut(end - 1).unwrap() = num;
                end -= 1;
                //end = end.checked_sub(1).unwrap_or(0);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_duplicate_zeros() {
            struct TestCase {
                name: &'static str,
                arr: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic 1",
                    arr: &[1, 0, 2, 3, 0, 4, 5, 0],
                    expect: &[1, 0, 0, 2, 3, 0, 0, 4],
                },
                TestCase {
                    name: "basic 2",
                    arr: &[1, 2, 3],
                    expect: &[1, 2, 3],
                },
                TestCase {
                    name: "fix 1",
                    arr: &[0, 0, 0, 0, 0, 0, 0],
                    expect: &[0, 0, 0, 0, 0, 0, 0],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut arr = testcase.arr.to_vec();
                duplicate_zeros(&mut arr);
                assert_eq!(testcase.expect, arr, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_find_closest() {
            struct TestCase {
                name: &'static str,
                words: &'static [&'static str],
                word1: &'static str,
                word2: &'static str,
                expect: i32,
            }

            vec![TestCase {
                name: "basic 1",
                words: &[
                    "I",
                    "am",
                    "a",
                    "student",
                    "from",
                    "a",
                    "university",
                    "in",
                    "a",
                    "city",
                ],
                word1: "a",
                word2: "student",
                expect: 1,
            }]
            .iter()
            .for_each(|testcase| {
                let words = testcase.words.iter().map(|s| s.to_string()).collect();
                let actual = find_closest(
                    words,
                    testcase.word1.to_string(),
                    testcase.word2.to_string(),
                );
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_merge() {
            struct TestCase {
                name: &'static str,
                nums1: &'static [i32],
                m: i32,
                nums2: &'static [i32],
                n: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums1: &[1, 2, 3, 0, 0, 0],
                    m: 3,
                    nums2: &[2, 5, 6],
                    n: 3,
                },
                TestCase {
                    name: "basic 2",
                    nums1: &[1],
                    m: 1,
                    nums2: &[],
                    n: 0,
                },
                TestCase {
                    name: "basic 3",
                    nums1: &[0],
                    m: 0,
                    nums2: &[1],
                    n: 1,
                },
                TestCase {
                    name: "fix 1",
                    nums1: &[4, 5, 6, 0, 0, 0],
                    m: 3,
                    nums2: &[1, 2, 3],
                    n: 3,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let mut nums1 = testcase.nums1.to_vec();
                let mut nums2 = testcase.nums2.to_vec();
                merge(&mut nums1, testcase.m, &mut nums2, testcase.n);

                let mut nums = testcase.nums1.to_vec();
                let m = testcase.m as usize;
                nums[m..].copy_from_slice(&testcase.nums2[..(testcase.nums1.len() - m)]);
                nums.sort();

                assert_eq!(nums, nums1, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_reverse_words_1() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                expect: &'static str,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "the sky is blue",
                    expect: "blue is sky the",
                },
                TestCase {
                    name: "basic 2",
                    s: "  hello world  ",
                    expect: "world hello",
                },
                TestCase {
                    name: "basic 3",
                    s: "a good   example",
                    expect: "example good a",
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = reverse_words_1(testcase.s.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_reverse_words() {
            struct TestCase {
                name: &'static str,
                s: &'static [char],
                expect: &'static [char],
            }

            vec![TestCase {
                name: "basic",
                s: &[
                    't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
                ],
                expect: &[
                    'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
                ],
            }]
            .iter()
            .for_each(|testcase| {
                let mut s = testcase.s.to_vec();
                reverse_words(&mut s);
                assert_eq!(testcase.expect, s, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_replace_space() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                expect: &'static str,
            }

            vec![TestCase {
                name: "basic",
                s: "We are happy.",
                expect: "We%20are%20happy.",
            }]
            .iter()
            .for_each(|testcase| {
                let actual = replace_space(testcase.s.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_reverse_str() {
            struct TestCase {
                name: &'static str,
                s: &'static str,
                k: i32,
                expect: &'static str,
            }

            vec![
                TestCase {
                    name: "basic",
                    s: "abcdefg",
                    k: 2,
                    expect: "bacdfeg",
                },
                TestCase {
                    name: "basic 2",
                    s: "abcd",
                    k: 2,
                    expect: "bacd",
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = reverse_str(testcase.s.to_string(), testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_advantage_count() {
            struct TestCase {
                name: &'static str,
                nums1: &'static [i32],
                nums2: &'static [i32],
                expect: &'static [i32],
            }

            vec![
                TestCase {
                    name: "basic",
                    nums1: &[2, 7, 11, 15],
                    nums2: &[1, 10, 4, 11],
                    expect: &[2, 11, 7, 15],
                },
                TestCase {
                    name: "basic",
                    nums1: &[12, 24, 8, 32],
                    nums2: &[13, 25, 32, 11],
                    expect: &[24, 32, 8, 12],
                },
            ]
            .iter()
            .for_each(|testcase| {
                let acutal = advantage_count(testcase.nums1.to_vec(), testcase.nums2.to_vec());
                assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
            });
        }

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
                    let mut tmp = testcase.nums.iter().copied().collect::<HashSet<i32>>();
                    tmp.remove(&testcase.val);
                    tmp
                };
                let expect_length = testcase.nums.iter().fold(0, |acc, &x| {
                    if x != testcase.val {
                        return acc + 1;
                    }
                    acc
                });

                let mut tmp = testcase.nums.to_vec();
                let length = remove_element(&mut tmp, testcase.val) as usize;

                assert_eq!(expect_length, length, "{} length not match", testcase.name);

                let actual = tmp[..length].iter().copied().collect::<HashSet<i32>>();
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
                    .map(|l| build_list_from_slice(l))
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
/// * 简单
///     * [944. 删列造序](min_deletion_size)
///     * [961. 在长度 2N 的数组中找出重复 N 次的元素](repeated_n_times)
///     * [1200. 最小绝对差](minimum_abs_difference)
/// * 中等
///     * [2024. 考试的最大困扰度](max_consecutive_answers)
///     * [1004. 最大连续1的个数 III](longest_ones)
///     * [209. 长度最小的子数组](min_sub_array_len)
///     * [567. 字符串的排列](check_inclusion)
///     * [3. 无重复字符的最长子串](length_of_longest_substring)
///     * [904. 水果成篮](total_fruit)
///     * [438. 找到字符串中所有字母异位词](find_anagrams)
///     * [713. 乘积小于 K 的子数组](num_subarray_product_less_than_k)
///     * [面试题 01.05. 一次编辑](one_edit_away)
/// * 困难
///     * [76. 最小覆盖子串](min_window)
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
        let mut f1: Option<(i32, usize)> = None;
        let mut f2: Option<(i32, usize)> = None;

        let mut max_length = 0;

        let mut left = 0;
        for right in 0..fruits.len() {
            let f3_calss = fruits.get(right).unwrap();

            if f1.is_none() || *f3_calss == f1.unwrap().0 {
                f1.replace((*f3_calss, right));
            } else if f2.is_none() || *f3_calss == f2.unwrap().0 {
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

    /// [713. 乘积小于 K 的子数组](https://leetcode.cn/problems/subarray-product-less-than-k/)
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut curr_mul = 1;

        let mut left = 0;
        for right in 0..nums.len() {
            curr_mul *= nums.get(right).copied().unwrap();
            while left <= right && curr_mul >= k {
                curr_mul /= nums.get(left).copied().unwrap();
                left += 1;
            }
            // count += right - left + 1 会导致溢出
            count += right + 1 - left
        }
        count as i32
    }

    /// [944. 删列造序](https://leetcode.cn/problems/delete-columns-to-make-sorted/)
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        if strs.len() <= 1 {
            return 0;
        }
        let n = strs.first().unwrap().len();
        let mut ans = 0;
        'NextPoi: for i in 0..n {
            for win in strs.windows(2) {
                let (a, b) = (win.get(0).unwrap(), win.get(1).unwrap());
                let (a_bytes, b_bytes) = (a.as_bytes(), b.as_bytes());
                if b_bytes[i] < a_bytes[i] {
                    ans += 1;
                    continue 'NextPoi;
                }
            }
        }
        ans
    }

    /// [面试题 01.05. 一次编辑](https://leetcode.cn/problems/one-away-lcci/)
    pub fn one_edit_away(first: String, second: String) -> bool {
        if second.len() > first.len() {
            return one_edit_away(second, first);
        }
        if first.len() - second.len() >= 2 {
            return false;
        }

        let (first_bytes, second_bytes) = (first.as_bytes(), second.as_bytes());
        let (mut a, mut b) = (0, 0);
        let mut cnt = 0;
        while a < first_bytes.len() && b < second_bytes.len() {
            if first_bytes[a] == second_bytes[b] {
                a += 1;
                b += 1;
                continue;
            }
            if first_bytes.len() == second_bytes.len() {
                a += 1;
                b += 1;
                cnt += 1;
                continue;
            }
            // 向后错位1个
            a += 1;
            cnt += 1;
        }
        cnt < 2
    }

    /// [961. 在长度 2N 的数组中找出重复 N 次的元素](https://leetcode.cn/problems/n-repeated-element-in-size-2n-array/)
    ///
    /// 假设相同元素的间隔至少为2, 则总长度为 n + 2(n-1) = 3n - 2
    /// 当 n > 2 时, 3n - 2 > 2n, 不满足题意, 也即 n > 2 时, 间隔只能为 0或1 
    ///
    /// * 间隔为0的, 可以用窗口为2的检查
    /// * 间隔为1的, 分为开头是重复元素, 结尾是重复元素两种
    ///     * 如果两两判定消除, 使用剩余那个, 那重复元素在开头时, 会有问题
    ///     * 可以将窗口变为3
    /// * 间隔为2的, 可以将窗口变为4
    /// 
    /// 因此直接窗口使用4, 遍历即可
    /// 
    /// 写法1:
    /// ```
    /// pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    ///     // 最小长度为4
    ///     if nums.len() == 4{
    ///         let (a, b, c, d) = (nums[0], nums[1], nums[2], nums[3]);
    ///         if a==b { return a; }
    ///         else if b==c { return b; }
    ///         else if c==d { return c; }
    ///         else if a==c { return a; }
    ///         else if a==d { return a; }
    ///         else if b==d { return b; }
    ///     }
    /// 
    ///     for win in nums.windows(3) {
    ///         let (a, b, c) = (win[0], win[1], win[2]);
    ///         if a == b {
    ///             return a;
    ///         } else if b == c {
    ///             return b;
    ///         } else if a == c {
    ///             return c;
    ///         }
    ///     }
    ///     unreachable!()
    /// }
    /// ```
    /// 也可以统一逻辑
    #[rustfmt::skip]
    #[allow(clippy::if_same_then_else)]
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for win in nums.windows(4){
            let (a, b, c, d) = (win[0], win[1], win[2], win[3]);
            if a==b { return a; }
            else if b==c { return b; }
            else if c==d { return c; }
            else if a==c { return a; }
            else if a==d { return a; }
            else if b==d { return b; }
        }
        unreachable!()
    }

    /// [30. 串联所有单词的子串](https://leetcode.cn/problems/substring-with-concatenation-of-all-words/)
    ///
    /// 题目要求中的关键:
    ///     1. 长度相同的单词 (窗口大小)
    ///     2. 中间不能有其他字符 + 不需要考虑串联顺序 (匹配数量)
    ///
    /// 因此按照单词大小分割原字符串, 并记录每个单词的出现位置.
    /// 窗口大小即为单词个数(如果是看长度, 就是所有单词拼起来的长度)
    pub fn find_substring(_s: String, _words: Vec<String>) -> Vec<i32> {
        unimplemented!()
    }

    /// [1200. 最小绝对差](https://leetcode.cn/problems/minimum-absolute-difference/)
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut arr = arr;
        arr.sort();
        let mut ret = vec![];
        let mut min_diff = i32::MAX;
        arr.windows(2).for_each(|win| {
            let [a, b] = <[i32; 2]>::try_from(win).ok().unwrap();
            let diff = (a - b).abs();
            match diff.cmp(&min_diff) {
                Ordering::Equal => ret.push(vec![a, b]),
                Ordering::Less => {
                    ret.clear();
                    min_diff = diff;
                    ret.push(vec![a, b]);
                }
                _ => {}
            }
        });

        ret
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_repeated_n_times() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[1, 2, 3, 3],
                    expect: 3,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[2, 1, 2, 5, 3, 2],
                    expect: 2,
                },
                TestCase {
                    name: "basic 3",
                    nums: &[5, 1, 5, 2, 5, 3, 5, 4],
                    expect: 5,
                },
                TestCase {
                    name: "fix 1",
                    nums: &[9, 5, 6, 9],
                    expect: 9,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = repeated_n_times(testcase.nums.to_vec());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_one_edit_away() {
            struct TestCase {
                name: &'static str,
                first: &'static str,
                second: &'static str,
                expect: bool,
            }

            vec![
                TestCase {
                    name: "basic",
                    first: "pale",
                    second: "ple",
                    expect: true,
                },
                TestCase {
                    name: "basic 2",
                    first: "pales",
                    second: "pal",
                    expect: false,
                },
                TestCase {
                    name: "fix 1",
                    first: "intention",
                    second: "execution",
                    expect: false,
                },
                TestCase {
                    name: "fix 2",
                    first: "teacher",
                    second: "taches",
                    expect: false,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = one_edit_away(testcase.first.to_string(), testcase.second.to_string());
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_min_deletion_size() {
            struct TestCase {
                name: &'static str,
                strs: &'static [&'static str],
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    strs: &["cba", "daf", "ghi"],
                    expect: 1,
                },
                TestCase {
                    name: "basic 2",
                    strs: &["a", "b"],
                    expect: 0,
                },
                TestCase {
                    name: "basic 3",
                    strs: &["zyx", "wvu", "tsr"],
                    expect: 3,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let strs = testcase.strs.iter().map(|s| s.to_string()).collect();
                let actual = min_deletion_size(strs);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

        #[test]
        fn test_num_subarray_product_less_than_k() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[10, 5, 2, 6],
                    k: 100,
                    expect: 8,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[1, 2, 3],
                    k: 0,
                    expect: 0,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = num_subarray_product_less_than_k(testcase.nums.to_vec(), testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            });
        }

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

/// # 前缀和/前缀树
///
/// 特点: 前缀; 区间; 原数组不动
///
/// ## 题目
/// * 简单
///     * [303. 区域和检索 - 数组不可变](NumArray)
/// * 中等
///     * [304. 二维区域和检索 - 矩阵不可变](NumMatrix)
///     * [560. 和为 K 的子数组](subarray_sum)
/// * 困难
pub mod pre_sum {

    /// [303. 区域和检索 - 数组不可变](https://leetcode-cn.com/problems/range-sum-query-immutable/)
    #[allow(dead_code)]
    struct NumArray {
        pre: Vec<i32>,
    }

    #[allow(dead_code)]
    impl NumArray {
        fn new(nums: Vec<i32>) -> Self {
            let mut pre = vec![0];
            let mut curr_sum = 0;
            for num in nums {
                curr_sum += num;
                pre.push(curr_sum);
            }
            Self { pre }
        }

        fn sum_range(&self, left: i32, right: i32) -> i32 {
            let (mut left, mut right) = (left, right);
            if left < 0 {
                left = 0;
            }
            if right >= self.pre.len() as i32 {
                right = self.pre.len() as i32 - 2;
            }
            let (l, r) = (
                self.pre.get(left as usize).unwrap(),
                self.pre.get(right as usize + 1).unwrap(),
            );
            r - l
        }
    }

    /// [304. 二维区域和检索 - 矩阵不可变](https://leetcode-cn.com/problems/range-sum-query-2d-immutable/)
    #[allow(dead_code)]
    struct NumMatrix {
        pre: Vec<Vec<i32>>,
    }

    #[allow(dead_code)]
    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            let (row, col) = (matrix.len(), matrix.first().unwrap().len());
            let mut pre = vec![vec![0; col + 1]; row + 1];

            for r in 1..=row {
                // 从 1 开始的
                for c in 1..=col {
                    // 从1开始的, 含终点
                    pre[r][c] =
                        pre[r - 1][c] + pre[r][c - 1] - pre[r - 1][c - 1] + matrix[r - 1][c - 1];
                }
            }

            Self { pre }
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let (row1, col1, row2, col2) =
                (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
            self.pre[row2 + 1][col2 + 1] - self.pre[row1][col2 + 1] - self.pre[row2 + 1][col1]
                + self.pre[row1][col1]
        }
    }

    /// [560. 和为 K 的子数组](https://leetcode-cn.com/problems/subarray-sum-equals-k/)
    ///
    /// 和[1. 两数之和](https://leetcode-cn.com/problems/two-sum/) 相似, 只是这个是两数之差
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut curr = 0;
        let mut store = HashMap::new();

        // 用来处理 前缀和 恰好等于k 的情况
        // store.insert(0, 1);

        let mut ret = 0;
        for num in nums {
            curr += num;
            // 或者手动判是否为k, 手动+1
            if curr == k {
                ret += 1;
            }

            // 两数之差, 目标值
            let target = curr - k;
            ret += store.get(&target).unwrap_or(&0);

            *store.entry(curr).or_insert(0) += 1;
        }

        ret as i32
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut counter: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            let other = target - num;
            if let Some(v) = counter.get(&other) {
                return vec![v[0] as i32, idx as i32];
            }
            let entry = counter.entry(num).or_insert(Vec::<usize>::new());
            entry.push(idx);
        }

        vec![]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_subarray_sum() {
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                k: i32,
                expect: i32,
            }

            vec![
                TestCase {
                    name: "basic",
                    nums: &[1, 1, 1],
                    k: 2,
                    expect: 2,
                },
                TestCase {
                    name: "basic 2",
                    nums: &[1, 2, 3],
                    k: 3,
                    expect: 2,
                },
                TestCase {
                    name: "fix 1",
                    nums: &[1],
                    k: 0,
                    expect: 0,
                },
                TestCase {
                    name: "fix 2",
                    nums: &[-1, -1, 1],
                    k: 0,
                    expect: 1,
                },
            ]
            .iter()
            .for_each(|testcase| {
                let actual = subarray_sum(testcase.nums.to_vec(), testcase.k);
                assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
            })
        }

        #[test]
        fn test_sum_region() {
            struct Range {
                row1: i32,
                col1: i32,
                row2: i32,
                col2: i32,
            }

            struct TestCase {
                name: &'static str,
                matrix: &'static [&'static [i32]],
                query: &'static [(Range, i32)],
            }

            vec![TestCase {
                name: "basic",
                matrix: &[
                    &[3, 0, 1, 4, 2],
                    &[5, 6, 3, 2, 1],
                    &[1, 2, 0, 1, 5],
                    &[4, 1, 0, 1, 7],
                    &[1, 0, 3, 0, 5],
                ],
                query: &[
                    (
                        Range {
                            row1: 2,
                            col1: 1,
                            row2: 4,
                            col2: 3,
                        },
                        8,
                    ),
                    (
                        Range {
                            row1: 1,
                            col1: 1,
                            row2: 2,
                            col2: 2,
                        },
                        11,
                    ),
                    (
                        Range {
                            row1: 1,
                            col1: 2,
                            row2: 2,
                            col2: 4,
                        },
                        12,
                    ),
                ],
            }]
            .iter()
            .for_each(|testcase| {
                let nums = testcase.matrix.iter().map(|row| row.to_vec()).collect();
                let na = NumMatrix::new(nums);
                testcase
                    .query
                    .iter()
                    .enumerate()
                    .for_each(|(idx, (rng, expect))| {
                        let actual = na.sum_region(rng.row1, rng.col1, rng.row2, rng.col2);
                        assert_eq!(*expect, actual, "{} {} failed", testcase.name, &idx);
                    });
            })
        }

        #[test]
        fn test_sum_range() {
            struct Range {
                left: i32,
                right: i32,
            }
            struct TestCase {
                name: &'static str,
                nums: &'static [i32],
                query: &'static [(Range, i32)],
            }

            vec![TestCase {
                name: "basic",
                nums: &[-2, 0, 3, -5, 2, -1],
                query: &[
                    (Range { left: 0, right: 2 }, 1),
                    (Range { left: 2, right: 5 }, -1),
                    (Range { left: 0, right: 5 }, -3),
                ],
            }]
            .iter()
            .for_each(|testcase| {
                let na = NumArray::new(testcase.nums.to_vec());
                testcase
                    .query
                    .iter()
                    .enumerate()
                    .for_each(|(idx, (rng, expect))| {
                        let actual = na.sum_range(rng.left, rng.right);
                        assert_eq!(*expect, actual, "{} {} failed", testcase.name, &idx);
                    });
            })
        }
    }
}

/// 一些周边题目
pub mod ext;

pub mod ser;
