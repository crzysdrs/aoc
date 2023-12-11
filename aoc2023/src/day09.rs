use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9;
    type Input1 = Vec<Vec<i32>>;
    type Input2 = Vec<Vec<i32>>;
    type Sol1 = i32;
    type Sol2 = i32;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|v| {
                let mut items = vec![];
                let mut cur = v.clone();
                while !cur.iter().all(|v| *v == 0) {
                    let tmp = cur.windows(2).map(|xs| xs[1] - xs[0]).collect();
                    items.push(cur);
                    cur = tmp;
                }
                println!("{:?}", items);
                items.iter().map(|v| v.last().unwrap()).sum::<i32>()
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .map(|v| {
                let mut items = vec![];
                let mut cur = v.clone();
                while !cur.iter().all(|v| *v == 0) {
                    let tmp = cur.windows(2).map(|xs| xs[1] - xs[0]).collect();
                    items.push(cur);
                    cur = tmp;
                }
                items
                    .iter()
                    .rev()
                    .map(|v| v.first().unwrap())
                    .fold(0, |x, y| y - x)
            })
            .sum()
    }
}

crate::default_tests!(1696140818, 1152);
crate::string_tests!(
    [(
        foo_sol1,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        114
    )],
    [(
        foo_sol2,
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        2
    )]
);
