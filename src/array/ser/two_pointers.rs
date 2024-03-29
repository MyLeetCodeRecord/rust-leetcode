//! # 双指针
//! > swap 不就是双指针吗
//! 特点: 就地; 相对
//!
//! ## 概念
//! 双指针主要有 **快慢指针**, **左右指针**.
//! * 快慢指针: 两个指针同向而行，一快一慢
//! * 左右指针: 两个指针相向而行或者相背而行
//!
//! 落到具体数据结构中
//! * 数组中用索引代替指针
//! * 单链表只有快慢指针
//!
//! 其他形式的变种
//! * 滑动窗口
//! * 二分 可以视为左右指针
//! * 归并排序
//!
//! ## 题目链接
//! * 简单:
//!     * [27. 移除元素](remove_element)
//!     * [26. 删除有序数组中的重复项](remove_duplicates)
//!     * [283. 移动零](move_zeroes)
//!     * [344. 反转字符串](reverse_string)
//!     * [977. 有序数组的平方](sorted_squares)
//!     * [541. 反转字符串 II](reverse_str)
//!     * [剑指 Offer 05. 替换空格](replace_space)
//!     * [88. 合并两个有序数组](merge)
//! * 中等:
//!     * [167. 两数之和 II - 输入有序数组](two_sum)
//!     * [5. 最长回文子串](longest_palindrome)
//!     * [870. 优势洗牌](advantage_count)
//!     * [186. 翻转字符串里的单词 II](reverse_words)
//!     * [151. 颠倒字符串中的单词](reverse_words_1)
//!     * [面试题 17.11. 单词距离](find_closest)
//!     * [1089. 复写零](duplicate_zeros)
//!     * [1750. 删除字符串两端相同字符后的最短长度](minimum_length)
//! * [链表类型](crate::list::ser::two_pointers)
//!

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
    while let Some((index, val)) = nums2.pop() {
        
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
    let k = k as usize;
    let mut bytes = s.into_bytes();

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
    String::from_utf8(bytes).unwrap()
}

