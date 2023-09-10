//! # 数学相关题目
//!
//! ## 题目
//! * 简单
//!     * [1470. 重新排列数组](shuffle)
//!     * [1582. 二进制矩阵中的特殊位置](num_special)
//!     * [412. Fizz Buzz](fizz_buzz)
//!     * [1342. 将数字变成 0 的操作次数](number_of_steps)
//!     * [836. 矩形重叠](is_rectangle_overlap)
//!     * [867. 转置矩阵](transpose)
//! * 中等
//!     * [462. 最小操作次数使数组元素相等 II](min_moves2)
//!     * [667. 优美的排列 II](construct_array)
//!     * [1759. 统计同构子字符串的数目](count_homogenous)
//!     * [835. 图像重叠](largest_overlap)
//!     * [840. 矩阵中的幻方](num_magic_squares_inside)
//!     * [858. 镜面反射](mirror_reflection)
//! * 困难
//!     * [829. 连续整数求和](consecutive_numbers_sum)
//!     * [828. 统计子串中的唯一字符](unique_letter_string)

/// [462. 最少移动次数使数组元素相等 II](https://leetcode.cn/problems/minimum-moves-to-equal-array-elements-ii/)
///
/// 直觉是平均数, 甚至题目的两个示例都是平均数.
/// 但可以找一些极端的例子, 比如 `[1, 1, 1, 100]` 向平均数25靠拢, 需要150步, 而向1靠拢, 需要99, 显然更少.
///
/// 由于中位数性质, *所有数与中位数的绝对差之和最小*, 因此变化为中位数, 需要的步骤最少.
///
/// 中位数是按顺序排列的一组数据中居于中间位置的数.
/// 对于奇数序列, 有唯一中间, 对于偶数序列, 两个中值都检验一遍即可.
pub fn min_moves2(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    if length <= 1 {
        return 0;
    }

    let mut nums = nums;
    nums.sort();

    let a = nums.get(length / 2).copied().unwrap();
    let b = {
        if length % 2 == 0 {
            nums.get(length / 2).copied().unwrap()
        } else {
            nums.get(length / 2 - 1).copied().unwrap()
        }
    };

    let (a_sum, b_sum) = (
        nums.iter().map(|num| (*num - a).abs()).sum::<i32>(),
        nums.iter().map(|num| (*num - b).abs()).sum::<i32>(),
    );
    a_sum.min(b_sum)
}

/// [1470. 重新排列数组](https://leetcode.cn/problems/shuffle-the-array/)
///
/// 简单做, 可以申请额外数组, 然后用双数组交替填充.
///
/// nums[i] 被填到了 nums[2*i]; nums[i+n] 被填到了 nums[2*i+1]
///
/// 由于数值范围是 [1, 1000], 因此可以用低10位存原来的数, 高10位存新的值
///
/// **注意**: 为啥 需要 ` & 1023`? 难道 `x&1023 != x`?
/// 确实不等, 因为在迭代中数值在变化, 而这里需要的是旧值, 也就是低10位的, 因此需要 `&1023`
///
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut nums = nums;
    let n = n as usize;
    for i in 0..n {
        nums[2 * i] = nums[2 * i] | ((nums[i] & 1023) << 10);
        nums[2 * i + 1] = nums[2 * i + 1] | ((nums[i + n] & 1023) << 10);
    }
    nums.iter_mut().for_each(|num| {
        *num = *num >> 10;
    });
    nums
}

/// [1582. 二进制矩阵中的特殊位置](https://leetcode.cn/problems/special-positions-in-a-binary-matrix/)
pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    if mat.is_empty() {
        return 0;
    }
    let (h, w) = (mat.len(), mat.first().unwrap().len());

    use std::collections::HashSet;
    let mut visited_row = HashSet::new();
    let mut visited_col = HashSet::new();

    fn valid(mat: &[Vec<i32>], r: usize, c: usize, h: usize, w: usize) -> bool {
        for i in 0..h {
            if mat[i][c] == 1 && i != r {
                return false;
            }
        }
        for i in 0..w {
            if mat[r][i] == 1 && i != c {
                return false;
            }
        }
        return true;
    }

    let mut cnt = 0;

    for (r, line) in mat.iter().enumerate() {
        if visited_row.contains(&r) {
            continue;
        }
        for (c, e) in line.iter().enumerate() {
            if visited_col.contains(&c) {
                continue;
            }

            if *e == 0 {
                continue;
            }

            if valid(mat.as_slice(), r, c, h, w) {
                cnt += 1;
            }

            visited_col.insert(c);
            visited_row.insert(r);

            break;
        }
    }
    cnt
}

