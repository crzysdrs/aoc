use crate::Day;
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Color(String, String);
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Bag {
    color: Color,
}
#[derive(Debug)]
pub struct Rule {
    bag: Bag,
    contains: Vec<(usize, Bag)>,
}
use regex::Regex;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input = Rule;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let bag = Regex::new(r"(?:(\d+) )?(\S+) (\S+) bags?").unwrap();
        r.lines()
            .map(|l| {
                let l = l?;
                let mut bags = bag
                    .captures_iter(&l)
                    .filter(|b| b[2].to_string() != "no")
                    .map(|b| {
                        (
                            b.get(1).map(|n| n.as_str().parse::<usize>().unwrap()),
                            Bag {
                                color: Color(b[2].to_string(), b[3].to_string()),
                            },
                        )
                    });
                let first = bags.next().unwrap();
                let rest = bags.map(|(i, b)| (i.unwrap(), b)).collect::<Vec<_>>();

                Ok(Rule {
                    bag: first.1,
                    contains: rest,
                })
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let target = Bag {
            color: Color("shiny".to_string(), "gold".to_string()),
        };

        let mut found = HashSet::new();
        found.insert(&target);
        loop {
            let old_found = found.clone();
            found.extend(
                v.iter()
                    .map(|x| {
                        (
                            &x.bag,
                            x.contains
                                .iter()
                                .map(|(_, x)| old_found.contains(x))
                                .any(|x| x),
                        )
                    })
                    .filter(|(_, b)| *b)
                    .map(|(r, _)| r),
            );
            if old_found.len() == found.len() {
                break;
            }
        }

        found.remove(&target); /* a bag cannot contain itself */

        found.len()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let target = Bag {
            color: Color("shiny".to_string(), "gold".to_string()),
        };

        let mut worklist = vec![(1, &target)];
        let mut total = 0;
        while let Some(w) = worklist.pop() {
            total += w.0;
            let rule = v.iter().find(|r| r.bag == *w.1).unwrap();
            worklist.extend(rule.contains.iter().map(|(i, b)| (i * w.0, b)));
        }
        total - 1 /* remove the first stupid bag */
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 4);

        let s = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 126);
    }
}
