use crate::intcode::IntCodeMachine;
use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::{HashMap, HashSet};
use std::io::Result as IoResult;

use itertools::Itertools;
use std::convert::TryFrom;

use std::fmt;

impl fmt::Display for SpringWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpringWrite::T => "T",
                SpringWrite::J => "J",
            }
        )
    }
}

impl fmt::Display for SpringRead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpringRead::A => "A",
                SpringRead::B => "B",
                SpringRead::C => "C",
                SpringRead::D => "D",
                SpringRead::E => "E",
                SpringRead::F => "F",
                SpringRead::G => "G",
                SpringRead::H => "H",
                SpringRead::I => "I",
                SpringRead::T => "T",
                SpringRead::J => "J",
            }
        )
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Spring::And(x, y) => write!(f, "AND {} {}", x, y),
            Spring::Or(x, y) => write!(f, "OR {} {}", x, y),
            Spring::Not(x, y) => write!(f, "NOT {} {}", x, y),
            Spring::Walk => write!(f, "WALK"),
            Spring::Run => write!(f, "RUN"),
        }
    }
}

enum SpringWrite {
    T,
    J,
}

enum SpringRead {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    T,
    J,
}

enum Spring {
    And(SpringRead, SpringWrite),
    Or(SpringRead, SpringWrite),
    Not(SpringRead, SpringWrite),
    Walk,
    Run,
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day21.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    // Computes ((~A | ~B | ~C) && D)
    let cmds = vec![
        Spring::Not(SpringRead::A, SpringWrite::T),
        Spring::Not(SpringRead::B, SpringWrite::J),
        Spring::Or(SpringRead::J, SpringWrite::T),
        Spring::Not(SpringRead::C, SpringWrite::J),
        Spring::Or(SpringRead::J, SpringWrite::T),
        Spring::Not(SpringRead::D, SpringWrite::J),
        Spring::Not(SpringRead::J, SpringWrite::J),
        Spring::And(SpringRead::T, SpringWrite::J),
        Spring::Walk,
    ]
    .into_iter()
    .map(|x| format!("{}\n", x))
    .join("")
    .as_bytes()
    .iter()
    .map(|x| *x as isize)
    .collect::<Vec<_>>();

    let mut machine = IntCodeMachine::new(codes.clone(), cmds);

    machine.run();
    use std::convert::TryFrom;

    //println!("{}", machine.output().into_iter().map(|x| x as u8 as char).collect::<String>());
    println!("{:?}", machine.output().into_iter().last().unwrap());
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day21.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    // ~(ABC)D(H + E(I + F))
    let cmds = vec![
        Spring::Or(SpringRead::F, SpringWrite::J),
        Spring::Or(SpringRead::I, SpringWrite::J),
        Spring::And(SpringRead::E, SpringWrite::J),
        Spring::Or(SpringRead::H, SpringWrite::J),
        Spring::And(SpringRead::D, SpringWrite::J),
        Spring::Or(SpringRead::A, SpringWrite::T),
        Spring::And(SpringRead::B, SpringWrite::T),
        Spring::And(SpringRead::C, SpringWrite::T),
        Spring::Not(SpringRead::T, SpringWrite::T),
        Spring::And(SpringRead::T, SpringWrite::J),
        Spring::Run,
    ]
    .into_iter()
    .map(|x| format!("{}\n", x))
    .join("")
    .as_bytes()
    .iter()
    .map(|x| *x as isize)
    .collect::<Vec<_>>();

    let mut machine = IntCodeMachine::new(codes.clone(), cmds);

    machine.run();
    use std::convert::TryFrom;

    //println!("{}", machine.output().into_iter().map(|x| x as u8 as char).collect::<String>());
    println!("{:?}", machine.output().into_iter().last().unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(true);
    }
}
