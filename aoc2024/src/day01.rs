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
        l.iter()
            .zip(r.iter())
            .map(|(l, r)| usize::try_from(l.abs_diff(*r)).unwrap())
            .sum::<usize>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let (l, r): (Vec<_>, Vec<_>) = v.iter().cloned().unzip();
        l.iter()
            .map(|l| usize::try_from(*l).unwrap() * r.iter().filter(|r| **r == *l).count())
            .sum()
    }
}

crate::default_tests!(3714264, 18805872);
crate::string_tests!(
    [(
        foo_sol1,
        "3   4
4   3
2   5
1   3
3   9
3   3",
        11
    )],
    [(
        foo_sol2,
        "3   4
4   3
2   5
1   3
3   9
3   3",
        31
    )]
);
