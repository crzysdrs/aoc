use crate::Day;
use std::io::Result as IoResult;

pub struct Solution {}

const FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
use itertools::Itertools;
use std::collections::*;

impl Day for Solution {
    const DAY: u32 = 4;
    type Input = HashMap<String, String>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let v = r
            .lines()
            .map(|l| {
                let v = l
                    .unwrap()
                    .split(' ')
                    .flat_map(|i| i.split(':'))
                    .map(|s| s.to_owned())
                    .tuples()
                    .collect::<Vec<_>>();
                Ok(v)
            })
            .collect::<IoResult<Vec<_>>>()?;

        let v = v
            .into_iter()
            .group_by(|e| e.len() > 0)
            .into_iter()
            .map(|(_k, v)| v.into_iter().flatten().collect::<HashMap<_, _>>())
            .collect();

        Ok(v)
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        v.iter()
            .map(|state| {
                FIELDS
                    .iter()
                    .map(|f| state.get(*f).is_some() || *f == "cid")
                    .all(|x| x)
            })
            .filter(|v| *v)
            .count()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        v.iter()
            .map(|state| {
                let valid = FIELDS
                    .iter()
                    .map(|f| {
                        let valid = match (*f, state.get(*f)) {
                            ("byr", Some(v)) => {
                                let v = v.parse::<u32>().unwrap();
                                (1920..=2002).contains(&v)
                            }
                            ("iyr", Some(v)) => {
                                let v = v.parse::<u32>().unwrap();
                                (2010..=2020).contains(&v)
                            }
                            ("eyr", Some(v)) => {
                                let v = v.parse::<u32>().unwrap();
                                (2020..=2030).contains(&v)
                            }
                            ("hgt", Some(v)) => {
                                if let Some(p) = v.strip_suffix("cm") {
                                    let v = p.parse::<u32>().unwrap();
                                    (150..=193).contains(&v)
                                } else if let Some(p) = v.strip_suffix("in") {
                                    let v = p.parse::<u32>().unwrap();
                                    (59..=76).contains(&v)
                                } else {
                                    false
                                }
                            }
                            ("hcl", Some(v)) => {
                                if let Some(p) = v.strip_prefix("#") {
                                    p.len() == 6
                                        && p.chars()
                                            .map(|x| {
                                                ('a'..='f').contains(&x) || ('0'..='9').contains(&x)
                                            })
                                            .all(|x| x)
                                } else {
                                    false
                                }
                            }
                            ("ecl", Some(v)) => {
                                let choices = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                                choices.contains(&v.as_str())
                            }
                            ("pid", Some(v)) => {
                                v.len() == 9
                                    && v.chars().map(|x| ('0'..='9').contains(&x)).all(|x| x)
                            }
                            ("cid", _) => true,
                            (_, _) => false,
                        };
                        valid
                    })
                    .all(|x| x);
                valid
            })
            .filter(|v| *v)
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    #[test]
    fn test() {
        let v = Solution::process_input(std::io::BufReader::new(INVALID.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 0);

        let v = Solution::process_input(std::io::BufReader::new(VALID.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 4);
        //unimplemented!()
    }
}
