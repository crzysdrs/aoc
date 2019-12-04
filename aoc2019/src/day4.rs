use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

fn validate(v: u32, p2: bool) -> bool {
    let digits = (0..6)
        .rev()
        .map(|i| (v / 10_u32.pow(i)) % 10)
        .collect::<Vec<_>>();

    let never_decrease = digits
        .iter()
        .zip(digits[1..].iter())
        .map(|(prev, cur)| prev <= cur)
        .all(|i| i);

    let mut two_item_group = false;

    for (key, group) in &digits.iter().group_by(|x| **x) {
        two_item_group = two_item_group
            || if p2 {
                group.count() == 2
            } else {
                group.count() >= 2
            };
    }

    never_decrease && two_item_group
}

pub fn p1() -> IoResult<()> {
    let v = 372304..=847060;
    println!(
        "Day 4 Part 1 {}",
        v.map(|v| validate(v, false))
            .map(|b| if b { 1 } else { 0 })
            .sum::<u32>()
    );
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let v = 372304..=847060;
    println!(
        "Day 4 Part 2 {}",
        v.map(|v| validate(v, true))
            .map(|b| if b { 1 } else { 0 })
            .sum::<u32>()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(validate(111111));
        assert!(!validate(223450));
        assert!(!validate(123789));
    }
}
