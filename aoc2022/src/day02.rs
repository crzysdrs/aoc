use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

pub enum Wld {
    Win,
    Lose,
    Draw,
}

impl Wld {
    fn score(&self) -> u32 {
        match self {
            Wld::Win => 6,
            Wld::Draw => 3,
            Wld::Lose => 0,
        }
    }
}
impl Rps {
    fn win(&self, other: &Rps) -> Wld {
        match (self, other) {
            (a, b) if a.loses() == *b => Wld::Win,
            (a, b) if a == b => Wld::Draw,
            _ => Wld::Lose,
        }
    }
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn beats(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }
    fn loses(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input1 = Vec<(Rps, Rps)>;
    type Input2 = Vec<(Rps, Wld)>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let (a, b) = s.split_once(' ').unwrap();
                let rps = |s| match s {
                    'A' | 'X' => Rps::Rock,
                    'B' | 'Y' => Rps::Paper,
                    'C' | 'Z' => Rps::Scissors,
                    _ => panic!(),
                };
                (
                    rps(a.chars().next().unwrap()),
                    rps(b.chars().next().unwrap()),
                )
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        s.lines()
            .map(|s| {
                let (a, b) = s.split_once(' ').unwrap();
                let rps = |s| match s {
                    'A' => Rps::Rock,
                    'B' => Rps::Paper,
                    'C' => Rps::Scissors,
                    _ => panic!(),
                };
                let wld = |s| match s {
                    'X' => Wld::Lose,
                    'Y' => Wld::Draw,
                    'Z' => Wld::Win,
                    _ => panic!(),
                };
                (
                    rps(a.chars().next().unwrap()),
                    wld(b.chars().next().unwrap()),
                )
            })
            .collect()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter().map(|(a, b)| b.win(a).score() + b.score()).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .map(|(a, b)| {
                let rsp = match b {
                    Wld::Win => a.beats(),
                    Wld::Draw => *a,
                    Wld::Lose => a.loses(),
                };
                b.score() + rsp.score()
            })
            .sum()
    }
}

crate::default_tests!(11873, 12014);
crate::path_tests!(
    [(sol1, "test/day02.txt", 15)],
    [(sol2, "test/day02.txt", 12)]
);
