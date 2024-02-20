//! 特点:
//! * 去重
//! * 检测是否已经存在(比如无限循环)
//!
//! 可以使用set, 也可以根据数据范围, 手动维护hash
//!
//! ## 题目
//! * 简单
//!     * [349. 两个数组的交集](intersection)
//!     * [350. 两个数组的交集 II](intersect)
//!     * [1002. 查找共用字符](common_chars)
//!     * [202. 快乐数](is_happy)
//!     * [2351. 第一个出现两次的字母](repeated_character)
//! * 中等
//!     * [433. 最小基因变化](min_mutation)
//!     * [890. 查找和替换模式](find_and_replace_pattern)
//! * 困难
//!     * [41. 缺失的第一个正数](first_missing_positive)

/// [349. 两个数组的交集](https://leetcode-cn.com/problems/intersection-of-two-arrays/)
///
/// 思路1:
/// 由于题目说数组内元素范围是[0, 1000], 因此可以使用长度1001的数组, 直接用索引做hash
/// ```
/// pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
///     let mut mark0 = vec![0; 1001];
///     let mut mark1 = vec![0; 1001];
///     for num in nums1 {
///         *mark0.get_mut(num as usize).unwrap() += 1;
///     }
///     for num in nums2 {
///         *mark1.get_mut(num as usize).unwrap() -= 1;
///     }
///     let mut result = vec![];
///     for idx in 0..mark0.len() {
///         if mark0[idx] * mark1[idx] < 0 {
///             result.push(idx as i32);
///         }
///     }
///     result
/// }
/// ```
/// 不过由于元素可能重复出现, 直接使用一个数组去存标记, 有些不方便.
///
/// 思路2: 使用set
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let nums1_set = nums1.into_iter().collect::<HashSet<i32>>();
    let nums2_set = nums2.into_iter().collect::<HashSet<i32>>();
    nums1_set.intersection(&nums2_set).copied().collect()
}

/// [350. 两个数组的交集 II](https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/)
/// 直接使用set, 会消除内部的重复数据, 不符合要求.
///
/// 思路1:
/// 使用1001长度的数组维护, 仍然有效
/// ```
/// pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
///     let mut mark0 = vec![0; 1001];
///     let mut mark1 = vec![0; 1001];
///     for num in nums1 {
///         *mark0.get_mut(num as usize).unwrap() += 1;
///     }
///     for num in nums2 {
///         *mark1.get_mut(num as usize).unwrap() += 1;
///     }
///     let mut result = vec![];
///     for idx in 0..mark0.len() {
///         let cnt = std::cmp::min(mark0[idx], mark1[idx]);
///         for _ in 1..=cnt {
///             result.push(idx as i32);
///         }
///     }
///     result
/// }
/// ```
/// 思路2:
/// 排序后合并数组的方式, 求交集
/// 这样同时可以处理两个数组过大的问题
/// 同时可以减少磁盘IO次数
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    nums1.sort();
    nums2.sort();

    let (mut a, mut b) = (0usize, 0usize);
    let mut result = vec![];
    while a < nums1.len() && b < nums2.len() {
        let x = nums1.get(a).copied().unwrap();
        let y = nums2.get(b).copied().unwrap();
        match x.cmp(&y) {
            std::cmp::Ordering::Equal => {
                result.push(x);
                a += 1;
                b += 1;
            }
            std::cmp::Ordering::Less => {
                a += 1;
            }
            std::cmp::Ordering::Greater => {
                b += 1;
            }
        }
    }
    result
}

/// [1002. 查找共用字符](https://leetcode-cn.com/problems/find-common-characters/)
///
/// 优化的点:
/// 可以不用全部存储, 只需要留下最小数量即可
pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut mark: Vec<[i32; 26]> = vec![];
    for word in words {
        let mut line = [0; 26];
        for c in word.chars() {
            let idx = c as u32 - 'a' as u32;
            *line.get_mut(idx as usize).unwrap() += 1;
        }
        mark.push(line);
    }

    let mut result = vec![];
    for i in 0..26 {
        let mut min = i32::MAX;
        for line in mark.iter() {
            let c = line.get(i).copied().unwrap();
            min = std::cmp::min(min, c);
        }
        for _ in 1..=min {
            result.push(char::from_u32('a' as u32 + i as u32).unwrap());
        }
    }
    result.into_iter().map(|c| c.to_string()).collect()
}

/// [202. 快乐数](https://leetcode-cn.com/problems/happy-number/)
pub fn is_happy(n: i32) -> bool {
    fn next(n: i32) -> i32 {
        let mut n = n;
        let mut digit = vec![];
        while n > 0 {
            digit.push(n % 10);
            n /= 10;
        }
        digit.into_iter().fold(0, |acc, d| acc + d * d)
    }

    use std::collections::HashSet;
    let mut mark = HashSet::new();
    let mut num = n;
    while num != 1 {
        if !mark.insert(num) {
            return false;
        }
        num = next(num);
    }
    true
}

