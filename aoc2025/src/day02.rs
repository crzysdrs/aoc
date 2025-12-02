use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::ops::RangeInclusive;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input1 = Vec<RangeInclusive<usize>>;
    type Input2 = Vec<RangeInclusive<usize>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|l| {
                let (l, r) = l.split_once('-').unwrap();
                l.parse().unwrap()..=r.parse().unwrap()
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|range| {
                let mut count = 0;
                for v in range.clone().into_iter() {
                    let b10 = v.ilog10();

                    let split = b10 / 2 + 1;
                    let l = v / 10usize.pow(split);
                    let r = v % 10usize.pow(split);

                    //println!("{:?} {} {}", v, l, r);
                    if l == r && b10 % 2 != 0 {
                        count += v;
                    }
                }
                count
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut peices = vec![];
        v.iter()
            .map(|range| {
                let mut count = 0;
                for v in range.clone().into_iter() {
                    let b10 = v.ilog10() + 1;
                    'next: for split in (1..b10).filter(|i| b10 % i == 0) {
                        let mut num = v;
                        peices.clear();

                        for _ in 0..b10 / split {
                            let peice = num % 10usize.pow(split);
                            peices.push(peice);
                            num /= 10usize.pow(split);
                        }
                        //println!("b10: {} split: {} {} {:?}", b10, split, v, peices);
                        assert_eq!(peices.len(), (b10 / split) as usize);
                        //println!("{:?} {} {}", v, l, r);
                        if peices.iter().all(|x| x == peices.first().unwrap()) {
                            //println!("HIT");
                            count += v;
                            break 'next;
                        }
                    }
                }
                count
            })
            .sum()
    }
}

crate::default_tests!(18595663903, 19058204438);
crate::string_tests!(
    [(
        foo_sol1,
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        1227775554
    )],
    [
        (foo_sol2, "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 4174379265)
    ]
);
