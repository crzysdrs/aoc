use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub enum Dir {
    L,
    R,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input1 = Vec<(Dir, i32)>;
    type Input2 = Vec<(Dir, i32)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let mut c = s.chars();
                let d = match c.next().unwrap() {
                    'R' => Dir::R,
                    'L' => Dir::L,
                    _ => panic!(),
                };
                let v = c.as_str().parse().unwrap();
                (d, v)
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(input: &Self::Input1) -> Self::Sol1 {
        let mut v = 50;
        input
            .iter()
            .map(|(d, c)| {
                match d {
                    Dir::R => {
                        v += c;
                    }
                    Dir::L => {
                        v -= c;
                    }
                }
                v %= 100;
                if v == 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
    fn p2(input: &Self::Input2) -> Self::Sol2 {
        let mut v = 50;
        input
            .iter()
            .map(|(d, c)| {
                let mut c = *c;
                let mut count = 0;
                while c > 0 {
                    match d {
                        Dir::R => {
                            v += 1;
                        }
                        Dir::L => {
                            v -= 1;
                        }
                    }
                    if v == -100 || v == 0 || v == 100 {
                        v = 0;
                        count += 1;
                    }
                    c -= 1;
                }

                count
            })
            .sum()
    }
}

crate::default_tests!(1120, 6554);
crate::string_tests!(
    [(
        foo_sol1,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
        3
    )],
    [(
        foo_sol2,
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
        6
    )]
);
