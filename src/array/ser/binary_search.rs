//! # 二分查找
//!
//! 特点:
//! 1. 数组(连续内存, 或可用索引随机访问) + 有序
//! 2. 时间复杂度O(logn)
//!
//! 看到要求O(logn)的, 多和二分有关
//!
//! ## 基础 - 区间选择 - 写法
//!
//! > 没有重复元素
//!
//! 之所以有区间的写法问题, 是因为口语上的左边一半右边一半, 没有描述清边界问题.
//! 但其实把握住: 终止条件为 **搜索区间内没有元素** 这一条就可以了.
//!
//! 下面所有的情况讨论都是基于升序的.
//!
//! 关于`left`和`right`, 有以下几种取法:
//! * 左闭右闭: 初始化用 `left, right = 0, len(nums)-1`
//!     * `left`, `right`都是索引值
//!     * `left`, `right`包含在搜索区间,
//!         * 因此如果判定 `target < nums[mid]`, 则`right = mid -1`, 不能`right = mid` 否则区间多一个
//!         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
//!         * 终止条件为`while left <= right` 有等号
//!     * 注意:
//!         * 由于存在`right = mid -1`, 因此存在`right`小于0的可能(就一个元素时)
//!         * 终止条件为 **搜索区间内没有元素**, 即`left > right`;
//!             * `left == right`时还包含一个元素
//! * 左闭右开: 初始化用 `left, right = 0, len(nums)`
//!     * `left`为索引, `right`为右边界
//!     * `left`在搜索区间, `right`不在, `right-1`在
//!         * 因此如果判定 `target < nums[mid]`, 则`right = mid`, 不能`right = mid-1` 否则区间少一个
//!         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
//!         * 终止条件为`while left < right`
//!     * 注意:
//!         * 终止条件为 **搜索区间内没有元素**,
//!         * `right-left`为区间内元素的个数, `left==right`时就没有元素了
//! * 左开右开: 初始化用 `left, right = 1, len(nums)`
//!     * `left`, `right`为第*x*个中的*x*
//!     * `left`, `right`的含义不再是索引, 而是字面上的第几个
//!         * 和*左闭右闭*相似, 等效于同时加1
//!         * 因此如果判定 `target < nums[mid]`, 则`right = mid -1`, 表示不再检查第`mid`个
//!         * 同理, 如果`target > nums[mid]`, 则`left = mid+1`
//!         * 终止条件为`while left <= right` 有等号
//!     * 注意:
//!         * 由于字面意义的顺序比索引大1, 取值时注意减去
//!         * `right`不存在比0小的可能
//! * 左开右闭: 没啥意义, 不考虑
//!
//! ## 扩展 - 重复
//!
//! > 有重复元素, 查找第一个/最后一个出现的位置
//!
//! 与基础的区别在于找到目标元素后, 不是立刻`return`, 而是更改边界,  
//! 下面都是基于查找 第一个出现位置， 最后一个出现位置相似
//!
//! 关于`left`和`right`, 有以下几种取法:
//! * 左闭右闭: 初始化用 `left, right = 0, len(nums)-1`
//!     * `left`, `right`在搜索区间,
//!         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
//!         * `target < nums[mid]`, 则`right = mid-1` 与基础二分相同
//!     * 在`target == num[mid]`的处理上有两种方式,
//!         * 表示第一次出现至少不在`mid`右边, 令 `right = mid`, `mid`可能就是最早出现的那个, 需要保留为候选
//!             * 这时, 不在是*搜索区间*, 而是*候选区间*, 终止条件变为至多保留一个目标值,
//!             * 循环变为`while left < right` 保证至少保留一个
//!             * 同时需要在`right = left + 1`时需要手动判定跳出, 否则死循环
//!             * 最终使用`left`值即可(此时`left`==`right`)
//!         * `right = mid - 1`
//!             * 如果`mid`为第一个, 则`[0, mid-1]`一定都比`nums[mid]`小, 最终`left == mid`跳出循环
//!             * 循环使用`while left <= right` 保证每一个都搜索到
//!             * 最终使用`left`值即可
//!     * 注意:
//!         * `right = mid - 1` 存在 -1的可能
//!         * 将`target < nums[mid]` 和 `target == nums[mid]`合并, 统一`right = mid-1`比较方便
//!
//!  * 左闭右开: 初始化用 `left, right = 0, len(nums)`
//!     * `left`在搜索区间, `right`不在搜索区间, `right-1`在
//!         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
//!         * `target < nums[mid]`, 则`right = mid` 与基础二分相同
//!     * `target == num[mid]`的处理上有两种方式,
//!         * 令`right = mid+1`, 第一次出现至少不在`mid`右边, 因为可能第一次出现就是`mid`, 保留作为候选
//!             * 此时候选区间定义变化, 不再是搜索区间,
//!             * 循环条件变为`while left < right-1`, 保证至少保留一个
//!             * 同时需要在`right = left + 2`时需要手动判定跳出, 否则死循环
//!             * 使用`right-1`作为可能值, 可能负数
//!             * 此时为 `left == right - 1`， 因此使用`left`也可
//!         * 令`right = mid`
//!             * 如果`mid`为第一个, 则`[0, mid)`一定都比`nums[mid]`小, 最终`left == mid`跳出循环
//!             * 循环条件为 `while left < right`, 保证每一个都搜索到
//!             * 使用`left`值即可
//!     * 注意:
//!         * `while left < right-1` 可转化为`while left+1 < right`, 否则可能会有负数
//!         * 将`target < nums[mid]` 和 `target == nums[mid]`合并, 统一`right = mid`比较方便
//! * 左开右开: 初始化用 `left, right = 1, len(nums)`
//!     * `left`, `right`的含义不再是索引, 而是字面上的第几个
//!         * 和*左闭右闭*相似, 等效于同时加1
//!         * `target > nums[mid]`, 则`left = mid+1` 与基础二分相同
//!         * `target < nums[mid]`, 则`right = mid-1` 与基础二分相同
//!     * `target == num[mid]`的处理上有两种方式
//!
//! 总结起来,
//! * 还是维持**搜索空间** 比较好处理.
//! * *左开右开* 对于空数组有奇效
//!     * 在rust的类型强语义情景下， *左开右开*可以避免溢出
//! * `left` `right`含义相同(同为索引, 或同为第x个)时,
//!     * 循环条件为`while left <= right`， 其他为 `while left < right`
//!     * 切换`right`时需要, `right = mid -1`
//! * 最终取值和`left`的含义相关
//!     * 如果 `left` 为索引， 则最终值
//!     * 如果 `left` 为第x个， 最终值为`left-1`
//!
//! ## 扩展 - 单调函数
//!
//! 前面 对`nums[mid]`和`target`判大小, 其实用数学描述即为
//!
//! ```math
//! f(i) = nums_{i} >= target
//!      = \begin{cases}
//!            False, & i < target_{first} ,\\
//!            True,  & i \ge target_{first}
//!        \end{cases}
//! ```
//!
//! 即在左区间为`0`, 右区间为`1`, 这样就构成了一个 **二分**.
//!
//! 因此利用该特性, 结合上面的边界移动, 即可完整一般形式的 二分搜索.
//!
//! ## 题目链接
//!
//! * 简单:
//!     * [704. 二分查找](search)
//!     * [35. 搜索插入位置](search_insert)
//! * 中等:
//!     * [34. 在排序数组中查找元素的第一个和最后一个位置](search_range)
//!     * [875. 爱吃香蕉的珂珂](min_eating_speed)
//!     * [1011. 在 D 天内送达包裹的能力](ship_within_days)
//!     * [2226. 每个小孩最多能分到多少糖果](maximum_candies)
//!     * [436. 寻找右区间](find_right_interval)
//!     * [33. 搜索旋转排序数组](search_2)
//!     * [2560. 打家劫舍 IV](min_capability)
//! * 困难
//!     * [668. 乘法表中第k小的数](find_kth_number)
//!

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
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let first = first_occur(0, nums.len() - 1, |i| nums[i] >= target);

    if first >= nums.len() || nums[first] != target {
        // 没有满足>=target的(第一个满足 >= target的, 已经超出长度范围了)
        // 有满足的, 但是不是 target, 是其他的
        return vec![-1, -1];
    }

    let last = last_occur(0, nums.len() - 1, |i| nums[i] <= target);
    if last >= nums.len() || nums[last] != target {
        return vec![-1, -1];
    }

    vec![first as i32, last as i32]
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
        n /= 5;
        cnt += n;
    }
    cnt
}

