//! # 二维数组旋转
//!
//! ## 题目
//! * 简单
//!     * [剑指 Offer 58 - II. 左旋转字符串](reverse_left_words)
//! * 中等
//!     * [48. 旋转图像](rotate)
//!     * [54. 螺旋矩阵](spiral_order)
//!     * [59. 螺旋矩阵 II](generate_matrix)
//!     * [498. 对角线遍历](find_diagonal_order)
//! * 困难
//!

/// [剑指 Offer 58 - II. 左旋转字符串](https://leetcode-cn.com/problems/zuo-xuan-zhuan-zi-fu-chuan-lcof/)
///
/// 方法1: slice 有实现旋转方法 [core::slice::rotate::ptr_rotate] 直接调用即可
/// ```
/// pub fn reverse_left_words(s: String, n: i32) -> String {
///     let mut s = s;
///     let x = unsafe { s.as_bytes_mut() };
///     x.rotate_left(n as usize);
///     s
/// }
/// ```
/// 方法2:
/// 主要思路步骤就是, 先将 `[0..left]` 和 `[left+1..]` 两部分各自旋转, 再整体旋转.
///
pub fn reverse_left_words(s: String, n: i32) -> String {
    fn swap_range(x: &mut [u8], left: usize, right: usize) {
        let (mut left, mut right) = (left, right);
        while left < right {
            x.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    let mut s = s;
    let x = unsafe { s.as_bytes_mut() };

    // 前n个
    swap_range(x, 0, n as usize - 1);
    // n个之后的
    swap_range(x, n as usize, x.len() - 1);
    // 整体
    swap_range(x, 0, x.len() - 1);

    s
}

/// [48. 旋转图像](https://leetcode-cn.com/problems/rotate-image/)
///
/// 方法:
/// 1. 先沿主对角线镜像对称二维矩阵
/// 2. 再逐行反转二维矩阵
///
/// 规律:
/// * 矩阵顺时针旋转 = 矩阵沿主对角线对称变换 + 矩阵沿垂直对称线变换
/// * 矩阵逆时针旋转 = 矩阵沿次对角线对称变换 + 矩阵沿垂直对称线变换
#[allow(clippy::ptr_arg)]
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let length = matrix.len();
    for row in 0..length {
        for col in row..length {
            let tmp = matrix[row][col];
            matrix[row][col] = matrix[col][row];
            matrix[col][row] = tmp;
        }
    }

    matrix.iter_mut().for_each(|row| {
        let (mut left, mut right) = (0, row.len() - 1);
        while left < right {
            row.swap(left, right);
            left += 1;
            right -= 1;
        }
    })
}

/// [54. 螺旋矩阵](https://leetcode-cn.com/problems/spiral-matrix/)
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (matrix.len(), matrix.first().unwrap().len());

    let (mut upper_bound, mut lower_bound) = (0isize, m as isize - 1);
    let (mut left_bound, mut right_bound) = (0isize, n as isize - 1);

    let mut result = Vec::with_capacity(m * n);
    while result.len() < m * n {
        if upper_bound <= lower_bound {
            for col in left_bound..=right_bound {
                result.push(matrix[upper_bound as usize][col as usize]);
            }
            upper_bound += 1;
        }
        if left_bound <= right_bound {
            for row in upper_bound..=lower_bound {
                result.push(matrix[row as usize][right_bound as usize]);
            }
            right_bound -= 1;
        }
        if upper_bound <= lower_bound {
            for col in (left_bound..=right_bound).rev() {
                result.push(matrix[lower_bound as usize][col as usize]);
            }
            lower_bound -= 1;
        }
        if left_bound <= right_bound {
            for row in (upper_bound..=lower_bound).rev() {
                result.push(matrix[row as usize][left_bound as usize]);
            }
            left_bound += 1;
        }
    }
    result
}

/// [59. 螺旋矩阵 II](https://leetcode-cn.com/problems/spiral-matrix-ii/)
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut nums = 1..n * n + 1;
    let (mut upper_bound, mut lowwer_bound) = (1, n as usize);
    let (mut left_bound, mut right_bound) = (1, n as usize);

    let mut result = vec![vec![0; n as usize]; n as usize];

    while !nums.is_empty() {
        if upper_bound <= lowwer_bound {
            for col in left_bound..=right_bound {
                result[upper_bound - 1][col - 1] = nums.next().unwrap();
            }
            upper_bound += 1;
        }
        if left_bound <= right_bound {
            for row in upper_bound..=lowwer_bound {
                result[row - 1][right_bound - 1] = nums.next().unwrap();
            }
            right_bound -= 1;
        }
        if upper_bound <= lowwer_bound {
            for col in (left_bound..=right_bound).rev() {
                result[lowwer_bound - 1][col - 1] = nums.next().unwrap();
            }
            lowwer_bound -= 1;
        }
        if left_bound <= right_bound {
            for row in (upper_bound..=lowwer_bound).rev() {
                result[row - 1][left_bound - 1] = nums.next().unwrap();
            }
            left_bound += 1;
        }
    }
    result
}

