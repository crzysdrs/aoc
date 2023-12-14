use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
pub enum Spring {
    Unknown,
    Broken,
    Ok,
}

#[derive(Debug)]
pub struct SpringVec(Vec<Spring>);

#[derive(Hash, PartialEq, Eq)]
struct Key<'a> {
    spring: &'a [Spring],
    broken: &'a [usize],
}

fn section_match(spring1: &[Spring], count: usize) -> Option<&[Spring]> {
    if spring1.len() < count {
        return None;
    }

    let mut test = spring1.iter();

    if test.by_ref().take(count).all(|s| match s {
        Spring::Broken | Spring::Unknown => true,
        Spring::Ok => false,
    }) && test.by_ref().take(1).all(|s| match s {
        Spring::Ok | Spring::Unknown => true,
        Spring::Broken => false,
    }) {
        Some(test.as_slice())
    } else {
        None
    }
}

fn valid_count<'a>(
    seen: &mut HashMap<Key<'a>, usize>,
    spring: &'a [Spring],
    broken: &'a [usize],
) -> usize {
    //println!("{:?} {:?}", spring, broken);
    let key = Key { spring, broken };
    if let Some(v) = seen.get(&key) {
        return *v;
    } else if spring.is_empty() && !broken.is_empty() {
        return 0;
    } else if spring.is_empty() && broken.is_empty() {
        return 1;
    }

    let count = match spring[0] {
        Spring::Ok => valid_count(seen, &spring[1..], broken),
        Spring::Unknown if broken.is_empty() => valid_count(seen, &spring[1..], broken),
        Spring::Broken if broken.is_empty() => 0,
        Spring::Broken => {
            if let Some(rest) = section_match(spring, broken[0]) {
                valid_count(seen, rest, &broken[1..])
            } else {
                0
            }
        }
        Spring::Unknown => {
            valid_count(seen, &spring[1..], broken)
                + if let Some(rest) = section_match(spring, broken[0]) {
                    valid_count(seen, rest, &broken[1..])
                } else {
                    0
                }
        }
    };
    seen.insert(key, count);
    count
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
        v.iter()
            .map(|e| {
                let mut hash = HashMap::default();
                valid_count(&mut hash, &e.0 .0, &e.1)
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        use rayon::prelude::*;
        v.par_iter()
            .map(|e| {
                let mut hash = HashMap::default();
                valid_count(&mut hash, &e.0 .0, &e.1)
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
