use std::collections::VecDeque;
use std::convert::TryFrom;

#[derive(Debug,Clone)]
pub struct IntCodeMachine {
    code: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    ip: usize,
    halted: bool,
    rel: isize,
}

impl IntCodeMachine {
    pub fn new(code: Vec<isize>, input: Vec<isize>) -> IntCodeMachine {
        IntCodeMachine {
            ip: 0,
            halted: false,
            code,
            input: VecDeque::from(input),
            output: VecDeque::new(),
            rel: 0,
        }
    }

    pub fn code(&self) -> &[isize] {
        &self.code
    }
    pub fn alarm1202(&mut self) {
        self.inputs(12, 2);
    }
    pub fn inputs(&mut self, noun: isize, verb: isize) {
        self.code[1] = noun;
        self.code[2] = verb;
    }
    pub fn next_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }
    pub fn output(&mut self) -> Vec<isize> {
        self.output.drain(0..).collect()
    }
    pub fn halted(&self) -> bool {
        self.halted
    }
    pub fn feed_input(&mut self, v: isize) {
        self.input.push_back(v);
    }
    #[cfg(test)]
    pub fn test(&mut self) -> (Vec<isize>, Vec<isize>) {
        self.run();
        (
            self.input.iter().cloned().collect::<Vec<_>>(),
            self.output.iter().cloned().collect::<Vec<_>>(),
        )
    }
    pub fn done(&self, v: &str) {
        println!(
            "{} Remain Input: {:?}, Output: {:?}",
            v, self.input, self.output
        );
    }
    pub fn run(&mut self) -> bool {
        loop {
            match IntCode::from(&self.code[self.ip..], self.rel) {
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
                        &mut self.rel,
                        &mut self.input,
                        &mut self.output,
                    );
                }
                i => {
                    i.exec(
                        &mut self.ip,
                        &mut self.code,
                        &mut self.rel,
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
    rel: isize,
    idx: isize,
}

impl IntCodeVal {
    fn read(&self, codes: &[isize]) -> isize {
        match self.mode {
            ParameterMode::Immediate => self.idx,
            ParameterMode::Relative | ParameterMode::Position => {
                let idx = match self.mode {
                    ParameterMode::Relative => usize::try_from(self.idx + self.rel).unwrap(),
                    ParameterMode::Position => usize::try_from(self.idx).unwrap(),
                    _ => panic!("Unhandled mode"),
                };
                if idx >= codes.len() {
                    0
                } else {
                    codes[usize::try_from(idx).unwrap()]
                }
            }
        }
    }
    fn write(&self, codes: &mut Vec<isize>, value: isize) {
        let idx = match self.mode {
            ParameterMode::Immediate => panic!("Can't write literal {}", self.idx),
            ParameterMode::Relative => usize::try_from(self.idx + self.rel).unwrap(),
            ParameterMode::Position => usize::try_from(self.idx).unwrap(),
        };
        if idx >= codes.len() {
            codes.resize(idx + 1, 0);
        }
        codes[idx] = value;
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
    AdjustRel(IntCodeVal),
    End,
}
#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl IntCode {
    fn from(codes: &[isize], rel: isize) -> IntCode {
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
                        2 => ParameterMode::Relative,
                        _ => panic!("Unhandled Mode"),
                    }),
            )
            .map(|(v, mode)| IntCodeVal { idx: *v, mode, rel });

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
            9 => IntCode::AdjustRel(vals.next().unwrap()),
            99 => IntCode::End,
            v => panic!("Unhandled IntCode {}", v),
        }
    }

    fn exec(
        self,
        ip: &mut usize,
        codes: &mut Vec<isize>,
        rel: &mut isize,
        input: &mut VecDeque<isize>,
        output: &mut VecDeque<isize>,
    ) {
        match self {
            IntCode::Add(from_a, from_b, to) => {
                to.write(
                    codes,
                    from_a.read(codes, ) + from_b.read(codes, ),
                );
                *ip += 4;
            }
            IntCode::Multiply(from_a, from_b, to) => {
                to.write(
                    codes,
                    
                    from_a.read(codes, ) * from_b.read(codes, ),
                );
                *ip += 4;
            }
            IntCode::Output(to) => {
                output.push_back(to.read(codes, ));
                *ip += 2;
            }
            IntCode::Save(to) => {
                to.write(codes,  input.pop_front().expect("Need Some input"));
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
                    
                    if first.read(codes, ) < second.read(codes, ) {
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
            IntCode::AdjustRel(new_rel) => {
                *rel += new_rel.read(codes);
                *ip += 2;
            }
            IntCode::End => {}
        }
    }
}
