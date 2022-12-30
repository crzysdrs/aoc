use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Snafu(String);

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> i64 {
        snafu
            .0
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                5i64.pow(i as u32)
                    * match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '=' => -2,
                        '-' => -1,
                        _ => panic!(),
                    }
            })
            .sum()
    }
}

impl From<i64> for Snafu {
    fn from(mut i: i64) -> Snafu {
        if i == 0 {
            return Snafu("0".to_string());
        }
        let mut chars = vec![];
        while i > 0 {
            let b5 = i % 5;
            let new_c = match b5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    i += 2;
                    '='
                }
                4 => {
                    i += 1;
                    '-'
                }
                _ => panic!(),
            };
            chars.push(new_c);

            i /= 5;
        }
        chars.reverse();
        Snafu(chars.iter().collect())
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 25;
    type Input1 = Vec<Snafu>;
    type Input2 = ();
    type Sol1 = String;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines().map(|s| Snafu(s.to_string())).collect()
    }
    fn process_input2(_s: &str) -> Self::Input2 {}
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let snafu = Snafu::from(v.iter().map(|s| i64::from(s.clone())).sum::<i64>());
        snafu.0
    }
    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        0
    }
}

#[cfg(test)]
mod locals {
    use super::*;
    #[test]
    fn test() {
        let tests = "        1              1\n\
                     2              2\n\
                     3             1=\n\
                     4             1-\n\
                     5             10\n\
                     6             11\n\
                     7             12\n\
                     8             2=\n\
                     9             2-\n\
                    10             20\n\
                    15            1=0\n\
                    20            1-0\n\
                  2022         1=11-2\n\
                 12345        1-0---0\n\
             314159265  1121-1110-1=0";

        tests.lines().for_each(|l| {
            let v: Vec<_> = l.split_whitespace().collect();
            assert_eq!(i64::from(Snafu(v[1].to_string())), v[0].parse().unwrap());
            assert_eq!(
                Snafu(v[1].to_string()),
                Snafu::from(v[0].parse::<i64>().unwrap())
            );
        });
    }
}
crate::default_tests!("2=000=22-0-102=-1001", 0);
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
