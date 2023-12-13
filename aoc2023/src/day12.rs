use crate::Day;
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Spring {
    Unknown,
    Broken,
    Ok,
}

#[derive(Debug)]
pub struct SpringVec(Vec<Spring>);

impl SpringVec {
    fn unknown_bits(&self) -> usize {
        self.0.iter().filter(|s| **s == Spring::Unknown).count()
    }
    fn unknown_mask(&self) -> u128 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, s)| if *s == Spring::Unknown { 1 << i } else { 0 })
            .fold(0, |mut state, v| {
                state |= v;
                state
            })
    }
    fn known_broken(&self) -> u128 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, s)| if *s == Spring::Broken { 1 << i } else { 0 })
            .fold(0, |mut state, v| {
                state |= v;
                state
            })
    }
    fn find_all(&self, sections: &[usize]) -> usize {
        let len = self.0.len();
        let known = self.known_broken();
        let unknown_mask = self.unknown_mask();

        fn remain(
            unknown_mask: u128,
            known: u128,
            incoming: u128,
            sections: &[usize],
            offset: usize,
            len: usize,
        ) -> usize {
            if offset > 0 {
                let width_mask = (1 << offset) - 1;

                // println!("{:b}, {}, {}", incoming, offset, len);
                // println!("{:b}", known);
                // println!("KNown {:b}", known & width_mask);
                // println!("Mine  {:b}", (incoming & !unknown_mask) & width_mask);
                if (incoming & !unknown_mask) & width_mask != known & width_mask {
                    //  println!("Stop");
                    return 0;
                } else {
                    //    println!("Keep going");
                }
            }

            let mut count = 0;
            if !sections.is_empty() {
                let start = if offset == 0 { 0 } else { 1 };
                if len < sections.iter().sum::<usize>() + sections.len() - 1 {
                    //println!("Bail");
                    return count;
                }
                //println!("{:?}", start..(len - sections[0]));
                for i in start..=(len - sections[0]) {
                    let mut incoming = incoming;
                    for j in 0..sections[0] {
                        incoming |= 1 << (offset + j + i);
                    }
                    if len >= i + sections[0] {
                        count += remain(
                            unknown_mask,
                            known,
                            incoming,
                            &sections[1..],
                            offset + i + sections[0],
                            len - i - sections[0],
                        )
                    }
                }
            } else {
                //                println!("Valid? {:b}", incoming);
                count += if incoming & !unknown_mask == known {
                    1
                } else {
                    0
                };
            }
            count
        }
        remain(unknown_mask, known, 0, sections, 0, len)
    }
    // fn valid(&self, bits: u64, sections: &[usize]) -> bool {
    //     let mut bits_idx = 0;

    //     let computed: Vec<_> = self
    //         .0
    //         .iter()
    //         .map(|v| match v {
    //             Spring::Unknown => {
    //                 let n = if (bits & (1 << bits_idx)) != 0 {
    //                     Spring::Broken
    //                 } else {
    //                     Spring::Ok
    //                 };
    //                 bits_idx += 1;
    //                 n
    //             }
    //             v @ (Spring::Broken | Spring::Ok) => *v,
    //         })
    //         //.inspect(|v| println!("{:?}", v))
    //         .group_by(|k| *k)
    //         .into_iter()
    //         .filter(|(k, _g)| *k == Spring::Broken)
    //         .map(|(_k, g)| g.count())
    //         .collect();

    //     //println!("{:?}", computed);
    //     sections == computed
    // }
}

#[derive(Hash, Eq)]
struct Key<'a> {
    spring: &'a [Spring],
    broken: &'a [usize],
}
fn p(seen: &mut HashMap<Key, usize>, spring:&[Spring], broken: &[usize]) -> usize {
    let key =  &Key {
        spring, broken
    };
    if let Some(v) =  seen.get(&key) {
        return v;
    } else if spring.is_empty() && !broken.is_empty() {
        return 0;
    }

    if spring.iter().all(|v| v == Spring::Unknown || v == Spring::Broken) {
        //base case?
    } else {
        for (_b, s) in spring.iter().group_by(|k| k == Spring::Unknown || v == Spring::Broken) 
            .into_iter().filter(|(b, s)| b){
                
                    
            }

}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 12;
    type Input1 = Vec<(SpringVec, Vec<usize>)>;
    type Input2 = Vec<(SpringVec, Vec<usize>)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let (spring, nums) = l.split_once(' ').unwrap();
                let spring = spring
                    .chars()
                    .map(|c| match c {
                        '?' => Spring::Unknown,
                        '#' => Spring::Broken,
                        '.' => Spring::Ok,
                        _ => panic!(),
                    })
                    .collect();

                let nums = nums.split(',').map(|c| c.parse().unwrap()).collect();
                (SpringVec(spring), nums)
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
            .iter()
            .map(|l| {
                let mut new = vec![];
                for _ in 0..4 {
                    new.extend(l.0 .0.iter().copied());
                    new.push(Spring::Unknown);
                }
                new.extend(l.0 .0.iter().copied());
                (SpringVec(new), l.1.repeat(5))
            })
            .collect()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        todo!()
        // v.iter()
        //     .map(|e| {
        //         let v = e.0.unknown_bits();
        //         (0..(1 << v)).filter(|x| e.0.valid(*x as u64, &e.1)).count()
        //     })
        //     .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        use rayon::prelude::*;
        v.par_iter()
            .enumerate()
            .map(|(i, e)| {
                println!("Hi {}", i);
                e.0.find_all(&e.1)
            })
            .sum()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [
        (foo_sol1, "???.### 1,1,3", 1),
        (foo2_sol1, ".??..??...?##. 1,1,3", 4)
    ],
    [(foo_sol2, "???.### 1,1,3", 1)]
);
