use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

fn transform(mut v: usize, sub_num: usize) -> usize {
    v *= sub_num;
    v.rem_euclid(20201227)
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 25;
    type Input = usize;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines().flatten().map(|l| l.parse().unwrap()).collect())
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut loops = v
            .iter()
            .map(|sub| {
                (0..)
                    .scan(1, |state, _| {
                        *state = transform(*state, 7);
                        Some(*state)
                    })
                    .position(|v| v == *sub)
                    .unwrap()
                    + 1
            })
            .collect::<Vec<_>>();

        loops.reverse();
        let keys = v
            .iter()
            .zip(loops.iter())
            .map(|(sub, loop_num)| {
                let mut v = 1;
                for _ in 0..*loop_num {
                    v = transform(v, *sub)
                }
                v
            })
            .collect::<Vec<_>>();

        assert_eq!(keys[0], keys[1]);
        keys[0]
    }
    fn p2(_v: &[Self::Input]) -> Self::Sol2 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "5764801
17807724";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 14897079);
    }
}
