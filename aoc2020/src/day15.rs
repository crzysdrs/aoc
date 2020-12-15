use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

struct Spoken {
    prev : (usize, usize),
}

fn compute(v: &[usize], target: usize) -> usize {
    let mut spoken :HashMap<usize, Spoken> = HashMap::new();
    spoken.extend(v.iter().enumerate().map(|(i, x)| (*x, Spoken{prev: (i, 0)})));
    let last = *v.iter().last().unwrap();
    (spoken.len()..target).scan((last, true), |last, turn| {
        let next = if last.1 {
            0
        } else if let Some(Spoken { prev: (old, older) }) = spoken.get(&last.0) {
            old - older
        } else {
            unreachable!()
        };
        let mut first = false;
        spoken.entry(next).and_modify(|x| {
            first = false;
            std::mem::swap(&mut x.prev.0, &mut x.prev.1);
            x.prev.0 = turn;
        }).or_insert_with(|| {
            first = true;
            Spoken {prev: (turn, 0)}
        });
        *last = (next, first);
        Some(next)
    }).last().unwrap()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 15;
    type Input = usize;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let line = r.lines().next().unwrap()?;
        Ok(line.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        compute(v, 2020)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        compute(v, 30_000_000)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "0,3,6";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v),436);
        /* commented because p2 is slow */
        //  assert_eq!(Solution::p2(&v),175594);
    }
}