/// [498. 对角线遍历](https://leetcode.cn/problems/diagonal-traverse/)
pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (mat.len(), mat.first().unwrap().len());

    let (mut x, mut y) = (0, 0);
    let mut up = true;
    let mut result = Vec::with_capacity(m * n);
    while result.len() < m * n {
        result.push(mat.get(x).unwrap().get(y).copied().unwrap());
        // 矩阵不保证是方阵
        // 向上, 可能到达上边, 也可能是右边
        // 向下, 可能到达下边, 也可能是左边
        if up {
            // 原本向上
            if x == 0 {
                // 触到上边
                if y + 1 < n {
                    // 切换后还在上边
                    y += 1;
                } else {
                    x += 1;
                }
                up = false;
            } else if y == n - 1 {
                // 触到右边
                x += 1;
                up = false;
            } else {
                x -= 1;
                y += 1;
            }
        } else {
            // 原本向下
            if y == 0 {
                // 触到左边
                if x + 1 < m {
                    // 切换后还在左边
                    x += 1;
                } else {
                    y += 1;
                }
                up = true;
            } else if x == m - 1 {
                // 触到下边
                y += 1;
                up = true;
            } else {
                x += 1;
                y -= 1;
            }
        }
    }

    result
}

/// [832. 翻转图像](https://leetcode.cn/problems/flipping-an-image/)
#[rustfmt::skip]
pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut image = image;
    image.iter_mut().for_each(|line| {
        let (mut l, mut r) = (1, line.len());
        while l <= r {
            let (left, right) = (line[l - 1], line[r - 1]);
            if left == 0 { line[r-1] = 1; } else { line[r-1] = 0; }
            if right == 0 {line[l-1] = 1; } else { line[l-1] = 0; }
            l += 1; r -= 1;
        }
    });
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_flip_and_invert_image() {
        struct TestCase {
            image: Vec<Vec<i32>>,
            expect: Vec<Vec<i32>>,
        }

        vec![
            TestCase {
                image: vec2![[1, 1, 0], [1, 0, 1], [0, 0, 0]],
                expect: vec2![[1, 0, 0], [0, 1, 0], [1, 1, 1]],
            },
            TestCase {
                image: vec2![[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 1], [1, 0, 1, 0]],
                expect: vec2![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]],
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let TestCase { image, expect } = testcase;
            let actual = flip_and_invert_image(image);
            assert_eq!(expect, actual, "case {} failed", idx);
        });
    }

    #[test]
    fn test_find_diagonal_order() {
        struct TestCase {
            name: &'static str,
            mat: &'static [&'static [i32]],
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic 1",
                mat: &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
                expect: &[1, 2, 4, 7, 5, 3, 6, 8, 9],
            },
            TestCase {
                name: "basic 2",
                mat: &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
                expect: &[1, 2, 4, 7, 5, 3, 6, 8, 9],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let mat = testcase.mat.iter().map(|line| line.to_vec()).collect();
            let actual = find_diagonal_order(mat);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_generate_matrix() {
        struct TestCase {
            name: &'static str,
            n: i32,
            expect: &'static [&'static [i32]],
        }

        vec![
            TestCase {
                name: "basic",
                n: 3,
                expect: &[&[1, 2, 3], &[8, 9, 4], &[7, 6, 5]],
            },
            TestCase {
                name: "basic 2",
                n: 1,
                expect: &[&[1]],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = generate_matrix(testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_spiral_order() {
        struct TestCase {
            name: &'static str,
            matrix: &'static [&'static [i32]],
            expect: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic",
                matrix: &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
                expect: &[1, 2, 3, 6, 9, 8, 7, 4, 5],
            },
            TestCase {
                name: "basic 2",
                matrix: &[&[1, 2, 3, 4], &[5, 6, 7, 8], &[9, 10, 11, 12]],
                expect: &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            },
            TestCase {
                name: "fix 1",
                matrix: &[&[3], &[2]],
                expect: &[3, 2],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let matrix = testcase.matrix.iter().map(|x| x.to_vec()).collect();
            let actual = spiral_order(matrix);
            assert_eq!(testcase.expect, &actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_rotate() {
        struct TestCase {
            name: &'static str,
            matrix: &'static [&'static [i32]],
            expect: &'static [&'static [i32]],
        }

        vec![TestCase {
            name: "basic",
            matrix: &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
            expect: &[&[7, 4, 1], &[8, 5, 2], &[9, 6, 3]],
        }]
        .iter()
        .for_each(|testcase| {
            let mut matrix = testcase.matrix.iter().map(|x| x.to_vec()).collect();
            rotate(&mut matrix);
            assert_eq!(testcase.expect, matrix, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_reverse_left_words() {
        struct TestCase {
            name: &'static str,
            s: &'static str,
            n: i32,
            expect: &'static str,
        }

        vec![
            TestCase {
                name: "basic",
                s: "abcdefg",
                n: 2,
                expect: "cdefgab",
            },
            TestCase {
                name: "basic 2",
                s: "lrloseumgh",
                n: 6,
                expect: "umghlrlose",
            },
        ]
        .iter()
        .for_each(|testcase| {
            let s = testcase.s.to_string();
            let actual = reverse_left_words(s, testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