/// [667. 优美的排列 II](https://leetcode.cn/problems/beautiful-arrangement-ii/)
pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
    let mut result = (1..(n - k)).collect::<Vec<i32>>();
    let (mut i, mut j) = (n - k, n);
    while i <= j {
        result.push(i);
        if i != j {
            result.push(j);
        }
        i += 1;
        j -= 1;
    }
    result
}

/// [412. Fizz Buzz](https://leetcode.cn/problems/fizz-buzz/)
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .into_iter()
        .map(|num| {
            if num % 3 == 0 && num % 5 == 0 {
                "FizzBuzz".to_string()
            } else if num % 5 == 0 {
                "Buzz".to_string()
            } else if num % 3 == 0 {
                "Fizz".to_string()
            } else {
                num.to_string()
            }
        })
        .collect()
}

/// [1342. 将数字变成 0 的操作次数](https://leetcode.cn/problems/number-of-steps-to-reduce-a-number-to-zero/)
///
/// 思路1: 模拟
/// ```
/// pub fn number_of_steps(num: i32) -> i32 {
///     let mut num = num;
///     let mut cnt = 0;
///     while num > 0 {
///         if num % 2 == 0 {
///             num = num / 2;
///         } else {
///             num = num - 1;
///         }
///         cnt += 1;
///     }
///     cnt
/// }
/// ```
///
/// 思路2: 计算
/// 将num用用二进制表示, 则每次减1， 实际对应为 低位的 1变0， 每次除2, 实际对应为 整体右移一位
/// 也就是总的操作数为可以视为 将最高位1变为0 的步数
///
/// 其中右移总共 `31 - (leading_zero)` (有符号数, 第一位为符号位, 忽略)
/// 减1 次数, 即为1的个数
///
/// 注意: 如果原本为0， 这时 `31 - (leading_zero)`可能有溢出
///
pub fn number_of_steps(num: i32) -> i32 {
    (num.count_ones() + 31u32.checked_sub(num.leading_zeros()).unwrap_or(0)) as i32
}

/// [829. 连续整数求和](https://leetcode.cn/problems/consecutive-numbers-sum/)
///
/// 根据[等差数列的公式](https://en.wikipedia.org/wiki/Arithmetic_progression)
///
/// ```
/// pub fn consecutive_numbers_sum(n: i32) -> i32 {
///     fn is_k_consecutive(n: i32, k: i32) -> bool {
///         if k % 2 == 1 {
///             return n % k == 0;
///         }
///         return n % k > 0 && 2 * n % k == 0;
///     }
///     let mut ans = 0;
///     let mut k = 1;
///     while k * (k + 1) <= n * 2 {
///         if is_k_consecutive(n, k) {
///             ans += 1;
///         }
///         k += 1
///     }
///     return ans;
/// }
/// ```
/// 根据[等差数列的公式](https://en.wikipedia.org/wiki/Arithmetic_progression), 可以得到以下推导,  
/// 假定起始项为$a$, 差$d$为1 项数$k$, 和为$n$, 则
/// 
/// $$
/// n = \frac{k \times (a+a+(k-1))}{2}
/// $$
///
/// 等式变化可以得到
///
/// $n = \frac{k \times (a+a+(k-1))}{2} \Rightarrow 2n = k(2a+(k-1)) \Rightarrow \frac{2n}{k} = 2a + k - 1$
///
/// 由于题目要求起始项需要为*正整数*, 也就是$2a + k - 1$为正整数, 因此 `2 * n % k == 0`
///
/// 同时有$2a \ge 2$, 所以 $2a + k - 1 \ge k + 1 \Rightarrow \frac{2n}{k} \ge k + 1$
///
/// 综上, $k$为$2n$的约数, 且是较小的那个.
///
/// 因此在$[1, \sqrt{2n}]$的范围内找$k$即可.
pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut ans = 0;
    let n = 2 * n;
    let mut k = 1;

    while k * k < n {
        if n % k != 0 {
            k += 1;
            continue;
        }
        if (n / k - (k - 1)) % 2 == 0 {
            ans += 1;
        }
        k += 1;
    }
    ans
}

