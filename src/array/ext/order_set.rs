//! 有序集合
//! 主要是 [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) 和 [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)的使用
//!
//!

pub mod quiz_855 {
    use std::collections::BTreeSet;
    pub struct ExamRoom {
        n: i32,
        seats: BTreeSet<i32>,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl ExamRoom {
        pub fn new(n: i32) -> Self {
            Self {
                n,
                seats: BTreeSet::new(),
            }
        }

        pub fn seat(&mut self) -> i32 {
            let mut pos = 0;
            if !self.is_empty() {
                let mut dist = self.first(); // 不能直接是0, 因为可能之前leave 0
                let mut prev = None::<i32>;
                for &s in self.seats.iter() {
                    if let Some(pp) = prev {
                        let d = (s - pp) / 2;
                        if d > dist {
                            dist = d;
                            pos = pp + d;
                        }
                    }
                    prev.replace(s);
                }

                if self.n - 1 - self.last() > dist {
                    pos = self.n - 1;
                }
            }

            self.seats.insert(pos);
            pos
        }

        fn is_empty(&self) -> bool {
            self.seats.is_empty()
        }

        fn first(&self) -> i32 {
            self.seats.iter().next().copied().unwrap()
        }
        fn last(&self) -> i32 {
            self.seats.iter().last().copied().unwrap()
        }

        pub fn leave(&mut self, p: i32) {
            self.seats.remove(&p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_855() {
        enum OP {
            Seat(i32),
            Leave(i32),
        }

        let mut room = quiz_855::ExamRoom::new(10);

        vec![
            OP::Seat(0),
            OP::Seat(9),
            OP::Seat(4),
            OP::Seat(2),
            OP::Leave(4),
            OP::Seat(5),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, op)| match op {
            OP::Seat(expect) => {
                assert_eq!(expect, room.seat(), "case {} failed", idx);
            }
            OP::Leave(seat) => {
                room.leave(seat);
            }
        });
    }
}
