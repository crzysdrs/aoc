use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0u8, |state, c| ((u32::from(state) + c as u32) * 17) as u8)
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 15;
    type Input1 = Vec<String>;
    type Input2 = Vec<String>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .flat_map(|l| l.split(','))
            .map(|s| s.to_string())
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter().map(|s| hash(s) as usize).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut boxes = vec![vec![]; 256];

        v.iter().for_each(|s| {
            if let Some((label, length)) = s.split_once('=') {
                let b = &mut boxes[hash(label) as usize];
                let length = length.parse::<usize>().unwrap();
                if let Some(pos) = b.iter().position(|(l, _v)| *l == label) {
                    b[pos] = (label, length);
                } else {
                    b.push((label, length));
                }
            } else if let Some((label, _)) = s.split_once('-') {
                let b = &mut boxes[hash(label) as usize];
                if let Some(pos) = b.iter().position(|(l, _v)| *l == label) {
                    b.remove(pos);
                }
            } else {
                panic!()
            }
        });

        boxes
            .iter()
            .zip(1..)
            .flat_map(|(b, n)| b.iter().zip(1..).map(move |(l, s)| n * s * l.1))
            .sum()
    }
}

crate::default_tests!(510388, 291774);
crate::string_tests!(
    [(
        foo_sol1,
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        1320
    )],
    [(
        foo_sol2,
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        145
    )]
);
