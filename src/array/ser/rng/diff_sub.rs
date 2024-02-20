//! # 差分数组
//!
//! 特点:
//! * 区间
//! * 与前缀比: 原数组变动
//!
//! 步骤:
//! 1. 差分: `diff[0] = origin[0]; diff[i] = origin[i] - origin[i-1]`
//! 2. 操作: `diff[i] += inc; diff[j] -= inc`
//! 3. 还原: `origin[0] = diff[0]; origin[i] = origin[i-1] + diff[i]`
//!
//! ## 题目
//! * 简单
//! * 中等
//!     * [370. 区间加法](get_modified_array)
//!     * [1109. 航班预订统计](corp_flight_bookings)
//!     * [1094. 拼车](car_pooling)

/// [370. 区间加法](https://leetcode-cn.com/problems/range-addition/)
///
pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![0; length as usize];
    let mut diff = vec![0; length as usize];

    for update in updates.iter() {
        let (start, end, inc) = (update[0] as usize, update[1] as usize, update[2]);
        diff[start] += inc;
        if end + 1 < diff.len() {
            diff[end + 1] -= inc;
        }
    }

    // restore
    ret[0] = diff[0];
    for i in 1..ret.len() {
        ret[i] = diff[i] + ret[i - 1];
    }
    ret
}

/// [1109. 航班预订统计](https://leetcode-cn.com/problems/corporate-flight-bookings/)
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut ret = vec![0; n as usize];
    let mut diff = vec![0; n as usize];

    for booking in bookings.iter() {
        let (start, end, inc) = (booking[0] as usize - 1, booking[1] as usize - 1, booking[2]);
        diff[start] += inc;
        if end + 1 < diff.len() {
            diff[end + 1] -= inc;
        }
    }

    // restore
    ret[0] = diff[0];
    for i in 1..ret.len() {
        ret[i] = diff[i] + ret[i - 1];
    }
    ret
}

/// [1094. 拼车](https://leetcode-cn.com/problems/car-pooling/)
pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let end = { trips.iter().map(|trip| trip[2]).max().unwrap() };

    let mut counter = vec![0; end as usize];
    let mut diff = vec![0; end as usize];

    for trip in trips.iter() {
        // [from, to) 到了就下车, 不包含在内
        let (cnt, from, to) = (trip[0], trip[1] as usize, trip[2] as usize - 1);

        diff[from] += cnt;
        if to + 1 < diff.len() {
            diff[to + 1] -= cnt;
        }
    }
    // restore
    counter[0] = diff[0];
    for i in 1..counter.len() {
        counter[i] = diff[i] + counter[i - 1];
    }

    counter.iter().all(|cnt| *cnt <= capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_pooling() {
        struct TestCase {
            name: &'static str,
            trips: &'static [&'static [i32]],
            capacity: i32,
            expect: bool,
        }

        [TestCase {
                name: "basic",
                trips: &[&[2, 1, 5], &[3, 3, 7]],
                capacity: 4,
                expect: false,
            },
            TestCase {
                name: "basic 2",
                trips: &[&[2, 1, 5], &[3, 3, 7]],
                capacity: 5,
                expect: true,
            },
            TestCase {
                name: "fix 1",
                trips: &[&[2, 1, 5], &[3, 5, 7]],
                capacity: 3,
                expect: true,
            },
            TestCase {
                name: "fix 2",
                trips: &[&[9, 0, 1], &[3, 3, 7]],
                capacity: 4,
                expect: false,
            }]
        .iter()
        .for_each(|testcase| {
            let trips = testcase.trips.iter().map(|x| x.to_vec()).collect();
            let actual = car_pooling(trips, testcase.capacity);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_corp_flight_bookings() {
        struct TestCase {
            name: &'static str,
            bookings: &'static [&'static [i32]],
            n: i32,
            expect: &'static [i32],
        }

        [TestCase {
                name: "basic",
                bookings: &[&[1, 2, 10], &[2, 3, 20], &[2, 5, 25]],
                n: 5,
                expect: &[10, 55, 45, 25, 25],
            },
            TestCase {
                name: "basic 2",
                bookings: &[&[1, 2, 10], &[2, 2, 15]],
                n: 2,
                expect: &[10, 25],
            }]
        .iter()
        .for_each(|testcase| {
            let bookings = testcase.bookings.iter().map(|x| x.to_vec()).collect();
            let actual = corp_flight_bookings(bookings, testcase.n);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }

    #[test]
    fn test_get_modified_array() {
        struct TestCase {
            name: &'static str,
            length: i32,
            updates: &'static [&'static [i32]],
            expect: &'static [i32],
        }

        [TestCase {
            name: "basic",
            length: 5,
            updates: &[&[1, 3, 2], &[2, 4, 3], &[0, 2, -2]],
            expect: &[-2, 0, 3, 5, 3],
        }]
        .iter()
        .for_each(|testcase| {
            let updates = testcase.updates.iter().map(|x| x.to_vec()).collect();
            let actual = get_modified_array(testcase.length, updates);
            assert_eq!(testcase.expect, actual, "{} failed", testcase.name);
        });
    }
}
