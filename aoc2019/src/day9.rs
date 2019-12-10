use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use crate::intcode::IntCodeMachine;

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day9.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![1]);
    m.run();
    println!("Day9 Part1 {:?}", m.output().collect::<Vec<_>>());
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day9.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![2]);
    m.run();
    println!("Day9 Part2 {:?}", m.output().collect::<Vec<_>>());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        let mut m = IntCodeMachine::new(
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            vec![],
        );
        m.run();
        assert_eq!(
            m.output().cloned().collect::<Vec<_>>(),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        let mut m = IntCodeMachine::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], vec![]);
        m.run();
        assert_eq!(
            m.output().cloned().collect::<Vec<_>>(),
            vec![1219070632396864]
        );

        let mut m = IntCodeMachine::new(vec![104, 1125899906842624, 99], vec![]);
        m.run();
        assert_eq!(
            m.output().cloned().collect::<Vec<_>>(),
            vec![1125899906842624]
        );
    }
}
