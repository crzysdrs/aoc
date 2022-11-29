use crate::Day;
use itertools::Itertools;
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input = (usize, HashMap<char, usize>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines()
            .map(|l| Result::<_, std::io::Error>::Ok(l?.chars().collect()))
            .flatten()
            .group_by(|h: &HashSet<_>| h.len() > 0)
            .into_iter()
            .filter(|(k, _v)| *k)
            .map(|(_k, v)| {
                v.into_iter().fold((0, HashMap::new()), |mut state, x| {
                    for i in x.iter() {
                        state.1.entry(*i).and_modify(|x| *x = *x + 1).or_insert(1);
                    }
                    state.0 += 1;
                    state
                })
            })
            .collect())
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        v.iter().map(|(_, x)| x.keys().count()).sum()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        v.iter()
            .map(|(total, x)| x.values().filter(|x| *x == total).count())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 11);

        let s = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&input), 6);
    }
}
