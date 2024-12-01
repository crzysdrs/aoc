use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input1 = Vec<(i32, i32)>;
    type Input2 = Vec<(i32, i32)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let mut i = s.split_whitespace().map(|v| v.parse().unwrap());
                (i.next().unwrap(), i.next().unwrap())
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let (mut l, mut r): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        l.sort();
        r.sort();
        let sum = l
            .iter()
            .zip(r.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>();
        usize::try_from(sum).unwrap()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let (mut l, mut r): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        let mut s = 0;
        for x in l {
            s += usize::try_from(x).unwrap() * r.iter().filter(|r| **r == x).count();
        }
        s
    }
}

//crate::default_tests!((), ());
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
