use std::io::Result as IoResult;

use itertools::Itertools;

fn validate(v: u32, p2: bool) -> bool {
    let digits = (0..6)
        .rev()
        .map(|i| (v / 10_u32.pow(i)) % 10)
        .collect::<Vec<_>>();

    digits
        .iter()
        .zip(digits[1..].iter())
        .map(|(prev, cur)| prev <= cur)
        .all(|i| i)
        && digits
            .iter()
            .group_by(|x| **x)
            .into_iter()
            .map(|(_key, group)| {
                if p2 {
                    group.count() == 2
                } else {
                    group.count() >= 2
                }
            })
            .any(|i| i)
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
        assert!(validate(111111, false));
        assert!(!validate(223450, false));
        assert!(!validate(123789, false));
    }
}