/// [828. 统计子串中的唯一字符](https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string/)
///
/// 按照题意, 很容易写入如下枚举的方式, 时间复杂度 $O(n^3 + c \times n^2)$  
/// 枚举边界 $n^2$, 内层统计个数 $n + C$  
/// 最后超时
/// ```
/// pub fn unique_letter_string(s: String) -> i32 {
///     fn count_unique_chars(s: &str) -> usize {
///         use std::collections::HashMap;
///         let mut counter = HashMap::new();
///         for chr in s.chars() {
///             *counter.entry(chr).or_insert(0) += 1;
///         }
///         counter.into_iter().filter(|(_, v)| *v == 1).count()
///     }
///     let mut cnt = 0;
///     for i in 0..s.len() {
///         for j in i..s.len() {
///             cnt += count_unique_chars(s.get(i..=j).unwrap());
///         }
///     }
///     cnt as i32
/// }
/// ```
///
/// 优化方向: 将内层的遍历换成类似"滑动窗口", 不做重复统计
/// ```
/// pub fn unique_letter_string(s: String) -> i32 {
///     use std::collections::HashMap;
///
///     let s = s.as_bytes();
///
///     let mut cnt = 0;
///     for i in 0..s.len() {
///         let mut counter = HashMap::new();
///         for j in i..s.len() {
///             {
///                 *counter.entry(s[j]).or_insert(0) += 1;
///             }
///             cnt += counter.iter().filter(|(_, &v)| v == 1).count();
///         }
///     }
///     cnt as i32
/// }
/// ```
/// 时间复杂度 $O(n^2 + c \times n^2)$, 仍然超时
///
/// 其他思路:
/// 对于字串 `BCADEF`, 假定其前后都是`A`, 即`(A)BCADEF(A)`, 含统计字符`A`的唯一串有
/// `BCA, BCAD, BCADE, BCADEF,  CA, CAD, CADE, CADEF, A, AD, ADE, ADEF`
/// 站在`BC`的角度看, 后面有4种选择, `(), D, DE, DEF`
/// 站在`DEF`的角度看, 前面有3种选择, `BC, C, ()`
///
/// 所以这个段内, 含统计`A`字符的唯一串有 3 * 4 = 12, 即 (2+1) * (3+1),
/// 2为当前A到前一个A之间字符个数, 3为当前A到后一个A之间字符个数
///
/// 基于此, 可以对任意一个`(A)..A..(A)`进行计算, 最终加和即为结果
///
/// 边界情况: 字符只出现一次
///
pub fn unique_letter_string(s: String) -> i32 {
    let s = s.as_bytes();

    let mut last_pos = vec![-1; 26];
    let mut curr_pos = vec![-1; 26];

    let mut ans = 0;

    for (pos, &b) in s.iter().enumerate() {
        let i = (b - b'A') as usize;
        let pos = pos as i32;
        if curr_pos[i] > -1 {
            // 这个字符之前出现过, 这时的pos对应上述推导中的后一个
            ans = ans + (pos - curr_pos[i]) * (curr_pos[i] - last_pos[i]);
        }
        last_pos[i] = curr_pos[i];
        curr_pos[i] = pos;
    }
    // 对于只出现过一次的字符, 上面循环统计不到, 即等效 后一个 的位置为字符结尾
    for (last, curr) in last_pos.into_iter().zip(curr_pos.into_iter()) {
        if curr > -1 {
            ans = ans + (curr - last) * (s.len() as i32 - curr);
        }
    }
    ans
}

