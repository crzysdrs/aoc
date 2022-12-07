use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = Vec<char>;
    type Input2 = Vec<char>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.chars().collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let len = 4;
        v.windows(len)
            .enumerate().find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == w.len())
            .unwrap()
            .0
            + len
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let len = 14;
        v.windows(len)
            .enumerate().find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == w.len())
            .unwrap()
            .0
            + len
    }
}

crate::default_tests!(1582, 3588);
crate::string_tests!(
    [
        (t1, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        (t2, "nppdvjthqldpwncqszvftbrmjlhg", 6),
        (t3, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        (t4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    ],
    [
        (t5, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        (t6, "bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        (t7, "nppdvjthqldpwncqszvftbrmjlhg", 23),
        (t8, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        (t9, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    ]
);
