use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone)]
pub struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    c: Computer,
    program: Vec<u8>,
}

impl ToString for Computer {
    fn to_string(&self) -> String {
        format!(
            "A: {:b} B: {:b} C: {:b}, IP: {}",
            self.a, self.b, self.c, self.ip,
        )
    }
}
impl ToString for Input {
    fn to_string(&self) -> String {
        format!(
            "{}\nInstr:\n{}",
            self.c.to_string(),
            self.program
                .chunks_exact(2)
                .map(|c| Instr::parse(c).to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Iterator for Input {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            println!("{}", self.c.to_string());
            if self.c.ip == self.program.len() {
                return None;
            }
            let i = Instr::parse(&self.program[self.c.ip..]);
            println!("{}", i.to_string());

            let v = i.compute(&mut self.c);
            if v.is_some() {
                return v;
            }
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
}
#[derive(Debug)]
enum Combo {
    Register(Register),
    Literal(u8),
}

impl From<u8> for Combo {
    fn from(v: u8) -> Combo {
        match v {
            0..=3 => Combo::Literal(v),
            4 => Combo::Register(Register::A),
            5 => Combo::Register(Register::B),
            6 => Combo::Register(Register::C),
            _ => panic!(),
        }
    }
}

enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> OpCode {
        match v {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!(),
        }
    }
}

struct Instr {
    op: OpCode,
    combo: Combo,
}

impl Combo {
    fn compute(&self, c: &Computer) -> i64 {
        match self {
            Combo::Register(Register::A) => c.a,
            Combo::Register(Register::B) => c.b,
            Combo::Register(Register::C) => c.c,
            Combo::Literal(v) => i64::from(*v),
        }
    }
}
impl Instr {
    fn compute(&self, c: &mut Computer) -> Option<u8> {
        let mut jumped = false;
        let mut output = None;
        match self.op {
            OpCode::Adv => c.a = c.a >> self.combo.compute(c),
            OpCode::Bdv => c.b = c.a >> self.combo.compute(c),
            OpCode::Cdv => c.c = c.a >> self.combo.compute(c),
            OpCode::Bst => c.b = self.combo.compute(c) & 0x7,
            OpCode::Bxl => c.b = c.b ^ self.combo.compute(c),
            OpCode::Jnz => {
                if c.a != 0 {
                    c.ip = self.combo.compute(c).try_into().unwrap();
                    jumped = true;
                }
            }
            OpCode::Bxc => c.b = c.b ^ c.c,
            OpCode::Out => output = Some((self.combo.compute(c) & 0x7) as u8),
        }

        if !jumped {
            c.ip += 2;
        }
        output
    }
}

impl ToString for Instr {
    fn to_string(&self) -> String {
        match self.op {
            OpCode::Adv => {
                format!(
                    "ADV {:?} = {:?} / 2**{:?}",
                    Register::A,
                    Register::A,
                    self.combo
                )
            }
            OpCode::Bdv => {
                format!(
                    "BDV {:?} = {:?} / 2**{:?}",
                    Register::B,
                    Register::A,
                    self.combo
                )
            }
            OpCode::Cdv => {
                format!(
                    "CDV {:?} = {:?} / 2**{:?}",
                    Register::C,
                    Register::A,
                    self.combo
                )
            }
            OpCode::Bst => {
                format!("BST {:?} = {:?} % 8", Register::B, self.combo)
            }
            OpCode::Bxl => {
                format!(
                    "BXL {:?} = {:?} ^ {:?}",
                    Register::B,
                    Register::B,
                    self.combo
                )
            }
            OpCode::Jnz => {
                format!("JNZ {:?}", self.combo)
            }
            OpCode::Bxc => {
                format!(
                    "BXC {:?} = {:?} ^ {:?}",
                    Register::B,
                    Register::B,
                    Register::C
                )
            }
            OpCode::Out => {
                format!("OUT {:?} % 8", self.combo)
            }
        }
    }
}
impl Instr {
    fn parse(program: &[u8]) -> Instr {
        let op = OpCode::from(program[0]);
        let n = program[1];
        let combo = match op {
            OpCode::Adv | OpCode::Bst | OpCode::Bxc | OpCode::Out | OpCode::Bdv | OpCode::Cdv => {
                Combo::from(n)
            }
            OpCode::Bxl | OpCode::Jnz => Combo::Literal(n),
        };
        Instr { op, combo }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 17;
    type Input1 = Input;
    type Input2 = Input;
    type Sol1 = String;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let a = lines
            .by_ref()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let b = lines
            .by_ref()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let c = lines
            .by_ref()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let _ = lines.by_ref().next().unwrap();

        let program = lines
            .by_ref()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        Input {
            program,
            c: Computer { ip: 0, a, b, c },
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let v = (*v).clone();
        println!("{:?}", v);
        println!("{}", v.to_string());
        v.map(|s| s.to_string()).collect::<Vec<String>>().join(",")
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        println!("{:?}", v);
        println!("{}", v.to_string());
        let prog = v.program.clone();
        let mut digit = vec![0];
        for val in prog.iter().rev() {
            let mut new_digit = vec![];
            for d in &digit {
                for x in 0..=0x7 {
                    let mut v = (*v).clone();
                    v.c.a = (*d << 3) | x;
                    let keep_a = v.c.a;
                    if v.next().to_owned() == Some(*val) {
                        new_digit.push(keep_a);
                    }
                }
            }

            std::mem::swap(&mut digit, &mut new_digit);
        }
        let mut tmp = v.clone();
        tmp.c.a = digit[0];
        println!("{:?}", tmp.collect::<Vec<_>>());

        (digit[0]) as usize
    }
}

#[cfg(test)]
mod day17_custom {
    use super::*;
    fn run(c: &mut Input) -> Vec<u8> {
        c.collect()
    }
    #[test]
    fn test1() {
        let mut v = Input {
            c: Computer {
                ip: 0,
                a: 0,
                b: 0,
                c: 9,
            },
            program: vec![2, 6],
        };
        let _out = run(&mut v);
        assert_eq!(v.c.b, 1);
    }
    #[test]
    fn test2() {
        let mut v = Input {
            c: Computer {
                ip: 0,
                a: 2024,
                b: 0,
                c: 0,
            },
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let out = run(&mut v);
        assert_eq!(out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(v.c.a, 0);
    }
    #[test]
    fn test3() {
        let mut v = Input {
            c: Computer {
                ip: 0,
                a: 0,
                b: 29,
                c: 0,
            },
            program: vec![1, 7],
        };
        let _out = run(&mut v);
        assert_eq!(v.c.b, 26);
    }
    #[test]
    fn test4() {
        let mut v = Input {
            c: Computer {
                ip: 0,
                a: 0,
                b: 2024,
                c: 43690,
            },
            program: vec![4, 0],
        };
        let _out = run(&mut v);
        assert_eq!(v.c.b, 44354);
    }
}
crate::default_tests!("5,1,3,4,3,7,2,1,7", 216584205979245);
crate::string_tests!(
    [
        (
            foo_sol1,
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
            "4,6,3,5,6,3,5,2,1,0"
        ),
        (
            foo_sol1123,
            "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",
            "0,1,2"
        ),
        (
            foo_sol11234,
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
            "4,2,5,6,7,7,7,7,3,1,0"
        )
    ],
    [(
        foo_sol2,
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        117440
    )]
);