/// [433. 最小基因变化](https://leetcode.cn/problems/minimum-genetic-mutation/)
///
/// BFS 和层序遍历有些相似, 需要记录自己在第几层
///
/// 可以利用set去重, 用map缓存数据去除重复计算
pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    struct Diff {
        cache: HashMap<String, HashMap<String, usize>>,
    }
    impl Diff {
        fn new() -> Self {
            Diff {
                cache: HashMap::new(),
            }
        }

        fn diff(&mut self, s1: &String, s2: &String) -> usize {
            if let Some(d) = self.hit(s1, s2) {
                return d;
            }

            let d = self.diff_cal(s1, s2);
            self.store(s1, s2, d);

            d
        }

        fn diff_cal(&self, s1: &String, s2: &String) -> usize {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            b1.iter().zip(b2.iter()).filter(|(c1, c2)| c1 != c2).count()
        }

        fn hit(&self, s1: &String, s2: &String) -> Option<usize> {
            if let Some(inner) = self.cache.get(s1) {
                if let Some(d) = inner.get(s2) {
                    return Some(*d);
                }
            }
            None
        }
        fn store(&mut self, s1: &String, s2: &String, d: usize) {
            self.cache
                .entry(s1.to_owned())
                .or_default()
                .insert(s2.to_owned(), d);
            self.cache
                .entry(s2.to_owned())
                .or_default()
                .insert(s1.to_owned(), d);
        }
    }

    let mut differ = Diff::new();
    let mut visited = HashSet::new();
    let mut deque = VecDeque::new();
    deque.push_back((&start, 0));

    while !deque.is_empty() {
        let (curr, lvl) = deque.pop_front().unwrap();
        if end.eq(curr) {
            return lvl;
        }

        visited.insert(curr);
        bank.iter()
            .filter(|&s| {
                if differ.diff(s, curr) <= 1 && !visited.contains(&s) {
                    return true;
                }
                false
            })
            .for_each(|s| {
                if !visited.contains(&s) {
                    deque.push_back((s, lvl + 1));
                }
            });
    }

    -1
}

/// [890. 查找和替换模式](https://leetcode.cn/problems/find-and-replace-pattern/)
///
/// 记录替换规则, 如果发现旧的替换规则不符合, 判定为不能替换
/// 注意是一一映射
pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    use std::collections::HashMap;

    words
        .into_iter()
        .filter(|word| {
            let mut ft = HashMap::with_capacity(26); // 旧 -> 新
            let mut tf = HashMap::with_capacity(26); // 新 -> 旧
            for (raw, target) in word.chars().zip(pattern.chars()) {
                let entry = tf.entry(target).or_insert(raw);
                if !raw.eq(entry) {
                    return false;
                }
                let entry = ft.entry(raw).or_insert(target);
                if !target.eq(entry) {
                    return false;
                }
            }
            true
        })
        .collect()
}

/// [41. 缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive/)
///
/// nums 最多能有 [1, nums.len()]
/// 因此可以利用nums自身作为hash表, 记录出现与否, 比如将数字对应位置变为负数
/// 由于存在负数, 因此可以将原本的负数 置为 Len + 1
///
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let l = nums.len();
    let mut nums = nums;

    nums.iter_mut().for_each(|x| {
        if *x <= 0 {
            *x = l as i32 + 1
        }
    });

    for i in 0..l {
        let pos = nums.get(i).copied().unwrap().unsigned_abs() as usize;
        if pos > l {
            continue;
        }
        let num = nums.get_mut(pos - 1).unwrap();
        *num = 0 - num.abs();
    }

    for i in 0..l {
        if *nums.get(i).unwrap() > 0 {
            return i as i32 + 1;
        }
    }

    l as i32 + 1
}

