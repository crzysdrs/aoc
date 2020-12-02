use crate::Day;
use regex::Regex;
use std::io::Result as IoResult;
use std::path::Path;

pub struct Solution {}

struct Pw {
    min: usize,
    max: usize,
    c: char,
    pw: String,
}

fn parse_input(input: &str) -> Vec<Pw> {
    let re = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();

    let v = input
        .lines()
        .map(|x| {
            let matches = re.captures(x).unwrap();
            Pw {
                min: matches[1].parse::<usize>().unwrap(),
                max: matches[2].parse::<usize>().unwrap(),
                c: matches[3].chars().next().unwrap(),
                pw: matches[4].to_string(),
            }
        })
        .collect::<Vec<_>>();
    v
}

fn parse_path(path: &Path) -> IoResult<Vec<Pw>> {
    let v = parse_input(&std::fs::read_to_string(path)?);
    Ok(v)
}

fn valid1(pws: &[Pw]) -> usize {
    pws.iter()
        .filter(|m| {
            let count = m.pw.chars().filter(|c| *c == m.c).count();
            count >= m.min && count <= m.max
        })
        .count()
}

fn valid2(pws: &[Pw]) -> usize {
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
impl Day for Solution {
    const DAY: u32 = 2;
    fn p1() -> IoResult<()> {
        let v = parse_path(&Self::input())?;
        println!("Valid {}", valid1(&v));

        Ok(())
    }
    fn p2() -> IoResult<()> {
        let v = parse_path(&Self::input())?;

        println!("Valid {}", valid2(&v));

        Ok(())
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

        let pws = parse_input(&input);
        assert_eq!(valid1(&pws), 2);
        assert_eq!(valid2(&pws), 1);
    }
}
