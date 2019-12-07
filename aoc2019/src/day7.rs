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
    halt_input: bool,
    ip: usize,
    halted: bool,
}

impl IntCodeMachine {
    fn new(code: Vec<isize>, mut input: Vec<isize>) -> IntCodeMachine {
        input.reverse();
        IntCodeMachine {
            ip: 0,
            halt_input: false,
            halted: false,
            code,
            input,
            output: Vec::new(),
        }
    }
    fn halt_input(&mut self) {
        self.halt_input = true;
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
    fn alarm1202(&mut self) {
        self.inputs(12, 2);
    }
    fn inputs(&mut self, noun: isize, verb: isize) {
        self.code[1] = noun;
        self.code[2] = verb;
    }
    fn run(&mut self) -> bool {
        loop {
            match IntCode::from(&self.code[self.ip..]) {
                IntCode::End => {
                    self.halted = true;
                    break true;
                }
                i @ IntCode::Save(_) => {
                    if self.halt_input && self.input.len() == 0 {
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

        //println!("Orig {}", codes[0]);
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

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        //"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string()
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let result = (0..=4)
        .permutations(5)
        .map({
            |phases| {
                let p = phases.clone();
                let v = std::iter::successors(Some((phases, 0isize)), |(phases, input)| {
                    if let Some(phase) = phases.first() {
                        let mut m = IntCodeMachine::new(codes.clone(), vec![*phase, *input]);
                        m.run();
                        Some((phases[1..].to_vec(), m.output[0]))
                    } else {
                        None
                    }
                })
                .last()
                .unwrap()
                .1;
                //println!("{:?} {}", p, v);
                v
            }
        })
        .max();

    println!("Result {}", result.unwrap());
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let result = (5..=9)
        .permutations(5)
        .map({
            |phases| {
                let p = phases.clone();
                let mut machines = phases
                    .iter()
                    .enumerate()
                    .map(|(i, phase)| (i, IntCodeMachine::new(codes.clone(), vec![*phase])))
                    .collect::<Vec<_>>();

                let mut start_input = 0;
                loop {
                    let v = machines
                        .iter_mut()
                        .scan(start_input, |input, (num, machine)| {
                            machine.halt_input();
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
        .max();

    println!("Result {}", result.unwrap());
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
