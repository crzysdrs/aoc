use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

fn find_pair(v: &[u64], sum: u64) -> Option<(u64, u64)> {
    if v.len() < 2 {
        return None;
    }
    let mut i = 0;
    let mut j = v.len() - 1;

    while i < j {
        use std::cmp::Ordering;
        match (v[i] + v[j]).cmp(&sum) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            _ => return Some((v[i], v[j])),
        }
    }
    None
}

pub struct Solution {}

impl Solution {
    fn p1_preamble(v: &[u64], preamble_len: usize) -> u64 {
        let mut preamble: Vec<_> = v.iter().take(preamble_len).cloned().collect();
        preamble.sort();

        v.iter()
            .enumerate()
            .skip(preamble_len)
            .map(|(i, value)| {
                /* This could be sped up by making 'preamble' a hashset, but it's fast enough */
                let found = find_pair(&preamble, *value);
                let old = v[i - preamble_len];
                let idx = preamble.binary_search(&old).unwrap();
                preamble.remove(idx);
                let new_idx = match preamble.binary_search(value) {
                    Ok(n) => n,
                    Err(n) => n,
                };
                preamble.insert(new_idx, *value);
                (i, value, found.is_some())
            })
            .find(|(_, _, f)| !(*f))
            .map(|(_, v, _)| *v)
            .unwrap()
    }
    fn p2_preamble(v: &[u64], preamble_len: usize) -> u64 {
        let invalid = Self::p1_preamble(v, preamble_len);

        let mut range = 0..0;

        loop {
            let sum: u64 = v[range.clone()].iter().sum();
            match sum.cmp(&invalid) {
                std::cmp::Ordering::Less => {
                    range.end += 1;
                }
                std::cmp::Ordering::Greater => {
                    range.start += 1;
                }
                _ => break,
            }
        }
        v[range.clone()].iter().min().unwrap() + v[range].iter().max().unwrap()
    }
}
impl Day for Solution {
    const DAY: u32 = 9;
    type Input = u64;
    type Sol1 = u64;
    type Sol2 = u64;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                Ok(l.parse::<u64>().unwrap())
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        Self::p1_preamble(v, 25)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        Self::p2_preamble(v, 25)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1_preamble(&v, 5), 127);
        assert_eq!(Solution::p2_preamble(&v, 5), 62);
    }
}
