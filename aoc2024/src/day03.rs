use crate::Day;
use regex;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 3;
    type Input1 = String;
    type Input2 = String;
    type Sol1 = i64;
    type Sol2 = i64;

    fn process_input1(s: &str) -> Self::Input1 {
        s.to_string()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let r = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        r.captures_iter(v)
            .map(|m| {
                let a: i64 = m[1].parse().unwrap();
                let b: i64 = m[2].parse().unwrap();
                a * b
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let r = regex::Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut enabled = true;
        r.captures_iter(v)
            .filter_map(|c| {
                if c[0].starts_with("do()") {
                    enabled = true;
                    None
                } else if c[0].starts_with("don't()") {
                    enabled = false;
                    None
                } else if enabled {
                    Some(c)
                } else {
                    None
                }
            })
            .map(|m| {
                let a: i64 = m[1].parse().unwrap();
                let b: i64 = m[2].parse().unwrap();
                a * b
            })
            .sum()
    }
}

//crate::default_tests!((), ());
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
