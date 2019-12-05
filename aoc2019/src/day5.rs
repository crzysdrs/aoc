use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use std::convert::TryFrom;

#[derive(Debug)]
struct IntCodeMachine {
    code: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl IntCodeMachine {
    fn new(code: Vec<isize>, input: Vec<isize>) -> IntCodeMachine {
        IntCodeMachine {
            code,
            input,
            output: Vec::new(),
        }
    }
    fn test(&mut self) -> (&[isize], &[isize]) {
        self.run();
        (&self.input, &self.output)
    }
    fn done(&self, v: &str) {
        println!(
            "{} Remain Input: {:?}, Output: {:?}",
            v, self.input, self.output
        );
    }
    fn alarm1202(&mut self) {
        self.inputs(12, 2);
    }
    fn inputs(&mut self, noun: isize, verb: isize) {
        self.code[1] = noun;
        self.code[2] = verb;
    }
    fn run(&mut self) {
        let mut start = 0;
        loop {
            match IntCode::from(&self.code[start..]) {
                IntCode {
                    op: IntCodeOp::End, ..
                } => break,
                i => {
                    let v = i.exec(
                        &mut start,
                        &mut self.code,
                        &mut self.input,
                        &mut self.output,
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
struct IntCode {
    modes: Vec<ParameterMode>,
    op: IntCodeOp,
}
#[derive(Debug)]
enum IntCodeOp {
    Add(isize, isize, isize),
    Multiply(isize, isize, isize),
    Save(isize),
    Output(isize),
    JumpIfTrue(isize, isize),
    JumpIfFalse(isize, isize),
    LessThan(isize, isize, isize),
    Equals(isize, isize, isize),
    End,
}
#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl IntCode {
    fn from(codes: &[isize]) -> IntCode {
        //println!("Orig {}", codes[0]);
        let code = match codes[0] % 100 {
            1 => IntCodeOp::Add(codes[1], codes[2], codes[3]),
            2 => IntCodeOp::Multiply(codes[1], codes[2], codes[3]),
            3 => IntCodeOp::Save(codes[1]),
            4 => IntCodeOp::Output(codes[1]),
            5 => IntCodeOp::JumpIfTrue(codes[1], codes[2]),
            6 => IntCodeOp::JumpIfFalse(codes[1], codes[2]),
            7 => IntCodeOp::LessThan(codes[1], codes[2], codes[3]),
            8 => IntCodeOp::Equals(codes[1], codes[2], codes[3]),
            99 => IntCodeOp::End,
            v => panic!("Unhandled IntCode {}", v),
        };

        let remain = u32::try_from(codes[0] / 100).unwrap();
        //println!("Remain: {}", remain);
        let modes = (0..4)
            //.rev()
            .map(|i| (remain / 10_u32.pow(i)) % 10)
            .map(|d| match d {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!("Unhandled Mode"),
            })
            .collect::<Vec<_>>();

        IntCode { modes, op: code }
    }
    fn read_mode(&self, codes: &[isize], pos: usize, idx: isize) -> isize {
        match self.modes[pos] {
            ParameterMode::Immediate => idx,
            ParameterMode::Position => codes[usize::try_from(idx).unwrap()],
        }
    }
    fn write_mode<'a>(&self, codes: &'a mut [isize], pos: usize, idx: isize) -> &'a mut isize {
        match self.modes[pos] {
            ParameterMode::Immediate => panic!("Can't write literal {} {}", pos, idx),
            ParameterMode::Position => &mut codes[usize::try_from(idx).unwrap()],
        }
    }
    fn exec(
        self,
        ip: &mut usize,
        codes: &mut [isize],
        input: &mut Vec<isize>,
        output: &mut Vec<isize>,
    ) {
        match self.op {
            IntCodeOp::Add(from_a, from_b, to) => {
                *self.write_mode(codes, 2, to) =
                    self.read_mode(codes, 1, from_b) + self.read_mode(codes, 0, from_a);
                *ip += 4;
            }
            IntCodeOp::Multiply(from_a, from_b, to) => {
                *self.write_mode(codes, 2, to) =
                    self.read_mode(codes, 1, from_b) * self.read_mode(codes, 0, from_a);
                *ip += 4;
            }
            IntCodeOp::Output(to) => {
                output.push(self.read_mode(codes, 0, to));
                *ip += 2;
            }
            IntCodeOp::Save(to) => {
                *self.write_mode(codes, 0, to) = input.pop().expect("Some input");
                *ip += 2;
            }
            IntCodeOp::JumpIfTrue(test, new_ip) => {
                if self.read_mode(codes, 0, test) != 0 {
                    *ip = usize::try_from(self.read_mode(codes, 1, new_ip)).unwrap();
                } else {
                    *ip += 3;
                }
            }
            IntCodeOp::JumpIfFalse(test, new_ip) => {
                if self.read_mode(codes, 0, test) == 0 {
                    *ip = usize::try_from(self.read_mode(codes, 1, new_ip)).unwrap();
                } else {
                    *ip += 3;
                }
            }
            IntCodeOp::LessThan(first, second, flag) => {
                *self.write_mode(codes, 2, flag) =
                    if self.read_mode(codes, 0, first) < self.read_mode(codes, 1, second) {
                        1
                    } else {
                        0
                    };
                *ip += 4;
            }
            IntCodeOp::Equals(first, second, flag) => {
                *self.write_mode(codes, 2, flag) =
                    if self.read_mode(codes, 0, first) == self.read_mode(codes, 1, second) {
                        1
                    } else {
                        0
                    };
                *ip += 4;
            }
            IntCodeOp::End => {}
        }
    }
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day5.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut machine = IntCodeMachine::new(codes, vec![1]);
    machine.run();
    machine.done("Part 1");
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day5.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut machine = IntCodeMachine::new(codes, vec![5]);
    machine.run();
    machine.done("Part 2");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]).test(),
            (&[][..], &[1][..])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![5]).test(),
            (&[][..], &[0][..])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![5]).test(),
            (&[][..], &[1][..])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]).test(),
            (&[][..], &[0][..])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8]).test(),
            (&[][..], &[1][..])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![7]).test(),
            (&[][..], &[0][..])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![8]).test(),
            (&[][..], &[0][..])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![7]).test(),
            (&[][..], &[1][..])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![0]
            )
            .test(),
            (&[][..], &[0][..])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![1]
            )
            .test(),
            (&[][..], &[1][..])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![0]
            )
            .test(),
            (&[][..], &[0][..])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![1]
            )
            .test(),
            (&[][..], &[1][..])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![7]
            )
            .test(),
            (&[][..], &[999][..])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![8]
            )
            .test(),
            (&[][..], &[1000][..])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![9]
            )
            .test(),
            (&[][..], &[1001][..])
        );
    }
}
