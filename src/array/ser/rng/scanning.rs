//! 扫描线
//!
//! 题目
//! * [850. 矩形面积 II](rectangle_area)

/// [850. 矩形面积 II](https://leetcode.cn/problems/rectangle-area-ii/)
///
/// > <https://oi-wiki.org/geometry/scanning/>
///
pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Ordering;
    use std::collections::HashSet;

    const MOD: i64 = 1_000_000_007;

    struct Range<T>
    where
        T: Ord,
    {
        pub start: T,
        pub end: T,
    }

    let coord_x = {
        let mut tmp = HashSet::new();
        for rect in rectangles.iter() {
            tmp.insert(rect[0]);
            tmp.insert(rect[2]);
        }
        let mut tmp2 = tmp.into_iter().collect::<Vec<i32>>();
        tmp2.sort();
        tmp2
    };

    let mut ans = 0i64;
    for i in 1..coord_x.len() {
        let (a, b) = (
            coord_x.get(i - 1).copied().unwrap(),
            coord_x.get(i).copied().unwrap(),
        );
        let mut lines = Vec::new();
        for rect in rectangles.iter() {
            // O(n)
            if rect[0] <= a && b <= rect[2] {
                // x轴方向, 在[a, b] 之间有矩形
                // 记录y轴的有效覆盖
                lines.push(Range {
                    start: rect[1],
                    end: rect[3],
                });
            }
        }
        lines.sort_by(
            |l1, l2| match (l1.start.cmp(&l2.start), l1.end.cmp(&l2.end)) {
                (Ordering::Equal, x) => x,
                (x, _) => x,
            },
        );

        let mut tot = 0i64;
        let (mut l, mut r) = (-1, -1);
        // 累加Y轴方向, 区间的总和
        // 交叠的部分, 不重复计入
        for line in lines {
            if line.start > r {
                // l..r..start..end
                // 新的片段 start..end
                tot += (r - l) as i64; // 记录Y轴上的区间总和
                Range { start: l, end: r } = line;
            } else if line.end > r {
                // l..start..r..end
                // 延展 r 到 end
                r = line.end;
            }
        }
        // 把剩余的加上
        tot += (r - l) as i64;
        // 计算结果
        ans += tot * (b - a) as i64;
        ans %= MOD;
    }
    ans as i32
}

///
pub fn rectangle_area_2(rectangles: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    const MOD: i64 = 1_000_000_007;

    // 记录下所有的 Y 轴线, 用于扫描
    // 从下到大, 去重
    let coord_y = {
        let mut tmp = HashSet::new();
        for rect in rectangles.iter() {
            tmp.insert(rect[1]);
            tmp.insert(rect[3]);
        }
        let mut tmp2 = tmp.into_iter().collect::<Vec<i32>>();
        tmp2.sort();
        tmp2
    };

    // 是沿Y轴, 平行于X轴扫描
    // 沿X轴正向视角看, 接触到矩形左边所在X线, 记为进+1, 到达右边所在X线, 记为出-1
    // 并不是要求和矩形的边相交, 而是要求和对应的X线相交
    // 这样平行于X轴, 随意画一条线, 通过累加 +1, -1 就可以得到 ""
    let sweep = {
        let mut tmp = vec![];
        for (idx, rect) in rectangles.iter().enumerate() {
            tmp.push((rect[0], idx, 1));
            tmp.push((rect[2], idx, 0-1));
        }
        tmp.sort();
        tmp
    };

    let mut ans = 0i64;
    let mut cursor = 0usize;
    // 存的是 [coord_y[k], coord_y[k+1]] 这个范围内, 有多少个有效的矩阵
    let mut cnt_cross_y = vec![0; coord_y.len()]; 

    while cursor < sweep.len() {
        let mut j = cursor;
        while j + 1 < sweep.len() && sweep[cursor].0 == sweep[j + 1].0 {
            // x线相同的, 聚合到一起
            j += 1;
        }
        if j + 1 == sweep.len() {
            // 最后一个X线, 找不出有交集的矩形了
            break;
        }

        // 一次性处理一批横坐标相同的
        for k in cursor..=j {
            let (_, idx, diff) = sweep[k];
            // 逐个遍历范围内的矩形
            // 检查其Y轴方向是否和
            let (down, up) = (rectangles[idx][1], rectangles[idx][3]);
            for x in 0..coord_y.len()-1 { // 这里有-1
                if down <= coord_y[x] && coord_y[x + 1] <= up {
                    // 注意有等号
                    cnt_cross_y[x] += diff;
                }
            }
        }
        // x在[sweep[j].0, sweep[j + 1].0]范围内, Y轴的区间加和
        let mut cover = 0;
        for k in 0..coord_y.len()-1 { // 这里有-1
            if cnt_cross_y[k] > 0 {
                // cnt_cross_y[k] > 0, 代表这个范围(x的区间), 有需要计数的区域
                cover += (coord_y[k + 1] - coord_y[k]) as i64;
            }
        }
        ans += cover * (sweep[j + 1].0 - sweep[j].0) as i64;
        ans %= MOD;
        cursor = j + 1;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;

    #[test]
    fn test_rectangle_area() {
        struct Testcase {
            rectangles: Vec<Vec<i32>>,
            expect: i32,
        }

        vec![
            Testcase {
                rectangles: vec2![[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]],
                expect: 6,
            },
            Testcase {
                rectangles: vec2![[0, 0, 1000000000, 1000000000]],
                expect: 49,
            },
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, testcase)| {
            let Testcase { rectangles, expect } = testcase;
            let acutal = rectangle_area(rectangles);
            assert_eq!(expect, acutal, "case {} failed", idx);
        });
    }
}
