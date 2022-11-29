use regex::Regex;
use std::collections::VecDeque;
use std::io::Result as IoResult;

enum Cmd {
    Cut(isize),
    Reverse,
    Inc(usize),
}

fn mutate_cards(cmds: String, len: usize) -> Vec<usize> {
    let inc = Regex::new(r#"deal with increment ([0-9]+)"#).unwrap();
    let new = Regex::new(r#"deal into new stack"#).unwrap();
    let cut = Regex::new(r#"cut (-?[0-9]+)"#).unwrap();
    let cmds = cmds
        .lines()
        .map(|x| {
            println!(" item {}", x);
            if inc.is_match(x) {
                Cmd::Inc(inc.captures(x).unwrap()[1].parse::<usize>().unwrap())
            } else if new.is_match(x) {
                Cmd::Reverse
            } else if cut.is_match(x) {
                Cmd::Cut(cut.captures(x).unwrap()[1].parse::<isize>().unwrap())
            } else {
                panic!("Unhandled item {}", x);
            }
        })
        .collect::<Vec<_>>();

    let mut v = (0..len).collect::<VecDeque<_>>();
    let mut tmp = vec![0; len];

    for c in cmds {
        match c {
            Cmd::Inc(i) => {
                (0..)
                    .step_by(i)
                    .enumerate()
                    .take(len)
                    .for_each(|(from_idx, to_idx)| {
                        tmp[to_idx % len] = v[from_idx];
                    });
                v.clear();
                v.extend(tmp.iter());
            }
            Cmd::Reverse => {
                tmp.clear();
                tmp.extend(v.iter());
                tmp.reverse();
                v.clear();
                v.extend(tmp.iter());
            }
            Cmd::Cut(i) => {
                if i > 0 {
                    v.rotate_left(i.unsigned_abs());
                } else {
                    v.rotate_right(i.unsigned_abs());
                }
            }
        }
    }
    v.iter().cloned().collect::<Vec<_>>()
}

pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day22.txt")?;
    let v = mutate_cards(s, 10_007);

    println!(
        "Position of card 2019 : {:?}",
        v.iter().position(|x| *x == 2019)
    );
    Ok(())
}

use num_bigint::BigInt;
use num_bigint::ToBigInt;

fn find_pos(cmds: String, shuffle: usize, len: usize) -> usize {
    let inc = Regex::new(r#"deal with increment ([0-9]+)"#).unwrap();
    let new = Regex::new(r#"deal into new stack"#).unwrap();
    let cut = Regex::new(r#"cut (-?[0-9]+)"#).unwrap();

    let b_len = len.to_bigint().unwrap();
    let b_shuffle = shuffle.to_bigint().unwrap();

    let cmds = cmds
        .lines()
        .map(|x| {
            println!(" item {}", x);
            if inc.is_match(x) {
                Cmd::Inc(inc.captures(x).unwrap()[1].parse::<usize>().unwrap())
            } else if new.is_match(x) {
                Cmd::Reverse
            } else if cut.is_match(x) {
                Cmd::Cut(cut.captures(x).unwrap()[1].parse::<isize>().unwrap())
            } else {
                panic!("Unhandled item {}", x);
            }
        })
        .collect::<Vec<_>>();

    let (a, b) = cmds
        .iter()
        .map(|c| match c {
            Cmd::Inc(i) => ((*i as i64), 0),
            Cmd::Reverse => (-1, -1),
            Cmd::Cut(i) => (1, -(*i as i64)),
        })
        .map(|(a, b)| (a.to_bigint().unwrap(), b.to_bigint().unwrap()))
        .fold(None, |state, (c, d)| {
            if let Some((a, b)) = state {
                //println!("{:?} {:?}", (a, b), (c, d));
                Some((a * &c % &b_len, b * &c + &d % &b_len))
            } else {
                Some((c, d))
            }
        })
        .unwrap();

    #[allow(dead_code)]
    fn exp(x: &BigInt, n: &BigInt) -> BigInt {
        if *n == 0.to_bigint().unwrap() {
            1.to_bigint().unwrap()
        } else if *n == 1.to_bigint().unwrap() {
            x.clone()
        } else if n.clone() % 2 == 0.to_bigint().unwrap() {
            exp(&(x * x), &(n / 2))
        } else {
            x * exp(&(x * x), &((n - 1) / 2))
        }
    }

    fn inv(a: &BigInt, n: &BigInt) -> BigInt {
        a.modpow(&(n - 2), n)
    }

    // compute number of shuffles
    let (a, b) = {
        let a_k = a.modpow(&b_shuffle, &b_len);
        let b = (&b * (&a_k - 1) * inv(&(&a - 1), &b_len)) % &b_len;
        (a_k, b)
    };

    // compute inverse
    let v = ((2020.to_bigint().unwrap() - &b) * inv(&a, &b_len)) % &b_len;

    //make it positive
    let v: BigInt = (v + &b_len) % &b_len;
    v.try_into().unwrap()
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day22.txt")?;
    let v = find_pos(s, 101741582076661, 119315717514047);

    println!("Position of card 2020 : {:?}", v);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            mutate_cards(
                "deal with increment 7
deal into new stack
deal into new stack"
                    .to_string(),
                10
            ),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );

        assert_eq!(
            mutate_cards(
                "cut 6
deal with increment 7
deal into new stack"
                    .to_string(),
                10
            ),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        );

        assert_eq!(
            mutate_cards(
                "deal with increment 7
deal with increment 9
cut -2"
                    .to_string(),
                10
            ),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        );

        assert_eq!(
            mutate_cards(
                "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"
                    .to_string(),
                10
            ),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        );
    }
}
