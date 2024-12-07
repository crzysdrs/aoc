use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Test {
    test: usize,
    values: Vec<usize>,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input1 = Vec<Test>;
    type Input2 = Vec<Test>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let (l, r) = s.split_once(':').unwrap();
                let test = l.parse().unwrap();
                let values = r
                    .trim()
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();
                Test { test, values }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .filter(|v| {
                for x in 0..(1 << (v.values.len() - 1)) {
                    let result = v.values.iter().skip(1).enumerate().fold(
                        v.values[0],
                        |mut state, (idx, v)| {
                            if x & (1 << idx) == 0 {
                                state *= v;
                            } else {
                                state += v;
                            }
                            state
                        },
                    );
                    if result == v.test {
                        return true;
                    }
                }
                false
            })
            .map(|v| v.test)
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .filter(|v| {
                for x in 0..3usize.pow((v.values.len() - 1) as u32) {
                    let result = v.values.iter().skip(1).enumerate().fold(
                        v.values[0],
                        |mut state, (idx, v)| {
                            let op = (x / 3usize.pow(idx as u32)) % 3;
                            match op {
                                0 => state *= v,
                                1 => state += v,
                                2 => {
                                    state = state * 10usize.pow(v.ilog10() + 1) + v;
                                }
                                _ => panic!(),
                            }
                            state
                        },
                    );
                    if result == v.test {
                        return true;
                    }
                }
                false
            })
            .map(|v| v.test)
            .sum()
    }
}

crate::default_tests!(21572148763543, 581941094529163);
crate::string_tests!(
    [(
        foo_sol1,
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        3749
    )],
    [(
        foo_sol2,
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        11387
    )]
);
