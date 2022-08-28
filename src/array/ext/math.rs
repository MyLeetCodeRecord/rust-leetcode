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

#[cfg(test)]
mod tests {
    use super::*;

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