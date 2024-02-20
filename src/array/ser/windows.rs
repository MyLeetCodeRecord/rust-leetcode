//! # 滑动窗口
//!
//! 特点:
//! * 连续
//! * two pointers 的扩展
//!
//! 主要点: 什么时机动哪个边界.
//!
//! ## 题目链接
//! * 简单
//!     * [944. 删列造序](min_deletion_size)
//!     * [961. 在长度 2N 的数组中找出重复 N 次的元素](repeated_n_times)
//!     * [1200. 最小绝对差](minimum_abs_difference)
//!     * [896. 单调数列](is_monotonic)
//! * 中等
//!     * [2024. 考试的最大困扰度](max_consecutive_answers)
//!     * [1004. 最大连续1的个数 III](longest_ones)
//!     * [209. 长度最小的子数组](min_sub_array_len)
//!     * [567. 字符串的排列](check_inclusion)
//!     * [3. 无重复字符的最长子串](length_of_longest_substring)
//!     * [904. 水果成篮](total_fruit)
//!     * [438. 找到字符串中所有字母异位词](find_anagrams)
//!     * [713. 乘积小于 K 的子数组](num_subarray_product_less_than_k)
//!     * [面试题 01.05. 一次编辑](one_edit_away)
//! * 困难
//!     * [76. 最小覆盖子串](min_window)
//!
//! ## 总结
//! * 快慢指针的扩展, 即都是向一个方向跑的
//! * 题目都是要求连续的xxx, 常见的比如连续子串, 连续子数组, 即能组成一个窗口
//! * 核心在于什么时机移动哪个边界
//!     * 一般右边界O(N)逐次移动, 即 一个 `for right in 0..lenggh`
//!     * 左边界根据窗口的定义条件移动探索目标解
//!     * 有时窗口大小是固定的

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
            let (a, b) = (win.first().unwrap(), win.get(1).unwrap());
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
/// [845. 数组中的最长山脉](https://leetcode.cn/problems/longest-mountain-in-array/)
/// * 阶段1: [滑动窗口](crate::array::ser::windows::longest_mountain)
///     * 枚举山顶
///     * 找出最小的山, 然后向左向右扩展
///     * 优化: 如果序列是 "上山"(严格递增), 那必然不是 "下山"(严格递减)
///     * 每个元素最多比较两次
/// * 阶段2: [DP 解法](crate::dp::no_class::longest_mountain)
/// * 阶段3: [双指针](crate::array::ser::two_pointers::longest_mountain)
pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cursor = 2; // 向后错位, 防止溢出
    while cursor <= arr.len().saturating_sub(1) {
        let (prev, curr, next) = (arr[cursor - 2], arr[cursor - 1], arr[cursor]);
        if prev < curr && curr > next {
            // 找到了山顶, 开始向两边扩展
            let mut left = cursor;
            while left > 1 && arr[left - 2] < arr[left - 1] {
                left -= 1; // 向左一格
            }
            let mut right = cursor;
            while right < arr.len() && arr[right - 1] > arr[right] {
                right += 1; // 向右一格
            }
            let tmp = (cursor - left) + 1 + (right - cursor);
            ans = ans.max(tmp);

            cursor = right; // 下山的不会变上山, 因此可以跳过一点
            continue;
        }
        cursor += 1;
    }
    ans as i32
}

/// [852. 山脉数组的峰顶索引](https://leetcode.cn/problems/peak-index-in-a-mountain-array/)
/// - 解法1: [滑动窗口](crate::array::ser::windows::peak_index_in_mountain_array)
///     - 和 [845. 数组中的最长山脉](longest_mountain) 相似,
///     - 不过不同于前题, 题目保证只有一个山顶, 因此可以在发现时, 直接返回, 但整体时间复杂度仍为O(n)
/// - 解法 2&3: [二分](crate::array::ser::binary_search::peak_index_in_mountain_array)
///
pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    for (idx, win) in arr.windows(3).enumerate() {
        if let [a, b, c] = &win {
            if a < b && b > c {
                return (idx + 1) as i32;
            }
        }
    }
    unreachable!()
}

/// [896. 单调数列](https://leetcode.cn/problems/monotonic-array/)
///
/// 思路1:
/// 从现象上看, 必须出现一个 "波峰" 或者 "波谷", 才会破坏单调性
/// 即, 相邻三个元素 abc, 如果 a < b > c 或者 a > b < c, 则不是单调数列
/// 因此, 只需要判断是否出现了这种情况即可
///
/// 思路2的漏洞, 单调性发生变化, 比如 11,11,9,4,3,3,3,1,-1,-1,3,3,3,5,5,5
/// 有很多相等元素, 在窗口内符合, 但是单调性发生了变化
/// 形状类似 \___/
///
/// 如果使用变量存储单调性, 则效果不如思路1
///
/// 思路2:
/// 可以利用排除法, 比如出现了 a < b, 则一定不是单调递减
/// 出现了 a > b, 则一定不是单调递增
/// 最后看是否有一种情况出现即可
pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut inc = true;
    let mut dec = true;
    for win in nums.windows(2) {
        if let [a, b] = &win {
            if a > b {
                inc = false;
            }
            if a < b {
                dec = false;
            }
        }
    }
    inc || dec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_monotonic() {
        struct TestCase {
            nums: Vec<i32>,
            expect: bool,
        }

        vec![
            TestCase {
                nums: vec![1, 2, 2, 3],
                expect: true,
            },
            TestCase {
                nums: vec![6, 5, 4, 4],
                expect: true,
            },
            TestCase {
                nums: vec![1, 3, 2],
                expect: false,
            },
            TestCase {
                nums: vec![1, 2, 2, 2],
                expect: true,
            },
            TestCase {
                nums: vec![11, 11, 9, 4, 3, 3, 3, 1, -1, -1, 3, 3, 3, 5, 5, 5],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, case)| {
            assert_eq!(is_monotonic(case.nums), case.expect, "{} failed", idx);
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
    fn test_longest_mountain() {
        struct TestCase {
            arr: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                arr: vec![2, 1, 4, 7, 3, 2, 5],
                expect: 5,
            },
            TestCase {
                arr: vec![2, 2, 2],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { arr, expect } = testcase;
            let actual = longest_mountain(arr);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_repeated_n_times() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
        .iter()
        .for_each(|testcase| {
            let answer = testcase.answer.to_string();
            let actual = max_consecutive_answers(answer, testcase.k);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
