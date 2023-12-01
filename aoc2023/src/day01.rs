use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input1 = Vec<(u32, u32)>;
    type Input2 = Vec<(u32, u32)>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                (
                    x.chars()
                        .filter(|x| x.is_ascii_digit())
                        .nth(0)
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                    x.chars()
                        .filter(|x| x.is_ascii_digit())
                        .last()
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                )
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        let prefixes = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        s.lines()
            .map(|mut x| {
                let mut vals: Vec<u32> = vec![];
                'outer: loop {
                    if x.is_empty() {
                        break 'outer;
                    }
                    for (i, p) in prefixes.iter().enumerate() {
                        if let Some(suffix) = x.strip_prefix(p) {
                            x = suffix;
                            vals.push(u32::try_from(i).unwrap());
                            continue 'outer;
                        }
                    }
                    let mut chars = x.chars();
                    let c = chars.next().unwrap();
                    if let Some(digit) = c.to_digit(10) {
                        vals.push(digit);
                    }
                    x = chars.as_str();
                }

                (*vals.first().unwrap(), *vals.last().unwrap())
            })
            .collect()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter().map(|(a, b)| a * 10 + b).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter().map(|(a, b)| a * 10 + b).sum()
    }
}

crate::default_tests!(54450, 54258);
crate::string_tests!(
    [(
        sol1,
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        142
    )],
    [(
        sol2,
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        281
    )]
);
