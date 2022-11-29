use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use regex::Regex;

pub enum Bit {
    B0,
    B1,
    BX,
}

pub enum Instr {
    Mask(Vec<Bit>),
    Mem(usize, usize),
}

struct Machine {
    mem: HashMap<usize, usize>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 14;
    type Input = Instr;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let mask = Regex::new(r"mask = ([01X]+)").unwrap();
        let mem = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").unwrap();
        r.lines()
            .map(|l| {
                let l = l?;

                let instr = if let Some(cap) = mask.captures(&l) {
                    Instr::Mask(
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .chars()
                            .map(|c| match c {
                                '0' => Bit::B0,
                                '1' => Bit::B1,
                                'X' => Bit::BX,
                                _ => panic!(),
                            })
                            .collect(),
                    )
                } else if let Some(cap) = mem.captures(&l) {
                    Instr::Mem(
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                        cap.get(2).unwrap().as_str().parse().unwrap(),
                    )
                } else {
                    panic!()
                };
                Ok(instr)
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut m = Machine {
            mem: HashMap::new(),
        };
        let mut or = 0;
        let mut and = 0;

        v.iter().for_each(|i| match i {
            Instr::Mask(mask) => {
                or = mask.iter().fold(0, |mut v, b| {
                    v <<= 1;
                    if matches!(b, Bit::B1) {
                        v |= 1;
                    }
                    v
                });
                and = mask.iter().fold(0, |mut v, b| {
                    v <<= 1;
                    if !matches!(b, Bit::B0) {
                        v |= 1;
                    }
                    v
                });
            }
            Instr::Mem(addr, val) => {
                let new = (val & and) | or;
                m.mem.entry(*addr).and_modify(|v| *v = new).or_insert(new);
            }
        });

        m.mem.values().sum()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut m = Machine {
            mem: HashMap::new(),
        };
        let mut or = 0;
        let mut float = 0;
        let mut float_count = 0;
        let mut save_mask: &[Bit] = &[];

        v.iter().for_each(|i| match i {
            Instr::Mask(mask) => {
                save_mask = mask.as_slice();
                or = mask.iter().fold(0, |mut v, b| {
                    v <<= 1;
                    if matches!(b, Bit::B1) {
                        v |= 1;
                    }
                    v
                });
                float = mask.iter().fold(0, |mut v, b| {
                    v <<= 1;
                    if matches!(b, Bit::BX) {
                        v |= 1;
                    }
                    v
                });
                float_count = mask.iter().filter(|x| matches!(x, Bit::BX)).count();
            }
            Instr::Mem(addr, val) => {
                let addr = addr | or;
                (0..2usize.pow(float_count as u32))
                    .map(|mut float_v| {
                        let new_float = save_mask.iter().fold(0, |mut v, b| {
                            v <<= 1;
                            if let Bit::BX = b {
                                v |= float_v & 1;
                                float_v >>= 1;
                            }

                            v
                        });
                        (addr & !float) | new_float
                    })
                    .for_each(|addr| {
                        m.mem.entry(addr).and_modify(|v| *v = *val).or_insert(*val);
                    });
            }
        });

        m.mem.values().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 165);

        let s = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 208);
    }
}
