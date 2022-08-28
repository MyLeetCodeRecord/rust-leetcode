/// [912. 排序数组](https://leetcode-cn.com/problems/sort-an-array/)
/// **快排** 主要思路是 分治, 分治可以用递归实现.
/// 步骤: 1. 确认 *轴*; 2. 根据 *轴*, 将数组切分; 3. 根据轴的位置, 递归处理每个新数组
/// 步骤一可变性不强, 可以选开头, 结尾, 中间, 随机, 方式很多不过没啥难度和技巧; 步骤三同理
/// 步骤2, 将小于x的放在x左边, 将大于x的放x右边.
///
/// 方式1: 使用额外数组, 挑数, 挑出来, 然后拼接在一起
/// 方式2: 快慢指针
/// ```rust
/// fn partition(nums: &mut [i32], left: usize, right: usize, pivot: usize) -> usize {
///     nums.swap(pivot, right);
///     let mut insert_idx = left;
///     for curr in left..right {
///         if nums[curr] <= nums[right] {
///             nums.swap(insert_idx, curr);
///             insert_idx += 1;
///         }
///     }
///     nums.swap(insert_idx, right);
///     insert_idx
/// }
/// ```
/// 方式3: 左右指针,
/// 但是rust的强类型约束, 加减导致边界会出现溢出, 不像其他语言是用`int`做索引
/// ```rust
/// fn partition(nums: &mut [i32], left: usize, right: usize, pivot: usize) -> usize {
///     let(mut l, mut r) = (left, right);
///     loop{
///         while l <= r && nums[l] <= nums[pivot]{
///             l += 1;
///         }
///         while l <= r && nums[r] >= nums[pivot]{
///             r -= 1;
///         }
///         if l >= r{
///             break;
///         }
///         nums.swap(l, r);
///     }
///     nums.swap(r, pivot);
///     return r;
/// }
/// ```
/// 不过思路和方式2的快慢指针是一样的, 只是用加了预处理, 将左右边界上明显不符合交换的元素, 做了跳过
///
pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    fn partition(nums: &mut [i32], left: usize, right: usize, pivot: usize) -> usize {
        nums.swap(pivot, right);
        let mut insert_idx = left;
        for curr in left..right {
            if nums[curr] <= nums[right] {
                nums.swap(insert_idx, curr);
                insert_idx += 1;
            }
        }
        nums.swap(insert_idx, right);
        insert_idx
    }

    fn random(from: usize, to: usize) -> usize {
        (from + to) / 2
    }

    fn sort_array_range(nums: &mut [i32], from: usize, to: usize) {
        if from >= to {
            return;
        }

        let mut pivot = random(from, to);
        pivot = partition(nums, from, to, pivot);
        if pivot > 0 {
            sort_array_range(nums, from, pivot - 1);
        }
        if pivot < nums.len() {
            sort_array_range(nums, pivot + 1, to)
        }
    }

    let mut nums = nums;
    let length = nums.len();
    sort_array_range(&mut nums, 0, length - 1);
    nums
}

/// [215. 数组中的第K个最大元素](https://leetcode-cn.com/problems/kth-largest-element-in-an-array/)
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn partition(nums: &mut [i32], left: usize, right: usize, pivot: usize) -> usize {
        nums.swap(pivot, right);
        let mut insert_idx = left;
        for curr in left..right {
            if nums[curr] >= nums[right] {
                // 这里是 大于等于
                nums.swap(insert_idx, curr);
                insert_idx += 1;
            }
        }
        nums.swap(insert_idx, right);

        insert_idx
    }
    fn random(from: usize, to: usize) -> usize {
        (from + to) / 2
    }

    fn sort_array_range(nums: &mut [i32], from: usize, to: usize, k: usize) {
        if from >= to {
            return;
        }

        let mut pivot = random(from, to);
        pivot = partition(nums, from, to, pivot);
        if pivot == k {
            // 提前终止
            return;
        }
        if pivot > 0 {
            sort_array_range(nums, from, pivot - 1, k);
        }
        if pivot < nums.len() {
            sort_array_range(nums, pivot + 1, to, k)
        }
    }
    let mut nums = nums;
    let length = nums.len();
    sort_array_range(&mut nums, 0, length - 1, k as usize - 1);
    return nums.get(k as usize - 1).copied().unwrap_or(0);
}

/// [56. 合并区间](https://leetcode.cn/problems/merge-intervals/)
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a.first().unwrap().cmp(b.first().unwrap()));

    let mut ret = vec![];

    let mut curr = 0;
    for i in 0..intervals.len() {
        if intervals[i][0] <= intervals[curr][1] {
            intervals[curr][1] = std::cmp::max(intervals[curr][1], intervals[i][1]);
        } else {
            ret.push(intervals[curr].clone());
            curr = i;
        }
    }
    ret.push(intervals[curr].clone());

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        struct TestCase {
            name: &'static str,
            intervals: &'static [&'static [i32]],
            expect: &'static [&'static [i32]],
        }

        vec![
            TestCase {
                name: "basic",
                intervals: &[&[1, 3], &[2, 6], &[8, 10], &[15, 18]],
                expect: &[&[1, 6], &[8, 10], &[15, 18]],
            },
            TestCase {
                name: "basic 2",
                intervals: &[&[1, 4], &[4, 5]],
                expect: &[&[1, 5]],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let intervals = testcase.intervals.iter().map(|rng| rng.to_vec()).collect();
            let acutal = merge(intervals);
            assert_eq!(testcase.expect, acutal, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_find_kth_largest() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
            k: i32,
            expect: i32,
        }
        vec![
            TestCase {
                name: "basic",
                nums: &[3, 2, 1, 5, 6, 4],
                k: 2,
                expect: 5,
            },
            TestCase {
                name: "basic 2",
                nums: &[3, 2, 3, 1, 2, 4, 5, 5, 6],
                k: 4,
                expect: 4,
            },
        ]
        .iter()
        .for_each(|testcase| {
            let actual = find_kth_largest(testcase.nums.to_vec(), testcase.k);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        })
    }

    #[test]
    fn test_sort_array() {
        struct TestCase {
            name: &'static str,
            nums: &'static [i32],
        }

        vec![
            TestCase {
                name: "basic",
                nums: &[5, 2, 3, 1],
            },
            TestCase {
                name: "basic 2",
                nums: &[5, 1, 1, 2, 0, 0],
            },
            TestCase {
                name: "fix 1",
                nums: &[-1, 2, -8, -10],
            },
        ]
        .iter()
        .for_each(|testcase| {
            let mut nums = testcase.nums.to_vec();
            let actual = sort_array(nums.clone());
            nums.sort();
            let expect = nums;
            assert_eq!(expect, actual, "{} failed", testcase.name);
        });
    }
}
