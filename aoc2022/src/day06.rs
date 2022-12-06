use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = Vec<char>;
    type Input2 = Vec<char>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.chars().collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let len = 4;
        v.windows(len)
            .enumerate()
            .filter(|(_, w)| w.iter().collect::<HashSet<_>>().iter().count() == w.len())
            .next()
            .unwrap()
            .0
            + len
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let len = 14;
        v.windows(len)
            .enumerate()
            .filter(|(_, w)| w.iter().collect::<HashSet<_>>().iter().count() == w.len())
            .next()
            .unwrap()
            .0
            + len
    }
}

crate::default_tests!(1582, 3588);
crate::string_tests!([(t1, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)], []);
