use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}

fn best(real: &mut HashMap<(usize, usize), usize>, count: usize, bank: &[u32]) -> usize {
    let val = if let Some(found) = real.get(&(count, bank.len())) {
        return *found;
    } else if count == 0 {
        0
    } else if count > bank.len() {
        0
    } else if bank.is_empty() {
        0
    } else {
        let (cur_bank, rest) = bank.split_at(1);
        let choose_this =
            cur_bank[0] as usize * 10usize.pow(count as u32 - 1) + best(real, count - 1, rest);
        let dont_choose_this = best(real, count, rest);

        std::cmp::max(choose_this, dont_choose_this)
    };
    real.insert((count, bank.len()), val);
    val
}

impl Day for Solution {
    const DAY: u32 = 3;
    type Input1 = Vec<Vec<u32>>;
    type Input2 = Vec<Vec<u32>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|b| {
                let mut real = HashMap::new();
                best(&mut real, 2, b)
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .map(|b| {
                let mut real = HashMap::new();
                best(&mut real, 12, b)
            })
            .sum()
    }
}

crate::default_tests!(17430, 171975854269367);
crate::string_tests!(
    [(
        foo_sol1,
        "987654321111111
811111111111119
234234234234278
818181911112111",
        357
    )],
    [(
        foo_sol2,
        "987654321111111
811111111111119
234234234234278
818181911112111",
        3121910778619
    )]
);
