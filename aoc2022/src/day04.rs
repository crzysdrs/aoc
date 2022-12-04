use crate::Day;
use core::ops::RangeInclusive;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 4;
    type Input1 = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;
    type Input2 = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let v = l
                    .split(&['-', ','])
                    .map(|i| i.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                (v[0]..=v[1], v[2]..=v[3])
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .filter(|(x, y)| {
                (y.contains(x.start()) && y.contains(x.end()))
                    || (x.contains(y.start()) && x.contains(y.end()))
            })
            .count()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .filter(|(x, y)| {
                y.contains(x.start())
                    || y.contains(x.end())
                    || x.contains(y.start())
                    || x.contains(y.end())
            })
            .count()
    }
}

crate::default_tests!(511, 821);
crate::path_tests!([(sol1, "test/day04.txt", 2)], [(sol2, "test/day04.txt", 4)]);