/// [835. 图像重叠](https://leetcode.cn/problems/image-overlap/)
pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let n = img1.len();
    let mut delta = HashMap::new();
    for i in 0..img1.len() {
        for j in 0..img1.len() {
            if img1[i][j] != 1 {
                continue;
            }
            for i2 in 0..img2.len() {
                for j2 in 0..img2.len() {
                    if img2[i2][j2] != 1 {
                        continue;
                    }
                    *delta.entry((i + n - i2, j + n - j2)).or_insert(0) += 1;
                }
            }
        }
    }
    delta.values().into_iter().max().copied().unwrap_or(0)
}

/// [836. 矩形重叠](https://leetcode.cn/problems/rectangle-overlap/)
/// 题目的矩形, 是靠 左下角/右上角 两个顶点确定的, 因此必然是垂直于X轴/Y轴的
///
/// 思路1: 排除法, 将没有交叠的情况枚举出来, 取反
/// ```
/// pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
///     use std::convert::TryFrom;
///     let [x1, y1, x2, y2] = <[i32; 4]>::try_from(rec1).ok().unwrap();
///     let [a1, b1, a2, b2] = <[i32; 4]>::try_from(rec2).ok().unwrap();
///     if x1 == x2 || y1 == y2 || a1 == a2 || b1 == b2 {
///         return false;
///     }
///     // 排除法
///     !(
///         x1 >= a2 || // rec1 在 rec2 的右边
///         x2 <= a1 || // rec1 在 rec2 的左边
///         y1 >= b2 || // rec1 在 rec2 的上方
///         y2 <= b1    // rec1 在 rec2 的下方
///     )
/// }
/// ```
/// 
/// 思路2: 投影
/// 如果有交叠, 投影到X轴的后, 两线段应该相交, 同理投影Y轴
#[rustfmt::skip]
pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    use std::convert::TryFrom;
    use std::cmp::{min, max};
    let [x1, y1, x2, y2] = <[i32; 4]>::try_from(rec1).ok().unwrap();
    let [a1, b1, a2, b2] = <[i32; 4]>::try_from(rec2).ok().unwrap();
    
    min(x2, a2) > max(x1, a1) && min(y2, b2) > max(y1, b1)
}

/// [840. 矩阵中的幻方](https://leetcode.cn/problems/magic-squares-in-grid/)
///
/// 垃圾题?
#[rustfmt::skip]
pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let (_r, _c) = (grid.len(), grid.first().unwrap().len());
    if _r < 3 || _c < 3 {
        return 0;
    }

    fn magic(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32) -> bool {
        let mut x = [a, b, c, d, e, f, g, h, i];
        x.sort();
        if !x.eq(&[1, 2, 3, 4, 5, 6, 7, 8, 9]) {
            return false;
        }
        [
            a + b + c, d + e + f, g + h + i,
            a + d + g, b + e + h, c + f + i,
            a + e + i, c + e + g,
        ]
        .iter()
        .all(|&x| x == 15)
    }

    let mut ans = 0;
    for r in 0.._r - 2 {
        for c in 0.._c - 2 {
            if grid[r + 1][c + 1] != 5 {
                continue;
            }
            if magic(
                grid[r][c], grid[r][c + 1], grid[r][c + 2],
                grid[r + 1][c], grid[r + 1][c + 1], grid[r + 1][c + 2],
                grid[r + 2][c], grid[r + 2][c + 1], grid[r + 2][c + 2],
            ) {
                ans += 1;
            }
        }
    }
    ans
}

/// [1759. 统计同构子字符串的数目](https://leetcode.cn/problems/count-number-of-homogenous-substrings/)
pub fn count_homogenous(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    const MODULO: i64 = 1_000_000_007;
    let mut chrs = s.chars();

    let mut sum = 0;
    let mut last = chrs.next().unwrap();
    let mut count: i64 = 1;

    for c in chrs {
        if last == c {
            count += 1;
            continue;
        }
        let cnt = (count + 1) * count / 2;
        sum = (sum + (cnt % MODULO) as i32) % MODULO as i32;

        last = c;
        count = 1;
    }

    let cnt = (count + 1) * count / 2;
    sum = (sum + (cnt % MODULO) as i32) % MODULO as i32;

    sum
}

