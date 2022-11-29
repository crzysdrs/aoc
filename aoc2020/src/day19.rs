use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use regex::Regex;

pub enum Rule {
    Single(usize, Vec<Vec<usize>>),
    Unary(usize, String),
}

pub struct Input {
    rules: Vec<Rule>,
    data: Vec<String>,
}

fn build_re_string(hm: &HashMap<usize, &Rule>, rule_num: usize, part2: bool) -> String {
    if let Some(r) = hm.get(&rule_num) {
        match (part2, rule_num) {
            (true, 8) => {
                let new = build_re_string(hm, 42, part2);
                format!("(?:{}+)", new)
            }
            (true, 11) => {
                /* these modifications are not a general solution, it would require a full CFG
                to compute this rule, but we can do it pretty easily for a few arbitrarily nested
                deep levels */
                let arbitrary = 20;
                let l = build_re_string(hm, 42, part2);
                let r = build_re_string(hm, 31, part2);
                let stupid: Vec<_> = (1..arbitrary)
                    .map(|i| format!("{}{}", l.repeat(i), r.repeat(i)))
                    .collect();
                format!("(?:{})", stupid.join("|"))
            }
            _ => match r {
                Rule::Single(_, v) => {
                    let new = v
                        .iter()
                        .map(|choice| {
                            choice
                                .iter()
                                .map(|v| build_re_string(hm, *v, part2))
                                .collect::<String>()
                        })
                        .collect::<Vec<_>>();
                    let new = new.join("|");

                    format!("(?:{})", new)
                }
                Rule::Unary(_, s) => s.to_string(),
            },
        }
    } else {
        unreachable!()
    }
}

fn build_re(input: &Input, part2: bool) -> usize {
    let mut rules = input
        .rules
        .iter()
        .map(|r| match r {
            Rule::Single(n, _) => (*n, r),
            Rule::Unary(n, _) => (*n, r),
        })
        .collect::<HashMap<_, _>>();

    let rule8 = Rule::Single(8, vec![vec![42], vec![42, 8]]);
    let rule11 = Rule::Single(11, vec![vec![42, 31], vec![42, 11, 31]]);
    if part2 {
        rules.entry(8).and_modify(|x| *x = &rule8);
        rules.entry(11).and_modify(|x| *x = &rule11);
    }
    let re_string = build_re_string(&rules, 0, part2);
    let re_string = format!("^{}$", re_string);

    let re = regex::RegexBuilder::new(&re_string)
        .size_limit(1 << 28)
        .build()
        .unwrap();

    input.data.iter().filter(|x| re.is_match(x)).count()
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 19;
    type Input = Input;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let rule_num = Regex::new(r"^([0-9]+): (.*)$").unwrap();
        let string = Regex::new(r#""([a-zA-Z]+)""#).unwrap();

        let mut lines = r.lines();
        let rules = lines
            .by_ref()
            .map(|x| x.unwrap())
            .take_while(|x| !x.is_empty())
            .map(|x| {
                let v = if let Some(cap) = rule_num.captures(&x) {
                    let rule_num = cap.get(1).unwrap().as_str().parse().unwrap();
                    let rest = cap.get(2).unwrap().as_str();

                    if let Some(s) = string.captures(rest) {
                        Rule::Unary(rule_num, s.get(1).unwrap().as_str().to_string())
                    } else {
                        let rules = rest
                            .split('|')
                            .map(|r| {
                                r.trim()
                                    .split(' ')
                                    .map(|x| x.parse::<usize>().unwrap())
                                    .collect()
                            })
                            .collect();
                        Rule::Single(rule_num, rules)
                    }
                } else {
                    unreachable!("{}", x)
                };
                v
            })
            .collect::<Vec<_>>();

        let data = lines.by_ref().map(|x| x.unwrap()).collect();

        Ok(vec![Input { rules, data }])
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        build_re(&v[0], false)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let input = &v[0];

        build_re(input, true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 2);

        let s = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 12);

        //unimplemented!()
    }
}
