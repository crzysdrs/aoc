use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone)]
pub struct RuckSack {
    l: HashSet<char>,
    r: HashSet<char>,
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 3;
    type Input1 = Vec<RuckSack>;
    type Input2 = Vec<RuckSack>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                let (l, r) = x.split_at(x.len() / 2);
                let l = l.chars().collect();
                let r = r.chars().collect();
                RuckSack { l, r }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|r| r.l.intersection(&r.r).copied().map(priority).sum::<u32>())
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.chunks_exact(3)
            .map(|x| {
                let badge = x.iter().map(|x| x.l.union(&x.r).copied().collect()).fold(
                    ('a'..='z').chain('A'..='Z').collect::<HashSet<_>>(),
                    |mut state, x| {
                        state = state.intersection(&x).copied().collect();
                        state
                    },
                );
                badge.iter().copied().map(priority).sum::<u32>()
            })
            .sum()
    }
}

crate::default_tests!(8349, 2681);
crate::path_tests!(
    [(sol1, "test/day03.txt", 157)],
    [(sol2, "test/day03.txt", 70)]
);