/// [793. 阶乘函数后 K 个零](https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function/)
///
/// 相对与[172. 阶乘后的零](trailing_zeroes), *172*是给定阶乘求结尾有几个0, *793*是给定数量, 求是多少的阶乘
///
/// 由 *172*可知, 一段范围内的阶乘, 0的数量不变, 即存在相等, 求边界
///
/// 题目 k 的范围是`[0, 10**9]`, 对应的阶乘范围很大, 可以检测 `i32::MAX` 的0的数量大于 10**9, 因此可以作为右边界
/// 但是实测, 用 `i32::MAX` 会超时.
///
/// 通过 *172* 可以得到, 右边界可以用 5*k
///
pub fn preimage_size_fzf(k: i32) -> i32 {
    fn left_bound(k: i32) -> i32 {
        let (mut left, mut right) = (0, 5 * k);
        while left <= right {
            let mid = left + (right - left) / 2;
            let count = trailing_zeroes(mid);
            if count >= k {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        right + 1
    }

    left_bound(k + 1) - left_bound(k)
}

/// [852. 山脉数组的峰顶索引](https://leetcode.cn/problems/peak-index-in-a-mountain-array/)
///
/// - 解法1: [滑动窗口](crate::array::ser::windows::peak_index_in_mountain_array)
/// - 解法2: 二分无重复
///   - 整体由 _严格单调递增_ 和 _严格单调递减_ 两段组成
///   - 目标是找到这个枢纽
///   - 两端取中, 取`mid`周边两个元素, mid-1, mid, mid+1
///     - 如果这三个元素递减, 则处在下山, 因此枢纽在左边
///     - 如果这三个元素递增, 则处在上山, 因此枢纽在右边
///     - 否则mid就是山顶
///
/// ```
/// pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
///     let (mut left, mut right) = (1usize, arr.len());
///     // 两端含义相同, 保留等号
///     while left + 2 <= right {
///         let mid = left + (right - left) / 2;
///         let (prev, curr, next) = (arr[mid - 2], arr[mid - 1], arr[mid]);
///         if prev > curr && curr > next {
///             // 下山ing
///             right = mid - 1;
///         } else if prev < curr && curr < next {
///             // 上山ing
///             left = mid + 1;
///         } else {
///             return (mid - 1) as i32;
///         }
///     }
///     if arr[left - 1] < arr[right - 1] {
///         right as i32 - 1
///     } else {
///         left as i32 - 1
///     }
/// }
/// ```
/// - 解法3: 二分有重复
///   - 对于相邻的两个, [a, b]
///     - 上山时, 都是 a < b, b可以是山顶
///     - 下山时, 都是 a > b, a可以是山顶
///     - 第一次出现   a > b 时, a为山顶
///     - 最后一次出现 a < b 时, b为山顶
///   - 因此可以按照有重复元素的二分查找
///
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    first_occur(1, arr.len() - 2, |i| arr[i] > arr[i + 1]) as i32
}

/// 要求序列满足 `[from, x]` cmp 返回 false, `[x+1, end]` 返回 true
/// 即一开始必须是 false
///
/// 返回的 `I`, 不保证 `cmp(I) == true`
/// 这个函数保证的是 `[from, I)` cmp不会返回 true, `[from, I)` 可能是空
///
/// 做了防溢出, `from`和`end`可以是0, 即传入索引
fn first_occur<F>(from: usize, end: usize, cmp: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut left, mut right) = (from, end);
    while left <= right {
        let mid = left + (right - left) / 2;
        if cmp(mid) {
            match mid.checked_sub(1) {
                Some(r) => {
                    right = r;
                }
                None => {
                    return mid;
                }
            }
        } else {
            left = mid + 1;
        }
    }
    right + 1
}

