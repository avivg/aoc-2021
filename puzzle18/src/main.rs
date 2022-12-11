fn main() {
    for func in [day18::part1, day18::part2] {
        let start = std::time::Instant::now();
        let res = func(INPUT);
        let dur = start.elapsed().as_nanos();

        println!("{res} [{dur} ns]");
    }
}

mod day18 {
    use std::{fmt::Debug, ops::Add};

    pub fn part1(input: &str) -> u64 {
        todo!()
    }

    pub fn part2(input: &str) -> u64 {
        todo!()
    }

    trait Magnitude {
        fn magnitude(&self) -> u64;
    }

    impl Magnitude for u32 {
        fn magnitude(&self) -> u64 {
            *self as u64
        }
    }

    #[derive(Eq, Clone)]
    enum SnailNum {
        Val(u32),
        Pair(Box<SnailNum>, Box<SnailNum>),
    }

    impl Magnitude for SnailNum {
        fn magnitude(&self) -> u64 {
            match self {
                SnailNum::Val(v) => v.magnitude(),
                SnailNum::Pair(sn1, sn2) => 3 * sn1.magnitude() + 2 * sn2.magnitude(),
            }
        }
    }

    impl Debug for SnailNum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Val(arg0) => write!(f, "{}", *arg0),
                Self::Pair(arg0, arg1) => write!(f, "[{:?},{:?}]", *arg0.as_ref(), *arg1.as_ref()),
            }
        }
    }

    impl From<u32> for SnailNum {
        fn from(v: u32) -> Self {
            Self::Val(v)
        }
    }

    impl From<(SnailNum, SnailNum)> for SnailNum {
        fn from((l, r): (SnailNum, SnailNum)) -> Self {
            Self::Pair(Box::new(l), Box::new(r))
        }
    }

    impl PartialEq for SnailNum {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Val(l0), Self::Val(r0)) => l0 == r0,
                (Self::Pair(l0, l1), Self::Pair(r0, r1)) => l0 == r0 && l1 == r1,
                _ => false,
            }
        }
    }

    impl SnailNum {
        fn next(iter: &mut impl Iterator<Item = char>) -> Self {
            let c = iter.next().unwrap();
            match c {
                '0'..='9' => (c as u32 - '0' as u32).into(),
                _ => {
                    assert_eq!(c, '[');
                    let left = SnailNum::next(iter);
                    iter.next(); // skip ','
                    let right = SnailNum::next(iter);
                    iter.next(); // skip ']'
                    (left, right).into()
                }
            }
        }

        fn reduce(self) -> Self {
            self
        }

        fn split(self) -> SNSplit {
            SNSplit::split(self)
        }
    }

    impl Add for SnailNum {
        type Output = SnailNum;

        fn add(self, rhs: Self) -> Self::Output {
            SnailNum::from((self, rhs)).reduce()
        }
    }

    #[derive(Debug)]
    enum SNSplit {
        Yes(SnailNum),
        No(SnailNum),
    }

    impl SNSplit {
        fn split(sn: SnailNum) -> Self {
            use SNSplit::{No, Yes};
            match sn {
                SnailNum::Pair(l, r) => match SNSplit::split(l.as_ref().clone()) {
                    Yes(splitted_l) => Yes(SnailNum::Pair(Box::new(splitted_l), r)),
                    No(unchanged_l) => match SNSplit::split(r.as_ref().clone()) {
                        Yes(splitted_r) => Yes((unchanged_l, splitted_r).into()),
                        No(unchanged_r) => No((unchanged_l, unchanged_r).into()),
                    },
                },
                SnailNum::Val(v) => {
                    if v >= 10 {
                        Yes(((v / 2).into(), (v - (v / 2)).into()).into())
                    } else {
                        No(sn)
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::{SNSplit, SnailNum};
        use crate::day18::Magnitude;

        #[test]
        fn parse_and_magnitude() {
            let sn = SnailNum::next(
                &mut "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".chars(),
            );
            dbg!(&sn);
            assert_eq!(4140, sn.magnitude());
        }

        #[test]
        fn add_no_reduce() {
            let sn1 = SnailNum::next(&mut "[1,2]".chars());
            let sn2 = SnailNum::next(&mut "3".chars());
            let sum = sn1 + sn2;
            if let SnailNum::Pair(onetwo, three) = &sum {
                assert_eq!(three.magnitude(), 3);
                if let SnailNum::Pair(one, two) = onetwo.as_ref() {
                    assert_eq!(1, one.magnitude());
                    assert_eq!(2, two.magnitude());
                } else {
                    panic!("Expected pair");
                }
            } else {
                panic!("Expected pair");
            }
        }

        #[test]
        fn split_val() {
            let sn = SnailNum::from(10);
            if let SNSplit::Yes(SnailNum::Pair(l, r)) = dbg!(sn.split()) {
                assert_eq!(
                    (&SnailNum::from(5), &SnailNum::from(5)),
                    (l.as_ref(), r.as_ref())
                );
            } else {
                panic!();
            }
            let sn = SnailNum::from(11);
            if let SNSplit::Yes(SnailNum::Pair(l, r)) = dbg!(sn.split()) {
                assert_eq!(
                    (&SnailNum::from(5), &SnailNum::from(6)),
                    (l.as_ref(), r.as_ref())
                );
            } else {
                panic!();
            }
        }

        #[test]
        fn split_pair() {
            match SnailNum::from((13.into(), 5.into())).split() {
                SNSplit::Yes(SnailNum::Pair(l, five)) => {
                    assert_eq!(five.as_ref(), &SnailNum::Val(5));
                    match l.as_ref() {
                        SnailNum::Pair(six, seven) => {
                            assert_eq!(six.as_ref(), &SnailNum::Val(6));
                            assert_eq!(seven.as_ref(), &SnailNum::Val(7));
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
            match SnailNum::from((7.into(), 17.into())).split() {
                SNSplit::Yes(SnailNum::Pair(seven, r)) => {
                    assert_eq!(seven.as_ref(), &SnailNum::Val(7));
                    match r.as_ref() {
                        SnailNum::Pair(eight, nine) => {
                            assert_eq!(eight.as_ref(), &SnailNum::Val(8));
                            assert_eq!(nine.as_ref(), &SnailNum::Val(9));
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
    }
}

const INPUT: &str = include_str!("input.txt");
