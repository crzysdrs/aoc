use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = Vec<(usize, usize)>;
    type Input2 = Vec<(usize, usize)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let time = lines.by_ref().next().unwrap();
        let dist = lines.by_ref().next().unwrap();

        time.split_ascii_whitespace()
            .zip(dist.split_ascii_whitespace())
            .skip(1)
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        let mut lines = s.lines();
        let time = lines.by_ref().next().unwrap();
        let time = time.strip_prefix("Time:").unwrap();
        let dist = lines.by_ref().next().unwrap();
        let dist = dist.strip_prefix("Distance:").unwrap();
        let time = time
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse()
            .unwrap();
        let dist = dist
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse()
            .unwrap();
        vec![(time, dist)]
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|(time, dist)| (0..*time).filter(|t| t * (*time - t) > *dist).count())
            .product()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .map(|(time, dist)| (0..*time).filter(|t| t * (*time - t) > *dist).count())
            .product()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "Time:      7  15   30
Distance:  9  40  200",
        288
    )],
    [(
        foo_sol2,
        "Time:      7  15   30
Distance:  9  40  200",
        71503
    )]
);
