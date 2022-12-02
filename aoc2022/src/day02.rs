use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub enum WLD {
    Win,
    Lose,
    Draw,
}

impl WLD {
    fn score(&self) -> u32 {
        match self {
            WLD::Win => 6,
            WLD::Draw => 3,
            WLD::Lose => 0,
        }
    }
}
impl RPS {
    fn win(&self, other: &RPS) -> WLD {
        match (self, other) {
            (Self::Rock, Self::Scissors) => WLD::Win,
            (Self::Scissors, Self::Paper) => WLD::Win,
            (Self::Paper, Self::Rock) => WLD::Win,
            (a, b) if a == b => WLD::Draw,
            _ => WLD::Lose,
        }
    }
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input1 = Vec<(RPS, RPS)>;
    type Input2 = Vec<(RPS, WLD)>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let (a, b) = s.split_once(' ').unwrap();
                let rps = |s| match s {
                    'A' | 'X' => RPS::Rock,
                    'B' | 'Y' => RPS::Paper,
                    'C' | 'Z' => RPS::Scissors,
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
                    'A' => RPS::Rock,
                    'B' => RPS::Paper,
                    'C' => RPS::Scissors,
                    _ => panic!(),
                };
                let wld = |s| match s {
                    'X' => WLD::Lose,
                    'Y' => WLD::Draw,
                    'Z' => WLD::Win,
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
                    WLD::Win => match a {
                        RPS::Rock => RPS::Paper,
                        RPS::Paper => RPS::Scissors,
                        RPS::Scissors => RPS::Rock,
                    },
                    WLD::Draw => *a,
                    WLD::Lose => match a {
                        RPS::Rock => RPS::Scissors,
                        RPS::Paper => RPS::Rock,
                        RPS::Scissors => RPS::Paper,
                    },
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
