use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

enum IntCode {
    Add(usize, usize, usize),
    Multiply(usize, usize, usize),
    End,
}

impl IntCode {
    fn from(codes: &[usize]) -> IntCode {
        match codes[0] {
            1 => IntCode::Add(codes[1], codes[2], codes[3]),
            2 => IntCode::Multiply(codes[1], codes[2], codes[3]),
            99 => IntCode::End,
            v => panic!("Unhandled IntCode {}", v),
        }
    }
    fn alarm1202(codes: &mut [usize]) {
        codes[1] = 12;
        codes[2] = 2;
    }

    fn inputs(codes: &mut [usize], noun: usize, verb: usize) {
        codes[1] = noun;
        codes[2] = verb;
    }
    fn run(codes: &mut [usize]) {
        let mut start = 0;
        loop {
            match IntCode::from(&codes[start..]) {
                IntCode::End => break,
                i => {
                    let v = i.exec(codes);
                    start += v.unwrap();
                }
            }
        }
    }
    fn exec(self, codes: &mut [usize]) -> Option<usize> {
        match self {
            IntCode::Add(from_a, from_b, to) => {
                codes[to] = codes[from_a] + codes[from_b];
                Some(4)
            }
            IntCode::Multiply(from_a, from_b, to) => {
                codes[to] = codes[from_a] * codes[from_b];
                Some(4)
            }
            IntCode::End => None,
        }
    }
}
pub fn p1() -> IoResult<()> {
    let mut codes = std::fs::read_to_string("input/day2.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    IntCode::alarm1202(&mut codes);
    IntCode::run(&mut codes);
    println!("Day 2 P1: {}", codes[0]);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day2.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut codes = codes.clone();
            IntCode::inputs(&mut codes, noun, verb);
            IntCode::run(&mut codes);
            if codes[0] == 19690720 {
                println!("noun {} verb {}", noun, verb);
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
        let mut code = [1, 0, 0, 0, 99];
        IntCode::run(&mut code);
        assert_eq!(&code, &[2, 0, 0, 0, 99]);

        let mut code = [2, 3, 0, 3, 99];
        IntCode::run(&mut code);
        assert_eq!(&code, &[2, 3, 0, 6, 99]);

        let mut code = [2, 4, 4, 5, 99, 0];
        IntCode::run(&mut code);
        assert_eq!(&code, &[2, 4, 4, 5, 99, 9801]);

        let mut code = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        IntCode::run(&mut code);
        assert_eq!(&code, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
