use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 5;
    type Input1 = Input;
    type Input2 = Input;
    type Sol1 = String;
    type Sol2 = String;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let v: Vec<Vec<_>> = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|l| {
                l.chars()
                    .skip(1)
                    .step_by(4)
                    .map(|x| match x {
                        ' ' | '1'..='9' => None,
                        a => Some(a),
                    })
                    .collect()
            })
            .collect();

        let mut stacks = vec![vec![]; v.len()];
        v.iter().for_each(|r| {
            r.iter().enumerate().for_each(|(i, c)| {
                if let Some(p) = c {
                    stacks[i].insert(0, *p);
                }
            })
        });

        let moves: Vec<_> = lines
            .map(|l| {
                let v: Vec<_> = l.split(' ').collect();
                Move {
                    count: v[1].parse().unwrap(),
                    from: v[3].parse::<usize>().unwrap() - 1,
                    to: v[5].parse::<usize>().unwrap() - 1,
                }
            })
            .collect();

        Input { stacks, moves }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut stacks = v.stacks.clone();

        v.moves.iter().for_each(|m| {
            (0..m.count).for_each(|_| {
                let e = stacks[m.from].pop().unwrap();
                stacks[m.to].push(e)
            });
        });

        stacks
            .iter_mut()
            .map(|s| s.pop())
            .flatten()
            .collect::<String>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut stacks = v.stacks.clone();

        v.moves.iter().for_each(|m| {
            let mut popped: Vec<_> = (0..m.count)
                .map(|_| stacks[m.from].pop().unwrap())
                .collect();
            popped.reverse();
            stacks[m.to].extend(popped);
        });

        stacks
            .iter_mut()
            .map(|s| s.pop())
            .flatten()
            .collect::<String>()
    }
}

crate::default_tests!("CVCWCRTVQ", "CNSCZWLVT");
crate::path_tests!(
    [(sol1, "test/day05.txt", "CMZ")],
    [(sol2, "test/day05.txt", "MCD")]
);
