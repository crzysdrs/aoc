use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input1 = Vec<Vec<i32>>;
    type Input2 = Vec<Vec<i32>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()).collect())
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .filter(|level| {
                let safe1 = level.windows(2).all(|x| x[0] <= x[1]);
                let safe2 = level.windows(2).all(|x| x[0] >= x[1]);

                let safe3 = level
                    .windows(2)
                    .all(|x| (1..=3).contains(&x[0].abs_diff(x[1])));

                (safe1 || safe2) && safe3
            })
            .count()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .filter(|level| {
                for i in 0..level.len() {
                    let mut level = (*level).clone();
                    level.remove(i);
                    let safe1 = level.windows(2).all(|x| x[0] <= x[1]);
                    let safe2 = level.windows(2).all(|x| x[0] >= x[1]);

                    let safe3 = level
                        .windows(2)
                        .all(|x| (1..=3).contains(&x[0].abs_diff(x[1])));

                    if (safe1 || safe2) && safe3 {
                        return true;
                    }
                }
                false
            })
            .count()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        2
    )],
    [(
        foo_sol2,
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        4
    )]
);
