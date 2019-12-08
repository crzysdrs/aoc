use std::io::Result as IoResult;
use crate::intcode::IntCodeMachine;

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day2.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![]);
    m.alarm1202();
    m.run();
    println!("Day 2 P1: {}", m.code()[0]);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day2.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut m = IntCodeMachine::new(codes.clone(), vec![]);
            m.inputs(noun, verb);
            m.run();
            if m.code()[0] == 19690720 {
                println!("Day 2 P2: {}", 100 * noun + verb);
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_codes() {
        let mut m = IntCodeMachine::new(vec![1, 0, 0, 0, 99], vec![]);
        m.run();
        assert_eq!(m.code(), &[2, 0, 0, 0, 99]);

        let mut m = IntCodeMachine::new(vec![2, 3, 0, 3, 99], vec![]);
        m.run();
        assert_eq!(m.code(), &[2, 3, 0, 6, 99]);

        let mut m = IntCodeMachine::new(vec![2, 4, 4, 5, 99, 0], vec![]);
        m.run();
        assert_eq!(m.code(), &[2, 4, 4, 5, 99, 9801]);

        let mut m = IntCodeMachine::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]);
        m.run();
        assert_eq!(m.code(), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);



    }
}