/// 要求序列满足 `[from, x]` cmp 返回 true, `[x+1, end]` 返回 false
/// 即一开始必须时 true
///
/// 返回的 `I` 不保证 `cmp(I) == false`
/// 这个函数保证的是 `(I, end]` cmp不会返回true, `(I, end]` 可能是空
///
/// 做了防溢出, `from`和`end`可以是0, 即传入索引
fn last_occur<F>(from: usize, end: usize, cmp: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let (mut left, mut right) = (from, end);
    while left <= right {
        let mid = left + (right - left) / 2;
        if cmp(mid) {
            left = mid + 1;
        } else {
            match mid.checked_sub(1) {
                Some(r) => {
                    right = r;
                }
                None => {
                    return 0;
                }
            }
        }
    }
    left.saturating_sub(1)
}

/// [2560. 打家劫舍 IV](https://leetcode.cn/problems/house-robber-iv/description/)
/// 
/// 即`f(y)`为最大偷取金额为`y`的情况下, 可以偷取的房屋最大数量
/// 显然`f(y)`是非递减函数(y越大, 可以选择的节点越多?)
/// 
/// 由于`f(y)`是非递减函数, 因此可以使用二分查找
pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let (mut min, mut max) = (nums.iter().min().copied().unwrap(), nums.iter().max().copied().unwrap());
    while min <= max{
        let mid = min + (max - min) / 2;

        let mut count = 0;
        let mut visited = false;
        for &num in nums.iter() {
            if num <= mid && !visited{
                count += 1;
                visited = true;
            } else {
                visited = false;
            }
        }

        // 非递减, 找最小
        // 因此等号在这里
        if count >= k{
            max = mid - 1;
        } else {
            min = mid + 1;
        }
    }
    min
}


