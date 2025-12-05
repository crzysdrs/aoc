use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::ops::RangeInclusive;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 5;
    type Input1 = (Vec<RangeInclusive<usize>>, Vec<usize>);
    type Input2 = (Vec<RangeInclusive<usize>>, Vec<usize>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let ranges = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let (l, r) = s.split_once('-').unwrap();
                l.parse().unwrap()..=r.parse().unwrap()
            })
            .collect();

        let ids = lines.by_ref().map(|s| s.parse().unwrap()).collect();
        (ranges, ids)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1((ranges, ids): &Self::Input1) -> Self::Sol1 {
        ids.iter()
            .filter(|id| ranges.iter().find(|r| r.contains(id)).is_some())
            .count()
    }
    fn p2((ranges, _ids): &Self::Input2) -> Self::Sol2 {
        let mut ranges = ranges.clone();
        loop {
            let mut hit = false;
            'again: for (i, r) in ranges.iter().enumerate() {
                for (j, r2) in ranges[i + 1..].iter().enumerate() {
                    if i != j
                        && (r.contains(r2.start())
                            || r.contains(r2.end())
                            || r2.contains(r.start())
                            || r2.contains(r.end()))
                    {
                        let start = std::cmp::min(*r.start(), *r2.start());
                        let end = std::cmp::max(*r.end(), *r2.end());
                        ranges[i] = start..=end;
                        ranges.swap_remove(j + i + 1);
                        hit = true;
                        break 'again;
                    }
                }
            }

            if !hit {
                break;
            }
        }
        ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

crate::default_tests!(652, 341753674214273);
crate::string_tests!(
    [(
        foo_sol1,
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        3
    )],
    [(
        foo_sol2,
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        14
    )]
);
