use crate::Day;
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Mult,
    Plus,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = (Vec<Vec<u64>>, Vec<Op>);
    type Input2 = (Vec<Vec<u64>>, Vec<Op>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let items: Vec<Vec<u64>> = lines
            .by_ref()
            .take_while_ref(|s| !(s.starts_with('*') || s.starts_with('+')))
            .map(|s| s.split_whitespace().map(|v| v.parse().unwrap()).collect())
            .collect();

        let ops: Vec<_> = lines
            .map(|s| {
                s.split_whitespace()
                    .map(|s| match s {
                        "*" => Op::Mult,
                        "+" => Op::Plus,
                        _ => panic!(),
                    })
                    .collect()
            })
            .next()
            .unwrap();

        assert!(items.iter().all(|x| x.len() == ops.len()));

        (items, ops)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        let ops: Vec<_> = s
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| match s {
                "*" => Op::Mult,
                "+" => Op::Plus,
                _ => panic!(),
            })
            .collect();

        let lines: Vec<_> = s.lines().map(|v| v.as_bytes().to_vec()).collect();

        let mut new_lines = vec![];
        for n in 0..lines[0].len() {
            let mut new_line = vec![];
            for line in lines.iter().take(lines.len() - 1) {
                new_line.push(line[n]);
            }
            new_lines.push(new_line);
        }

        let lines: Vec<_> = new_lines
            .iter()
            .map(|v| str::from_utf8(v).unwrap())
            .collect();

        let vals = lines
            .split(|v| v.chars().all(|c| c.is_whitespace()))
            .map(|s| s.iter().map(|v| v.trim().parse().unwrap()).collect())
            .collect();

        (vals, ops)
    }
    fn p1((items, ops): &Self::Input1) -> Self::Sol1 {
        ops.iter()
            .enumerate()
            .map(|(i, op)| {
                items.iter().map(|item| item[i]).fold(
                    match op {
                        Op::Mult => 1,
                        Op::Plus => 0,
                    },
                    |mut state, item| {
                        match op {
                            Op::Mult => state *= item,
                            Op::Plus => state += item,
                        }
                        state
                    },
                )
            })
            .sum::<u64>() as usize
    }
    fn p2((items, ops): &Self::Input2) -> Self::Sol2 {
        ops.iter()
            .zip(items.iter())
            .map(|(op, items)| {
                items.iter().fold(
                    match op {
                        Op::Mult => 1,
                        Op::Plus => 0,
                    },
                    |mut state, item| {
                        match op {
                            Op::Mult => state *= item,
                            Op::Plus => state += item,
                        }
                        state
                    },
                )
            })
            .sum::<u64>() as usize
    }
}

crate::default_tests!(5322004718681, 9876636978528);
crate::string_tests!(
    [(
        foo_sol1,
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        4277556
    )],
    [(
        foo_sol2,
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        3263827
    )]
);
