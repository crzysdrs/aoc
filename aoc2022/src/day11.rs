use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone)]
pub struct Monk {
    inspected: usize,
    start: Vec<usize>,
    op: String,
    div_by: usize,
    t: usize,
    f: usize,
}

impl Monk {
    fn inspect(&mut self) -> Vec<(usize, usize)> {
        self.start
            .drain(..)
            .map(|old| {
                self.inspected += 1;
                let mut new = match self.op.as_ref() {
                    "new = old * 19" => old * 19,
                    "new = old * 6" => old * 6,
                    "new = old * old" => old * old,
                    "new = old + 3" => old + 3,
                    "new = old * 3" => old * 3,
                    "new = old + 7" => old + 7,
                    "new = old + 5" => old + 5,
                    "new = old + 8" => old + 8,
                    "new = old + 4" => old + 4,
                    "new = old + 6" => old + 6,
                    "new = old * 2" => old * 2,
                    expr => unimplemented!("{:?}", expr),
                };
                new /= 3;

                if new % self.div_by == 0 {
                    (new, self.t)
                } else {
                    (new, self.f)
                }
            })
            .collect()
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 11;
    type Input1 = Vec<Monk>;
    type Input2 = Vec<Monk>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut input = s.lines();

        let mut monks = vec![];

        loop {
            let mut monk = input.by_ref().take_while(|l| !l.is_empty());
            if monk.next().is_none() {
                break;
            }
            let start = monk
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect::<Vec<_>>();

            let op = monk
                .next()
                .unwrap()
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .to_string();

            let test = monk
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|v| v.parse().unwrap())
                .unwrap();
            let t = monk
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|v| v.parse().unwrap())
                .unwrap();
            let f = monk
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|v| v.parse().unwrap())
                .unwrap();

            monk.next();

            monks.push(Monk {
                inspected: 0,
                start,
                op,
                div_by: test,
                t,
                f,
            })
        }

        monks
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let rounds = 20;

        let mut monks = v.to_vec();
        for _ in 0..rounds {
            for m in 0..monks.len() {
                let worries = monks[m].inspect();
                for w in worries {
                    monks[w.1].start.push(w.0);
                }
            }
        }
        monks.sort_by_key(|m| m.inspected);
        monks.reverse();
        monks.iter().take(2).map(|m| m.inspected).product()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let rounds = 10000;

        let mut monks = v.to_vec();
        let monks_mod: usize = v.iter().map(|m| m.div_by).product();

        for _ in 0..rounds {
            for m in 0..monks.len() {
                let worries = monks[m].inspect();
                for w in worries {
                    monks[w.1].start.push(w.0 % monks_mod);
                }
            }
        }
        monks.sort_by_key(|m| m.inspected);
        monks.reverse();
        monks.iter().take(2).map(|m| m.inspected).product()
    }
}

crate::default_tests!(50616, 11309046332);
crate::path_tests!(
    [(t1, "test/day11.txt", 10605)],
    [(t2, "test/day11.txt", 2713310158)]
);
