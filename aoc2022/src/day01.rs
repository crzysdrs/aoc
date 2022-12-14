use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input1 = Vec<Vec<u32>>;
    type Input2 = Vec<Vec<u32>>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut elves = vec![];
        let mut lines = s.lines();

        loop {
            let elf: Vec<_> = lines
                .by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();
            if elf.is_empty() {
                break;
            }
            elves.push(elf)
        }
        elves
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter().map(|x| x.iter().sum()).max().unwrap()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        v.sort_by_key(|x| x.iter().sum::<u32>());
        v.reverse();
        v.iter().map(|x| x.iter().sum::<u32>()).take(3).sum()
    }
}

crate::default_tests!(71506, 209603);
crate::path_tests!(
    [(sol1, "test/day01.txt", 24000)],
    [(sol2, "test/day01.txt", 45000)]
);