/// [2529. 正整数和负整数的最大计数](https://leetcode.cn/problems/maximum-count-of-positive-integer-and-negative-integer)
/// 
/// 注意: 0 既不是正整数也不是负整数
/// 
/// 思路: 利用单调性, 找到第一个正整数和最后一个负整数, 然后计算各自的长度
pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let first_positive = {
        let tmp = first_occur(0, nums.len()-1, |i| nums[i] > 0);
        if tmp == 0{
            if nums[0] > 0{
                0i32
            } else {
                1i32
            }
        } else if tmp == nums.len(){
            nums.len() as i32
        } else {
            tmp as i32
        }

    };
    let last_negative = {
        let tmp = last_occur(0, nums.len()-1, |i| nums[i] < 0);
        if tmp == 0 {
            if nums[0] < 0{
                1i32
            } else {
                0i32
            }
        } else if tmp == nums.len(){
            nums.len() as i32
        } else {
            tmp as i32 + 1
        }
    };
    
    let pos_len = nums.len() as i32 - first_positive;
    let neg_len = last_negative;

    pos_len.max(neg_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_count(){
        struct TestCase{
            name: &'static str,
            nums: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase{
                name: "basic",
                nums: vec![-5, -4, -3, -2, -1, 1, 2, 3, 4, 5],
                expect: 5,
            },
            TestCase{
                name: "basic 2",
                nums: vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5],
                expect: 5,
            },
            TestCase{
                name: "basic 3",
                nums: vec![1, 2, 3, 4, 5],
                expect: 5,
            },
            TestCase{
                name: "basic 4",
                nums: vec![-5, -4, -3, -2, -1],
                expect: 5,
            },
            TestCase{
                name: "basic 5",
                nums: vec![0, 1, 2, 3, 4, 5],
                expect: 5,
            },
            TestCase{
                name: "basic 6",
                nums: vec![-5, -4, -3, -2, -1, 0],
                expect: 5,
            },
        ]
        .iter()
        .for_each(|TestCase{name, nums, expect}|{
            let actual = maximum_count(nums.to_vec());
            assert_eq!(*expect, actual, "{} failed", name);
        });
    }

    #[test]
    fn test_min_capability() {
        struct TestCase {
            nums: Vec<i32>,
            k: i32,
            expect: i32,
        }

        vec![
            TestCase {
                nums: vec![2, 3, 5, 9],
                k: 2,
                expect: 5,
            },
            TestCase {
                nums: vec![2, 7, 9, 3, 1],
                k: 2,
                expect: 2,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idc, TestCase { nums, k, expect })| {
            let acutal = min_capability(nums, k);
            assert_eq!(expect, acutal, "case {} failed", idc);
        });
    }

    #[test]
    fn test_peak_index_in_mountain_array() {
        struct TestCase {
            arr: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                arr: vec![0, 1, 0],
                expect: 1,
            },
            TestCase {
                arr: vec![0, 2, 1, 0],
                expect: 1,
            },
            TestCase {
                arr: vec![0, 10, 5, 2],
                expect: 1,
            },
            TestCase {
                arr: vec![3, 4, 5, 1],
                expect: 2,
            },
            TestCase {
                arr: vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19],
                expect: 2,
            },
            TestCase {
                arr: vec![3, 5, 3, 2, 0],
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { arr, expect } = testcase;
            let actual = peak_index_in_mountain_array(arr);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_preimage_size_fzf() {
        struct TestCase {
            name: &'static str,
            k: i32,
            expect: i32,
        }

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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
        [TestCase {
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
            }]
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
        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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
        [TestCase {
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
            }]
        .iter()
        .for_each(|testcase| {
            let actual = ship_within_days(testcase.weights.clone(), testcase.days);
            assert_eq!(actual, testcase.expect, "{} failed", testcase.name);
        })
    }
}