/// [2351. 第一个出现两次的字母](https://leetcode.cn/problems/first-letter-to-appear-twice/)
///
/// 题目说明只有26个小写字母, 但是返回类型时char, 在rust中转化起来费劲
pub fn repeated_character(s: String) -> char {
    use std::collections::HashMap;
    let mut counter = HashMap::new();
    for c in s.chars() {
        let entry = counter.entry(c).or_insert(0);
        *entry += 1;
        if *entry == 2 {
            return c;
        }
    }
    unreachable!("题目保证有解")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_character() {
        struct TestCase {
            s: String,
            expect: char,
        }

        vec![
            TestCase {
                s: "abccbaacz".to_string(),
                expect: 'c',
            },
            TestCase {
                s: "abcdd".to_string(),
                expect: 'd',
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = repeated_character(s);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_first_missing_positive() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        [TestCase {
                name: "basic 1",
                nums: &[1, 2, 0],
                expect: 3,
            },
            TestCase {
                name: "basic 2",
                nums: &[3, 4, -1, 1],
                expect: 2,
            },
            TestCase {
                name: "basic 3",
                nums: &[7, 8, 9, 11, 12],
                expect: 1,
            },
            TestCase {
                name: "fix 1",
                nums: &[1],
                expect: 2,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = first_missing_positive(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_and_replace_pattern() {
        struct TestCase {
            name: &'static str,
            words: &'static [&'static str],
            pattern: &'static str,
            expect: &'static [&'static str],
        }

        [TestCase {
            name: "basic 1",
            words: &["abc", "deq", "mee", "aqq", "dkd", "ccc"],
            pattern: "abb",
            expect: &["mee", "aqq"],
        }]
        .iter()
        .for_each(|testcase| {
            let words = testcase.words.iter().map(|s| s.to_string()).collect();
            let actual = find_and_replace_pattern(words, testcase.pattern.to_string());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_min_mutation() {
        struct TestCase {
            name: &'static str,
            start: &'static str,
            end: &'static str,
            bank: &'static [&'static str],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                start: "AACCGGTT",
                end: "AACCGGTA",
                bank: &["AACCGGTA"],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                start: "AACCGGTT",
                end: "AAACGGTA",
                bank: &["AACCGGTA", "AACCGCTA", "AAACGGTA"],
                expect: 2,
            },
            TestCase {
                name: "basic 3",
                start: "AAAAACCC",
                end: "AACCCCCC",
                bank: &["AAAACCCC", "AAACCCCC", "AACCCCCC"],
                expect: 3,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let bank = testcase.bank.iter().map(|s| s.to_string()).collect();
            let actual = min_mutation(testcase.start.to_string(), testcase.end.to_string(), bank);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_is_happy() {
        struct TestCase {
            name: &'static str,
            n: i32,
            expect: bool,
        }
        [TestCase {
                name: "basic",
                n: 19,
                expect: true,
            },
            TestCase {
                name: "basic 2",
                n: 2,
                expect: false,
            }]
        .iter()
        .for_each(|testcase| {
            let actual = is_happy(testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_common_chars() {
        struct TestCase {
            name: &'static str,
            words: &'static [&'static str],
            expect: &'static [&'static str],
        }

        [TestCase {
                name: "basic",
                words: &["bella", "label", "roller"],
                expect: &["e", "l", "l"],
            },
            TestCase {
                name: "basic 2",
                words: &["cool", "lock", "cook"],
                expect: &["c", "o"],
            }]
        .iter()
        .for_each(|testcase| {
            let words: Vec<String> = testcase.words.iter().map(|s| s.to_string()).collect();
            let mut actual = common_chars(words);
            let mut expect: Vec<String> = testcase.expect.iter().map(|s| s.to_string()).collect();
            actual.sort();
            expect.sort();

            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_intersect() {
        struct TestCase {
            name: &'static str,
            nums1: &'static [i32],
            nums2: &'static [i32],
            expect: &'static [i32],
        }

        [TestCase {
                name: "basic",
                nums1: &[1, 2, 2, 1],
                nums2: &[2, 2],
                expect: &[2, 2],
            },
            TestCase {
                name: "basic 2",
                nums1: &[4, 9, 5],
                nums2: &[9, 4, 9, 8, 4],
                expect: &[9, 4],
            }]
        .iter()
        .for_each(|testcase| {
            let mut actual = intersect(testcase.nums1.to_vec(), testcase.nums2.to_vec());
            actual.sort();
            let mut expect = testcase.expect.to_vec();
            expect.sort();

            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_intersection() {
        use std::collections::HashSet;
        struct TestCase {
            name: &'static str,
            nums1: &'static [i32],
            nums2: &'static [i32],
            expect: &'static [i32],
        }

        [TestCase {
                name: "basic",
                nums1: &[1, 2, 2, 1],
                nums2: &[2, 2],
                expect: &[2],
            },
            TestCase {
                name: "basic 2",
                nums1: &[4, 9, 5],
                nums2: &[9, 4, 9, 8, 4],
                expect: &[9, 4],
            }]
        .iter()
        .for_each(|testcase| {
            let actual = intersection(testcase.nums1.to_vec(), testcase.nums2.to_vec());
            let actual_set: HashSet<i32> = HashSet::from_iter(actual);
            let expect: HashSet<i32> = HashSet::from_iter(testcase.expect.iter().copied());

            assert_eq!(expect, actual_set, "{} failed", testcase.name);
        });
    }
}
