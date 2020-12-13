use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}

#[derive(Copy, Clone, Debug)]
pub struct Bus(usize);

impl Day for Solution {
    const DAY: u32 = 13;
    type Input = (usize, Option<Bus>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines();
        let timestamp = lines
            .next()
            .map(|x| {
                let x: String = x.unwrap();
                x.parse::<usize>().unwrap()
            })
            .unwrap();

        let second = lines
            .next()
            .map(|x| {
                x.unwrap()
                    .split(',')
                    .map(|b| {
                        if b == "x" {
                            None
                        } else {
                            Some(Bus(b.parse::<usize>().unwrap()))
                        }
                    })
                    .map(|b| (timestamp, b))
                    .collect::<Vec<_>>()
            })
            .unwrap();

        Ok(second)
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let timestamp = v[0].0;
        let best = v
            .iter()
            .flat_map(|(_, b)| b)
            .map(|b| {
                let b_time = b.0
                    * if timestamp % b.0 == 0 {
                        timestamp / b.0
                    } else {
                        timestamp / b.0 + 1
                    };
                (b, b_time)
            })
            .min_by_key(|(_, b_time)| *b_time)
            .unwrap();

        best.0 .0 * (best.1 - timestamp)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let buses = v
            .iter()
            .map(|(_, b)| b)
            .enumerate()
            .flat_map(|(i, b)| b.map(|b| (i, b)))
            .collect::<Vec<_>>();

        let mut sort_buses = buses.clone();
        /* arrange busses by modulo, descending */
        sort_buses.sort_by_key(|(_, b)| b.0);
        sort_buses.reverse();

        /* renumber their bus index to be when they should arrive */
        sort_buses
            .iter_mut()
            .for_each(|(i, b)| *i = (*i * b.0 - *i) % b.0);

        /* chinese remainder theorem, sieve method */
        let (timestamp, _) = sort_buses.iter().skip(1).fold(sort_buses[0], |b0, b1| {
            let x_n = (0usize..)
                .filter_map(|c| {
                    let v = b0.0 + c * b0.1 .0;
                    if (v % b1.1 .0) == b1.0 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .nth(0)
                .unwrap();
            (x_n, Bus(b0.1 .0 * b1.1 .0))
        });

        timestamp
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "0
17,x,13,19";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 3417);
        let s = "0
67,7,59,61";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 754018);

        let s = "0
67,x,7,59,61";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 779210);

        let s = "0
67,7,x,59,61";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 1261476);

        let s = "0
1789,37,47,1889";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 1202161486);
    }
}