/// [剑指 Offer 05. 替换空格](https://leetcode-cn.com/problems/ti-huan-kong-ge-lcof/)
pub fn replace_space(s: String) -> String {
    let mut bytes = s.into_bytes();
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
    String::from_utf8(bytes).unwrap()
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
    let mut bytes = s.into_bytes();
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
    String::from_utf8(bytes).unwrap()
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

/// [838. 推多米诺](https://leetcode.cn/problems/push-dominoes/)
///
/// 思路1: 复杂度 O2
/// ```rust
/// pub fn push_dominoes(dominoes: String) -> String {
///     let mut dominoes = dominoes;
///     let s = unsafe { dominoes.as_bytes_mut() };
///     // 只需要关注前面最后一个向右倒的, 以及后面最早一个向左倒的
///     // 其他组合不会影响当前的状态, 可以不记录
///     let (mut last_right, mut next_left): (Option<usize>, Option<usize>) = (None, None);
///
///     let mut cursor = 0;
///     while cursor < s.len() {
///         if s[cursor] == b'R' {
///             last_right.replace(cursor);
///         } else if s[cursor] == b'L' {
///             last_right.take();
///         } else if s[cursor] == b'.' {
///             next_left.take();
///             for j in cursor + 1..s.len() {
///                 if s[j] == b'L' {
///                     next_left.replace(j);
///                     break;
///                 } else if s[j] == b'R' {
///                     break;
///                 }
///             }
///             if last_right.is_none() && next_left.is_none() {
///                 // 不变
///             } else if last_right.is_none() {
///                 s[cursor] = b'L';
///             } else if next_left.is_none() {
///                 s[cursor] = b'R';
///             } else {
///                 let (r, l) = (last_right.unwrap(), next_left.unwrap());
///                 match (cursor - r).cmp(&(l - cursor)) {
///                     std::cmp::Ordering::Equal => {}
///                     std::cmp::Ordering::Greater => {
///                         s[cursor] = b'L';
///                     }
///                     std::cmp::Ordering::Less => {
///                         s[cursor] = b'R';
///                     }
///                 }
///             }
///
///             // last_right.take();
///             // if s[cursor] == b'R' {
///             //     last_right.replace(cursor);
///             // }
///         }
///         cursor += 1;
///     }
///     dominoes
/// }
/// ```
///
pub fn push_dominoes(dominoes: String) -> String {
    let mut s = dominoes.into_bytes();

    let (mut cursor, mut last) = (0, None::<u8>);

    while cursor < s.len() {
        if s[cursor] == b'R' || s[cursor] == b'L' {
            last.replace(s[cursor]);
            cursor += 1;
            continue;
        }

        let mut end = cursor;
        while end + 1 < s.len() && s[end + 1] == b'.' {
            end += 1;
        }

        let tmp = end + 1; // 暂存
        let next = s.get(tmp).copied();
        // cursor, end
        match (last, next) {
            (Some(b'R'), Some(b'L')) => {
                while cursor < end {
                    s[cursor] = b'R';
                    s[end] = b'L';

                    cursor += 1;
                    end -= 1;
                }
            }
            (_, Some(b'L')) => {
                for i in cursor..tmp {
                    s[i] = b'L';
                }
            }
            (Some(b'R'), _) => {
                for i in cursor..tmp {
                    s[i] = b'R';
                }
            }
            _ => {}
        }
        cursor = tmp;
    }
    String::from_utf8(s).unwrap()
}

/// [844. 比较含退格的字符串](https://leetcode.cn/problems/backspace-string-compare/)
/// 栈模拟版本 [844. 比较含退格的字符串](crate::array::ser::stack::backspace_compare)
pub fn backspace_compare(s: String, t: String) -> bool {
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let (mut si, mut ti) = (s.len(), t.len()); // 留一个偏移, 防止usize溢出
    let (mut sd, mut td) = (0, 0);
    while si > 0 || ti > 0 {
        // or
        while si > 0 {
            if s[si - 1] == b'#' {
                // 回退符, 计数, 跳过
                sd += 1;
                si -= 1;
            } else if sd > 0 {
                // 需要回退, 回退一格
                sd -= 1;
                si -= 1;
            } else {
                break;
            }
        }
        while ti > 0 {
            if t[ti - 1] == b'#' {
                td += 1;
                ti -= 1;
            } else if td > 0 {
                td -= 1;
                ti -= 1;
            } else {
                break;
            }
        }

        if ti != 0 && si != 0 {
            // 同时没有到头, 判断一下
            if s[si - 1] != t[ti - 1] {
                return false;
            }
            // 相同, 两边同时向前一格
            si -= 1;
            ti -= 1;
        } else if ti != 0 || si != 0 {
            // 有一个还没有结束
            return false;
        }
    }
    true
}

/// [845. 数组中的最长山脉](https://leetcode.cn/problems/longest-mountain-in-array/)
/// * 阶段1: [滑动窗口](crate::array::ser::windows::longest_mountain)
/// * 阶段2: [DP 解法](crate::dp::no_class::longest_mountain)
/// * 阶段3: [双指针](crate::array::ser::two_pointers::longest_mountain)
///     * 区别于 滑动窗口的枚举山顶, 可以枚举山脚
pub fn longest_mountain(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    let mut left = 0;
    while left + 2 < n {
        let mut right = left + 1;
        // 假定自己在左边山脚
        if arr[left] < arr[right] {
            // 阶段1, 先上山
            while right + 1 < n && arr[right] < arr[right + 1] {
                right += 1;
            }
            // 这时right在山顶
            // 阶段2: 下山
            if right + 1 < n && arr[right] > arr[right + 1] {
                while right + 1 < n && arr[right] > arr[right + 1] {
                    right += 1;
                }
                ans = ans.max(right - left + 1);
            }
            // 这时right在右山脚
        }
        // 已经检查过的, 不用再重复, 可以跳过
        left = right;
    }
    ans as i32
}

/// [1750. 删除字符串两端相同字符后的最短长度](https://leetcode.cn/problems/minimum-length-of-string-after-deleting-similar-ends/)
pub fn minimum_length(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let bs = s.as_bytes();
    let (mut left, mut right) = (1, bs.len()); // 使用[1, n], 防溢出

    while left < right {
        // 没等号
        let (l, r) = (bs[left - 1], bs[right - 1]);
        if l != r {
            break;
        }
        while left <= right {
            // 有等号
            if bs[left - 1] == l {
                left += 1;
                continue;
            }
            break;
        }
        while left <= right {
            // 有等号
            if bs[right - 1] == r {
                right -= 1;
                continue;
            }
            break;
        }
    }
    (right + 1 - left) as i32 // 先加1, 否则会溢出
}

/// [2037. 使每位学生都有座位的最少移动次数](https://leetcode.cn/problems/minimum-number-of-moves-to-seat-everyone/)
///
/// 思路: 局部最优之后, 全局最优
/// 局部最优, 就是两组内最接近的两个. 最接近的方式, 按照从小到大排序, 两两配对
pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort();
    students.sort();

    seats
        .iter()
        .zip(students.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum()
}
/// [849. 到最近的人的最大距离](https://leetcode.cn/problems/maximize-distance-to-closest-person/)
///
/// 1. 阶段1: [dp解法](crate::dp::no_class::max_dist_to_closest)
/// 2. 阶段2: [双指针解法](crate::array::ser::two_pointers::max_dist_to_closest)
///     * 和[845. 数组中的最长山脉](crate::array::ser::two_pointers::longest_mountain)相似, 向后探测
///     * 距离是向左向右两侧的, 取其中的最小值
///     * 成段的0, 只有中间那个才可能是合适位置
///
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len() as i32;
    let mut prev = -n;
    let mut ans = 0;

    let mut cursor = 0;
    while cursor < seats.len() {
        if seats[cursor] == 1 {
            prev = cursor as i32;
            cursor += 1;
            continue;
        }
        let mut future = cursor;
        while future < seats.len() && seats[future] == 0 {
            future += 1;
        }
        let length = (future - cursor) as i32;
        if prev < 0 {
            // 开头, 只有右端找到了1
            ans = ans.max(length);
        } else if future == seats.len() {
            // 结尾, 只有左端有1
            ans = ans.max(length);
        } else if length % 2 == 0 {
            ans = ans.max(length / 2);
        } else {
            ans = ans.max(length / 2 + 1);
        }
        cursor = future; // 成段的0, 只需要判断一次
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves_to_seat() {
        struct TestCase {
            seats: Vec<i32>,
            students: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                seats: vec![3, 1, 5],
                students: vec![2, 7, 4],
                expect: 4,
            },
            TestCase {
                seats: vec![4, 1, 5, 9],
                students: vec![1, 3, 2, 6],
                expect: 7,
            },
            TestCase {
                seats: vec![2, 2, 6, 6],
                students: vec![1, 3, 2, 6],
                expect: 4,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase {
                seats,
                students,
                expect,
            } = testcase;
            let actual = min_moves_to_seat(seats, students);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_max_dist_to_closest() {
        struct TestCase {
            seats: Vec<i32>,
            expect: i32,
        }

        vec![
            TestCase {
                seats: vec![1, 0, 0, 0, 1, 0, 1],
                expect: 2,
            },
            TestCase {
                seats: vec![1, 0, 0, 0],
                expect: 3,
            },
            TestCase {
                seats: vec![0, 1],
                expect: 1,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { seats, expect } = testcase;
            let actual = max_dist_to_closest(seats);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_minimum_length() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase { s: "ca", expect: 2 },
            Testcase {
                s: "cabaabac",
                expect: 0,
            },
            Testcase {
                s: "aabccabba",
                expect: 3,
            },
            Testcase {
                s: "aaa",
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = minimum_length(s.to_string());
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
    fn test_backspace_compare() {
        struct TestCase {
            s: &'static str,
            t: &'static str,
            expect: bool,
        }

        vec![
            TestCase {
                s: "ab#c",
                t: "ad#c",
                expect: true,
            },
            TestCase {
                s: "ab##",
                t: "c#d#",
                expect: true,
            },
            TestCase {
                s: "a#c",
                t: "b",
                expect: false,
            },
            TestCase {
                s: "a##c",
                t: "#a#c",
                expect: true,
            },
            TestCase {
                s: "bxj##tw",
                t: "bxo#j##tw",
                expect: true,
            },
            TestCase {
                s: "bbbextm",
                t: "bbb#extm",
                expect: false,
            },
            TestCase {
                s: "nzp#o#g",
                t: "b#nzp#o#g",
                expect: true,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, t, expect } = testcase;
            let actual = backspace_compare(s.to_string(), t.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_push_dominoes() {
        struct TestCase {
            dominoes: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                dominoes: "RR.L",
                expect: "RR.L",
            },
            TestCase {
                dominoes: ".L.R...LR..L..",
                expect: "LL.RR.LLRRLL..",
            },
            TestCase {
                dominoes: "R.R.L",
                expect: "RRR.L",
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { dominoes, expect } = testcase;
            let actual = push_dominoes(dominoes.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_duplicate_zeros() {
        struct TestCase {
            name: &'static str,
            arr: &'static [i32],
            expect: &'static [i32],
        }

        [TestCase {
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
            }]
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

        [TestCase {
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

        [TestCase {
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
            }]
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

        [TestCase {
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

        [TestCase {
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
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

        [TestCase {
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
            }]
        .iter()
        .for_each(|testcase| {
            let mut tmp = testcase.nums.to_vec();
            let actual = remove_duplicates(&mut tmp) as usize;
            assert_eq!(testcase.expect.len(), actual, "{} failed", testcase.name);
            assert_eq!(testcase.expect, &tmp[..actual], "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_move_zeroes() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: &'static [i32],
        }

        [TestCase {
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
            }]
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

        [TestCase {
                name: "basic",
                s: &['h', 'e', 'l', 'l', 'o'],
                expect: &['o', 'l', 'l', 'e', 'h'],
            },
            TestCase {
                name: "basic 2",
                s: &['H', 'a', 'n', 'n', 'a', 'h'],
                expect: &['h', 'a', 'n', 'n', 'a', 'H'],
            }]
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
            s: &'static str,
            expect: &'static str,
        }

        vec![
            TestCase {
                s: "babad",
                expect: "bab",
            },
            TestCase {
                s: "cbbd",
                expect: "bb",
            },
            TestCase {
                s: "a",
                expect: "a",
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcae)| {
            let actual = longest_palindrome(testcae.s.to_string());
            assert_eq!(testcae.expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_sorted_squares() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: &'static [i32],
        }

        [TestCase {
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
            }]
        .iter()
        .for_each(|testcase| {
            let actual = sorted_squares(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }
}
