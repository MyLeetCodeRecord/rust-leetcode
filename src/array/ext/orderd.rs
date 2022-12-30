//!
//!
//!

#[allow(dead_code)]
pub mod quiz_855 {
    use std::cell::RefCell;
    use std::collections::BTreeSet;

    struct ExamRoom {
        max: i32,
        seats: RefCell<BTreeSet<i32>>,
    }

    impl ExamRoom {
        fn new(n: i32) -> Self {
            Self {
                max: n,
                seats: RefCell::new(BTreeSet::new()),
            }
        }

        fn seat(&self) -> i32 {
            let len = self.seats.borrow().len();
            if len == 0 {
                self.seats.borrow_mut().insert(0);
                return 0;
            }

            let (mut s, mut dis) = (0, 0);
            {
                let b = self.seats.borrow();
                let (contain_0, contain_max) = (b.contains(&0), b.contains(&(self.max - 1)));
                let s0 = b.iter();
                let s1 = {
                    let mut tmp = b.iter();
                    let x = tmp.next().copied().unwrap();
                    if !contain_0 {
                        s = 0;
                        dis = x;
                    }
                    tmp
                };

                for (&start, &end) in s0.zip(s1) {
                    let d = (end - start) / 2;
                    if d > dis {
                        dis = d;
                        s = start + d;
                    }
                }
                if !contain_max {
                    // leetcode rust版本太低, 不支持last
                    // let last = b.iter().rev().next().copied().unwrap();
                    let last = b.last().copied().unwrap();
                    if self.max - 1 - last > dis {
                        s = self.max - 1;
                        //dis = self.max - last;
                    }
                }
            }

            self.seats.borrow_mut().insert(s);
            return s;
        }

        fn leave(&self, p: i32) {
            self.seats.borrow_mut().remove(&p);
        }
    }

    #[test]
    fn test_examroom_basic() {
        let er = ExamRoom::new(10);
        assert_eq!(0, er.seat());
        assert_eq!(9, er.seat());
        assert_eq!(4, er.seat());
        assert_eq!(2, er.seat());
        er.leave(4);
        assert_eq!(5, er.seat());
    }

    #[test]
    fn test_examroom_1() {
        let er = ExamRoom::new(10);
        assert_eq!(0, er.seat());
        assert_eq!(9, er.seat());
        assert_eq!(4, er.seat());
        er.leave(0);
        er.leave(4);
        assert_eq!(0, er.seat());
        assert_eq!(4, er.seat());
        assert_eq!(2, er.seat());
        assert_eq!(6, er.seat());
        assert_eq!(1, er.seat());
        assert_eq!(3, er.seat());
        assert_eq!(5, er.seat());
        assert_eq!(7, er.seat());
        assert_eq!(8, er.seat());
    }

    #[test]
    fn test_examroom_2() {
        let er = ExamRoom::new(4);
        assert_eq!(0, er.seat());
        assert_eq!(3, er.seat());
        assert_eq!(1, er.seat());
        assert_eq!(2, er.seat());
        er.leave(1);
        er.leave(3);
        assert_eq!(1, er.seat());
    }
}
