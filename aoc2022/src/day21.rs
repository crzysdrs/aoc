use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    Mul,
    Add,
    Div,
    Sub,
}

impl Op {
    fn comm(&self) -> bool {
        match self {
            Op::Mul | Op::Add => true,
            _ => false,
        }
    }

    fn rev(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Div => a * b,
            Op::Mul => a / b,
            Op::Add => a - b,
            Op::Sub => a + b,
        }
    }

    fn op(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Div => a / b,
            Op::Mul => a * b,
            Op::Add => a + b,
            Op::Sub => a - b,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    name: String,
    math: Math,
}

#[derive(Debug)]
pub enum Math {
    Constant(i64),
    BinOp(Op, String, String),
}

#[derive(Debug)]
pub enum Math2 {
    Constant(i64),
    Human,
    BinOp(Op, Box<Math2>, Box<Math2>),
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 21;
    type Input1 = Vec<Monkey>;
    type Input2 = Vec<Monkey>;
    type Sol1 = i64;
    type Sol2 = i64;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let (monkey, math) = l.split_once(':').unwrap();

                let items: Vec<_> = math.split_whitespace().collect();
                let math = if items.len() == 1 {
                    Math::Constant(items[0].parse().unwrap())
                } else {
                    let op = match items[1] {
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        "-" => Op::Sub,
                        "+" => Op::Add,
                        _ => unreachable!(),
                    };
                    Math::BinOp(op, items[0].to_string(), items[2].to_string())
                };
                Monkey {
                    name: monkey.to_string(),
                    math,
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        //println!("{:?}", v);
        let mut vals: HashMap<String, _> = HashMap::new();

        loop {
            if let Some(v) = vals.get("root") {
                return *v;
            }
            for m in v {
                if vals.get(&m.name).is_some() {
                    continue;
                }
                let new_val = match &m.math {
                    Math::Constant(v) => *v,
                    Math::BinOp(op, m1, m2) => match (vals.get(m1), vals.get(m2)) {
                        (Some(m1), Some(m2)) => match op {
                            Op::Mul => m1 * m2,
                            Op::Div => m1 / m2,
                            Op::Sub => m1 - m2,
                            Op::Add => m1 + m2,
                        },
                        _ => continue,
                    },
                };

                vals.insert(m.name.clone(), new_val);
            }
        }
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        fn simplify(monkeys: &[Monkey], monkey: &Monkey) -> Math2 {
            if monkey.name == "humn" {
                return Math2::Human;
            }
            match &monkey.math {
                Math::Constant(v) => Math2::Constant(*v),
                Math::BinOp(op, m1, m2) => {
                    match (
                        simplify(monkeys, monkeys.iter().find(|m| m.name == *m1).unwrap()),
                        simplify(monkeys, monkeys.iter().find(|m| m.name == *m2).unwrap()),
                    ) {
                        (Math2::Constant(v1), Math2::Constant(v2)) => Math2::Constant(match op {
                            Op::Mul => v1 * v2,
                            Op::Div => v1 / v2,
                            Op::Sub => v1 - v2,
                            Op::Add => v1 + v2,
                        }),
                        (a, b) => Math2::BinOp(*op, Box::new(a), Box::new(b)),
                    }
                }
            }
        }
        fn algebra(val: i64, m: &Math2) -> i64 {
            match m {
                Math2::BinOp(op, m1, m2) => match (&**m1, &**m2) {
                    (Math2::Constant(v), o) | (o, Math2::Constant(v)) if op.comm() => {
                        algebra(op.rev(val, *v), o)
                    }
                    (Math2::Constant(v), o) => algebra(op.op(*v, val), o),
                    (o, Math2::Constant(v)) => algebra(op.rev(val, *v), o),
                    (Math2::BinOp(_, _, _), Math2::Human)
                    | (Math2::Human, Math2::BinOp(_, _, _))
                    | (Math2::BinOp(_, _, _), Math2::BinOp(_, _, _))
                    | (Math2::Human, Math2::Human) => unreachable!(),
                },
                Math2::Constant(_v) => panic!(),
                Math2::Human => val,
            }
        }
        let simplified = simplify(v, v.iter().find(|m| m.name == "root").unwrap());
        match simplified {
            Math2::BinOp(_, m1, m2) => {
                if let Math2::Constant(v) = *m2 {
                    algebra(v, &m1)
                } else {
                    panic!()
                }
            }
            _ => panic!(),
        }
    }
}

crate::default_tests!(232974643455000, 3740214169961);
crate::path_tests!([(t1, "test/day21.txt", 152)], [(t2, "test/day21.txt", 301)]);
