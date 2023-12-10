use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Card {
    _id: usize,
    win: Vec<usize>,
    have: Vec<usize>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 4;
    type Input1 = Vec<Card>;
    type Input2 = Vec<Card>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                let (card, rest) = x.split_once(':').unwrap();
                let (win, have) = rest.split_once('|').unwrap();
                let win = win
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let have = have
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                let c: Vec<_> = card.split_ascii_whitespace().collect();

                Card {
                    _id: c[1].parse().unwrap(),
                    win,
                    have,
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|x| {
                let s: HashSet<_> = x.win.iter().collect();
                let s2: HashSet<_> = x.have.iter().collect();
                let m = s.intersection(&s2).count();
                match m {
                    0 => 0,
                    c => 2usize.pow((c - 1) as u32),
                }
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let wins: Vec<_> = v
            .iter()
            .map(|x| {
                let s: HashSet<_> = x.win.iter().collect();
                let s2: HashSet<_> = x.have.iter().collect();
                let m = s.intersection(&s2).count();
                m
            })
            .collect();

        let mut total = 0;
        let mut current: HashMap<usize, usize> = (1..=v.len()).map(|v| (v, 1)).collect();
        while !current.is_empty() {
            let mut next = HashMap::default();
            for d in current.drain() {
                total += d.1;
                for c in ((d.0 + 1)..).take(wins[d.0 - 1]) {
                    if c - 1 < wins.len() {
                        *next.entry(c).or_insert(0) += d.1;
                    }
                }
            }
            current = next;
        }

        total
    }
}

crate::default_tests!(22193, 5625994);
crate::string_tests!(
    [(
        foo_sol1,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        13
    )],
    [(
        foo_sol2,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        30
    )]
);
