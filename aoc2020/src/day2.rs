use crate::Day;
use regex::Regex;
use std::io::Result as IoResult;

pub struct Solution {}

pub struct Pw {
    min: usize,
    max: usize,
    c: char,
    pw: String,
}

impl Day for Solution {
    const DAY: u32 = 2;
    type Input = Pw;
    type Sol1 = usize;
    type Sol2 = usize;
    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let re = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

        r.lines()
            .map(|x| {
                let s = x?;
                let matches = re.captures(&s).unwrap();
                let pw = Pw {
                    min: matches[1].parse::<usize>().unwrap(),
                    max: matches[2].parse::<usize>().unwrap(),
                    c: matches[3].chars().next().unwrap(),
                    pw: matches[4].to_string(),
                };
                Ok(pw)
            })
            .collect::<IoResult<Vec<_>>>()
    }
    fn p1(pws: &[Self::Input]) -> Self::Sol1 {
        pws.iter()
            .filter(|m| {
                let count = m.pw.chars().filter(|c| *c == m.c).count();
                count >= m.min && count <= m.max
            })
            .count()
    }
    fn p2(pws: &[Self::Input]) -> Self::Sol2 {
        pws.iter()
            .filter(|m| {
                m.pw.chars()
                    .zip(1..)
                    .filter(|(_, i)| *i == m.min || *i == m.max)
                    .filter(|(c, _)| *c == m.c)
                    .count()
                    == 1
            })
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "1-3 a: abcde\n\
                     1-3 b: cdefg\n\
                     2-9 c: ccccccccc\n";

        let pws = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&pws), 2);
        assert_eq!(Solution::p2(&pws), 1);
    }
}
