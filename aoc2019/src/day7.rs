use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use std::convert::TryFrom;

#[derive(Debug)]
struct IntCodeMachine {
    code: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
    ip: usize,
    halted: bool,
}

impl IntCodeMachine {
    fn new(code: Vec<isize>, mut input: Vec<isize>) -> IntCodeMachine {
        input.reverse();
        IntCodeMachine {
            ip: 0,
            halted: false,
            code,
            input,
            output: Vec::new(),
        }
    }
    fn halted(&self) -> bool {
        self.halted
    }
    fn feed_input(&mut self, v: isize) {
        self.input.reverse();
        self.input.push(v);
        self.input.reverse();
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
    fn run(&mut self) -> bool {
        loop {
            match IntCode::from(&self.code[self.ip..]) {
                IntCode::End => {
                    self.halted = true;
                    break true;
                }
                i @ IntCode::Save(_) => {
                    if self.input.len() == 0 {
                        break false;
                    }
                    i.exec(
                        &mut self.ip,
                        &mut self.code,
                        &mut self.input,
                        &mut self.output,
                    );
                }
                i => {
                    let v = i.exec(
                        &mut self.ip,
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
struct IntCodeVal {
    mode: ParameterMode,
    idx: isize,
}

impl IntCodeVal {
    fn read(&self, codes: &[isize]) -> isize {
        match self.mode {
            ParameterMode::Immediate => self.idx,
            ParameterMode::Position => codes[usize::try_from(self.idx).unwrap()],
        }
    }
    fn write(&self, codes: &mut [isize], value: isize) {
        match self.mode {
            ParameterMode::Immediate => panic!("Can't write literal {}", self.idx),
            ParameterMode::Position => codes[usize::try_from(self.idx).unwrap()] = value,
        }
    }
}
#[derive(Debug)]
enum IntCode {
    Add(IntCodeVal, IntCodeVal, IntCodeVal),
    Multiply(IntCodeVal, IntCodeVal, IntCodeVal),
    Save(IntCodeVal),
    Output(IntCodeVal),
    JumpIfTrue(IntCodeVal, IntCodeVal),
    JumpIfFalse(IntCodeVal, IntCodeVal),
    LessThan(IntCodeVal, IntCodeVal, IntCodeVal),
    Equals(IntCodeVal, IntCodeVal, IntCodeVal),
    End,
}
#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl IntCode {
    fn from(codes: &[isize]) -> IntCode {
        let opcode = codes[0] % 100;
        let remain = u32::try_from(codes[0] / 100).unwrap();

        let mut vals = codes[1..]
            .iter()
            .zip(
                (0..)
                    .map(|i| (remain / 10_u32.pow(i)) % 10)
                    .map(|d| match d {
                        0 => ParameterMode::Position,
                        1 => ParameterMode::Immediate,
                        _ => panic!("Unhandled Mode"),
                    }),
            )
            .map(|(v, mode)| IntCodeVal { idx: *v, mode });

        match opcode {
            1 => IntCode::Add(
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            ),
            2 => IntCode::Multiply(
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            ),
            3 => IntCode::Save(vals.next().unwrap()),
            4 => IntCode::Output(vals.next().unwrap()),
            5 => IntCode::JumpIfTrue(vals.next().unwrap(), vals.next().unwrap()),
            6 => IntCode::JumpIfFalse(vals.next().unwrap(), vals.next().unwrap()),
            7 => IntCode::LessThan(
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            ),
            8 => IntCode::Equals(
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            ),
            99 => IntCode::End,
            v => panic!("Unhandled IntCode {}", v),
        }
    }

    fn exec(
        self,
        ip: &mut usize,
        codes: &mut [isize],
        input: &mut Vec<isize>,
        output: &mut Vec<isize>,
    ) {
        match self {
            IntCode::Add(from_a, from_b, to) => {
                to.write(codes, from_a.read(codes) + from_b.read(codes));
                *ip += 4;
            }
            IntCode::Multiply(from_a, from_b, to) => {
                to.write(codes, from_a.read(codes) * from_b.read(codes));
                *ip += 4;
            }
            IntCode::Output(to) => {
                output.push(to.read(codes));
                *ip += 2;
            }
            IntCode::Save(to) => {
                to.write(codes, input.pop().expect("Need Some input"));
                *ip += 2;
            }
            IntCode::JumpIfTrue(test, new_ip) => {
                if test.read(codes) != 0 {
                    *ip = usize::try_from(new_ip.read(codes)).unwrap();
                } else {
                    *ip += 3;
                }
            }
            IntCode::JumpIfFalse(test, new_ip) => {
                if test.read(codes) == 0 {
                    *ip = usize::try_from(new_ip.read(codes)).unwrap();
                } else {
                    *ip += 3;
                }
            }
            IntCode::LessThan(first, second, flag) => {
                flag.write(
                    codes,
                    if first.read(codes) < second.read(codes) {
                        1
                    } else {
                        0
                    },
                );
                *ip += 4;
            }
            IntCode::Equals(first, second, flag) => {
                flag.write(
                    codes,
                    if first.read(codes) == second.read(codes) {
                        1
                    } else {
                        0
                    },
                );
                *ip += 4;
            }
            IntCode::End => {}
        }
    }
}

pub fn series_machine(codes: &[isize], feedback: bool) -> isize {
    let starts = if feedback { 5..=9 } else { 0..=4 };
    starts
        .permutations(5)
        .map({
            |phases| {
                let p = phases.clone();
                let mut machines = phases
                    .iter()
                    .enumerate()
                    .map(|(i, phase)| (i, IntCodeMachine::new(codes.to_vec(), vec![*phase])))
                    .collect::<Vec<_>>();

                let mut start_input = 0;
                loop {
                    let v = machines
                        .iter_mut()
                        .scan(start_input, |input, (num, machine)| {
                            machine.feed_input(*input);
                            machine.run();
                            *input = machine.output.pop().unwrap();
                            Some(*input)
                        })
                        .last()
                        .unwrap();
                    if machines[0].1.halted {
                        break v;
                    } else {
                        start_input = v;
                    }
                }
            }
        })
        .max()
        .unwrap()
}
pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let r = series_machine(&codes, false);
    println!("Day 7 Part 1 {}", r);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let r = series_machine(&codes, true);
    println!("Day 7 Part 1 {}", r);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            series_machine(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                false
            ),
            43210
        );
        assert_eq!(
            series_machine(
                &[
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                false
            ),
            54321
        );
        assert_eq!(
            series_machine(
                &[
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                false
            ),
            65210
        );

        assert_eq!(
            series_machine(
                &[
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                true
            ),
            139629729
        );
        assert_eq!(
            series_machine(
                &[
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                true
            ),
            18216
        );
    }
}
