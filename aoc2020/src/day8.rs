use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Eq, PartialEq, Clone)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}
#[derive(Clone)]
pub struct Instr {
    op: OpCode,
    arg: i32,
}

struct Machine {
    code: Vec<Instr>,
    pc: usize,
    acc: i32,
}

impl Machine {
    fn new(code: Vec<Instr>) -> Machine {
        Machine {
            code,
            pc: 0,
            acc: 0,
        }
    }
    fn run(&mut self) {
        if self.pc < self.code.len() {
            let i = &self.code[self.pc];
            let mut next_pc = self.pc + 1;
            match i.op {
                OpCode::Nop => {}
                OpCode::Acc => {
                    self.acc += i.arg;
                }
                OpCode::Jmp => {
                    next_pc = (self.pc as isize + i.arg as isize) as usize;
                }
            }
            self.pc = next_pc;
        }
    }

    fn acc(&self) -> i32 {
        self.acc
    }
}

impl Iterator for Machine {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc == self.code.len() {
            None
        } else {
            self.run();
            Some((self.pc, self.acc))
        }
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 8;
    type Input = Instr;
    type Sol1 = i32;
    type Sol2 = i32;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|x| {
                let x = x?;
                let mut white = x.split(' ');
                let op = white.next().unwrap();
                let num = white.next().unwrap();

                let op = match op {
                    "nop" => OpCode::Nop,
                    "acc" => OpCode::Acc,
                    "jmp" => OpCode::Jmp,
                    _ => unreachable!("Bad opcode {}", op),
                };

                let arg = num.parse::<i32>().unwrap();

                Ok(Instr { op, arg })
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let m = Machine::new(v.to_vec());

        m.scan(HashSet::new(), |state, (pc, acc)| {
            if state.contains(&pc) {
                None
            } else {
                state.insert(pc);
                Some(acc)
            }
        })
        .last()
        .unwrap()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut machines = v
            .iter()
            .enumerate()
            .filter(|(_, instr)| instr.op == OpCode::Nop || instr.op == OpCode::Jmp)
            .map(|(i, instr)| {
                let new = match instr.op {
                    OpCode::Nop => OpCode::Jmp,
                    OpCode::Jmp => OpCode::Nop,
                    _ => unreachable!(),
                };
                let mut new_code = v.to_vec();
                new_code[i].op = new;
                Machine::new(new_code)
            })
            .collect::<Vec<_>>();

        loop {
            for m in &mut machines {
                if m.next().is_none() {
                    return m.acc();
                }
            }
        }
        // I would have preferred this solution to work
        // but running (potentially) infinite loops in each thread
        // doesn't work, since it won't schedule jobs until old ones finish.
        // machines.into_par_iter().find_map_any(|x| {
        //     println!("Running"); x.last()
        // }).unwrap().1
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    #[test]
    fn test() {
        //unimplemented!()
    }
}