/// [858. 镜面反射](https://leetcode.cn/problems/mirror-reflection/)
///
/// 每反射一次, 在y轴方向最终上升距离为q,
/// 如果上升 n 次后, n*q 为 p 的整数倍, 则到达了顶点
///
/// 也就是求最小公倍数
///
pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    let d = lcm(p, q);
    if (d / p) % 2 == 0 {
        return 0;
    } else if (d / q) % 2 == 0 {
        return 2;
    }
    return 1;
}

fn gcd(a: i32, b: i32) -> i32 {
    let (m, n) = (a.max(b), a.min(b));
    if n == 0 {
        return m;
    }
    return gcd(n, m % n);
}
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

/// [867. 转置矩阵](https://leetcode.cn/problems/transpose-matrix/)
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (matrix.len(), matrix.first().unwrap().len());
    let mut ret = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            ret[j][i] = matrix[i][j];
        }
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_transpose() {
        struct TestCase {
            matrix: Vec<Vec<i32>>,
            expect: Vec<Vec<i32>>,
        }

        vec![
            TestCase {
                matrix: vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]],
                expect: vec2![[1, 4, 7], [2, 5, 8], [3, 6, 9]],
            },
            TestCase {
                matrix: vec2![[1, 2, 3], [4, 5, 6]],
                expect: vec2![[1, 4], [2, 5], [3, 6]],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, TestCase { matrix, expect })| {
            let actual = transpose(matrix);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    #[rustfmt::skip]
    fn test_count_homogenous() {
        struct Testcase {
            s: &'static str,
            expect: i32,
        }

        vec![
            Testcase { s: "abbcccaa", expect: 13},
            Testcase { s: "xy", expect: 2 },
            Testcase { s: "zzzzz", expect: 15},
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { s, expect } = testcase;
            let actual = count_homogenous(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    #[rustfmt::skip]
    fn test_mirror_reflection() {
        struct TestCase {
            p: i32,
            q: i32,
            expect: i32,
        }

        vec![
            TestCase {p: 2, q: 1, expect: 2,}
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { p, q, expect } = testcase;
            let actual = mirror_reflection(p, q);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_num_magic_squares_inside() {
        struct Testcase {
            grid: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                grid: vec2![[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]],
                expect: 1,
            },
            Testcase {
                grid: vec2![[8]],
                expect: 0,
            },
            Testcase {
                grid: vec2![[5, 5, 5], [5, 5, 5], [5, 5, 5]],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { grid, expect } = testcase;
            let acutal = num_magic_squares_inside(grid);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }

    #[test]
    fn test_is_rectangle_overlap() {
        struct TestCase {
            rec1: Vec<i32>,
            rec2: Vec<i32>,
            expect: bool,
        }

        vec![
            TestCase {
                rec1: vec![0, 0, 2, 2],
                rec2: vec![1, 1, 3, 3],
                expect: true,
            },
            TestCase {
                rec1: vec![0, 0, 1, 1],
                rec2: vec![1, 0, 2, 1],
                expect: false,
            },
            TestCase {
                rec1: vec![0, 0, 1, 1],
                rec2: vec![2, 2, 3, 3],
                expect: false,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { rec1, rec2, expect } = testcase;
            let actual = is_rectangle_overlap(rec1, rec2);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_largest_overlap() {
        struct TestCase {
            img1: Vec<Vec<i32>>,
            img2: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            TestCase {
                img1: vec2![[1, 1, 0], [0, 1, 0], [0, 1, 0]],
                img2: vec2![[0, 0, 0], [0, 1, 1], [0, 0, 1]],
                expect: 3,
            },
            TestCase {
                img1: vec2![[1]],
                img2: vec2![[1]],
                expect: 1,
            },
            TestCase {
                img1: vec2![[0]],
                img2: vec2![[0]],
                expect: 0,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { img1, img2, expect } = testcase;
            let actual = largest_overlap(img1, img2);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_unique_letter_string() {
        struct TestCase {
            s: &'static str,
            expect: i32,
        }

        vec![
            TestCase {
                s: "ABC",
                expect: 10,
            },
            TestCase {
                s: "ABA",
                expect: 8,
            },
            TestCase {
                s: "LEETCODE",
                expect: 92,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { s, expect } = testcase;
            let actual = unique_letter_string(s.to_string());
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_consecutive_numbers_sum() {
        struct TestCase {
            n: i32,
            expect: i32,
        }

        vec![
            TestCase { n: 5, expect: 2 },
            TestCase { n: 9, expect: 3 },
            TestCase { n: 15, expect: 4 },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { n, expect } = testcase;
            let actual = consecutive_numbers_sum(n);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_number_of_steps() {
        struct TestCase {
            num: i32,
            expect: i32,
        }

        vec![
            TestCase { num: 14, expect: 6 },
            TestCase { num: 8, expect: 4 },
            TestCase {
                num: 123,
                expect: 12,
            },
            TestCase { num: 0, expect: 0 },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { num, expect } = testcase;
            let actual = number_of_steps(num);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_fizz_buzz() {
        struct TestCase {
            n: i32,
            expect: Vec<&'static str>,
        }

        vec![
            TestCase {
                n: 3,
                expect: vec!["1", "2", "Fizz"],
            },
            TestCase {
                n: 5,
                expect: vec!["1", "2", "Fizz", "4", "Buzz"],
            },
            TestCase {
                n: 15,
                expect: vec![
                    "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                    "13", "14", "FizzBuzz",
                ],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { n, expect } = testcase;
            let actual = fizz_buzz(n);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_construct_array() {
        struct TestCase {
            name: &'static str,
            n: i32,
            k: i32,
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic 1",
                n: 3,
                k: 1,
                expect: &[1, 2, 3],
            },
            TestCase {
                name: "basic 1",
                n: 3,
                k: 2,
                expect: &[1, 3, 2],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let acutal = construct_array(testcase.n, testcase.k);
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_num_special() {
        struct TestCase {
            name: &'static str,
            mat: &'static [&'static [i32]],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic 1",
                mat: &[&[1, 0, 0], &[0, 0, 1], &[1, 0, 0]],
                expect: 1,
            },
            TestCase {
                name: "basic 2",
                mat: &[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]],
                expect: 3,
            },
            TestCase {
                name: "basic 3",
                mat: &[&[0, 0, 0, 1], &[1, 0, 0, 0], &[0, 1, 1, 0], &[0, 0, 0, 0]],
                expect: 2,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let mat = testcase.mat.iter().map(|line| line.to_vec()).collect();
            let actual = num_special(mat);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_shuffle() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            n: i32,
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic 1",
                nums: &[2, 5, 1, 3, 4, 7],
                n: 3,
                expect: &[2, 3, 5, 4, 1, 7],
            },
            TestCase {
                name: "basic 2",
                nums: &[1, 2, 3, 4, 4, 3, 2, 1],
                n: 4,
                expect: &[1, 4, 2, 3, 3, 2, 4, 1],
            },
            TestCase {
                name: "basic 3",
                nums: &[1, 1, 2, 2],
                n: 2,
                expect: &[1, 2, 1, 2],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = shuffle(testcase.nums.to_vec(), testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_min_moves2() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            expect: i32,
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[1, 2, 3],
                expect: 2,
            },
            TestCase {
                name: "basic 2",
                nums: &[1, 1, 1, 100],
                expect: 99,
            },
            TestCase {
                name: "basic 3",
                nums: &[1, 10, 2, 9],
                expect: 16,
            },
            TestCase {
                name: "fix 1",
                nums: &[1],
                expect: 0,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = min_moves2(testcase.nums.to_vec());
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
