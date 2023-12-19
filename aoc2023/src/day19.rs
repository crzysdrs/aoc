use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Rate {
    X,
    M,
    A,
    S,
}

impl std::str::FromStr for Rate {
    type Err = ();
    fn from_str(l: &str) -> Result<Rate, ()> {
        match l {
            "x" => Ok(Rate::X),
            "m" => Ok(Rate::M),
            "a" => Ok(Rate::A),
            "s" => Ok(Rate::S),
            _ => panic!(),
        }
    }
}
#[derive(Debug)]
pub struct Cond {
    rate: Rate,
    gt: bool,
    val: usize,
}

#[derive(Debug)]
pub struct Rule {
    cond: Option<Cond>,
    target: String,
}

impl Rule {
    fn eval(&self, h: &HashMap<Rate, usize>) -> bool {
        match &self.cond {
            Some(c) => {
                let r = h.get(&c.rate).unwrap();
                let meet = if c.gt { *r > c.val } else { *r < c.val };
                meet
            }
            None => true,
        }
    }
}
#[derive(Debug)]
pub struct NamedRule {
    _name: String,
    rules: Vec<Rule>,
}

impl NamedRule {
    fn eval(&self, h: &HashMap<Rate, usize>) -> String {
        self.rules
            .iter()
            .find_map(|r| {
                if r.eval(h) {
                    Some(r.target.clone())
                } else {
                    None
                }
            })
            .unwrap()
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 19;
    type Input1 = (HashMap<String, NamedRule>, Vec<HashMap<Rate, usize>>);
    type Input2 = (HashMap<String, NamedRule>, Vec<HashMap<Rate, usize>>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let named_rules: HashMap<String, NamedRule> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (prefix, suffix) = l.split_once('{').unwrap();
                let suffix = &suffix[..suffix.len() - 1];
                let rules = suffix
                    .split(',')
                    .map(|rule| {
                        let (cond, target) = if rule.contains(':') {
                            let (cond, out) = rule.split_once(':').unwrap();
                            let op = if cond.contains('>') { '>' } else { '<' };
                            let (l, r) = cond.split_once(op).unwrap();
                            (
                                Some(Cond {
                                    gt: op == '>',
                                    val: r.parse().unwrap(),
                                    rate: l.parse().unwrap(),
                                }),
                                out.to_string(),
                            )
                        } else {
                            (None, rule.to_string())
                        };
                        Rule { cond, target }
                    })
                    .collect();
                (
                    prefix.to_string(),
                    NamedRule {
                        _name: prefix.to_string(),
                        rules,
                    },
                )
            })
            .collect();

        let starts: Vec<_> = lines
            .by_ref()
            .map(|l| {
                let l = &l[1..l.len() - 1];
                let m: HashMap<Rate, usize> = l
                    .split(',')
                    .map(|v| {
                        let (r, v) = v.split_once('=').unwrap();
                        (r.parse().unwrap(), v.parse().unwrap())
                    })
                    .collect();
                m
            })
            .collect();

        (named_rules, starts)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        //println!("{:?}", v);
        v.1.iter()
            .map(|s| {
                use std::borrow::Cow;
                let mut state: Cow<str> = "in".into();
                while !["A".into(), "R".into()].contains(&state) {
                    let new_state = v.0.get(state.as_ref()).unwrap().eval(&s).into();
                    //println!("{} -> {}", state, new_state);
                    state = new_state;
                }

                if *"A" == state {
                    s.values().sum()
                } else {
                    0
                }
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        use std::ops::Range;
        let m: HashMap<_, _> = [
            (Rate::X, 1..4001),
            (Rate::M, 1..4001),
            (Rate::A, 1..4001),
            (Rate::S, 1..4001),
        ]
        .into_iter()
        .collect();

        let mut seen: HashSet<(String, Vec<(Rate, Range<usize>)>)> = HashSet::new();

        let mut worklist = vec![("in".to_string(), m)];

        let mut accepted = vec![];

        while let Some((state, h)) = worklist.pop() {
            if state == "R" {
                continue;
            } else if state == "A" {
                accepted.push(h);
                continue;
            }
            let rule = v.0.get(&state).unwrap();
            let mut h_vec: Vec<_> = h.iter().map(|(k, v)| (*k, v.clone())).collect();
            h_vec.sort_by_key(|h| h.0);

            let key = (state, h_vec);
            if seen.get(&key).is_some() {
                continue;
            }
            seen.insert(key);

            worklist.extend(rule.rules.iter().scan(h, |h, rule| match &rule.cond {
                Some(c) => {
                    let r = h.get(&c.rate).unwrap();
                    if r.contains(&c.val) {
                        if c.gt {
                            let mid = std::cmp::min(c.val + 1, r.end);
                            let prefix = r.start..mid;
                            let suffix = mid..r.end;

                            let mut new = (*h).clone();
                            new.insert(c.rate, suffix);
                            h.insert(c.rate, prefix);
                            Some((rule.target.clone(), new))
                        } else {
                            let mid = std::cmp::max(c.val, r.start);
                            let prefix = r.start..mid;
                            let suffix = mid..r.end;

                            let mut new = (*h).clone();
                            new.insert(c.rate, prefix);
                            h.insert(c.rate, suffix);
                            Some((rule.target.clone(), new))
                        }
                    } else if if c.gt {
                        r.end <= c.val
                    } else {
                        r.start > c.val
                    } {
                        None
                    } else {
                        Some((rule.target.clone(), (*h).clone()))
                    }
                }
                None => Some((rule.target.clone(), (*h).clone())),
            }));
        }

        for a1 in &accepted {
            for a2 in &accepted {
                if a1 == a2 {
                    continue;
                }
                assert!(!a1
                    .iter()
                    .all(|(k, v)| { range_overlap(a2.get(k).unwrap(), v) }));
            }
        }

        fn range_overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
            b.contains(&a.start) || a.contains(&b.start)
        }

        println!("{:#?}", accepted);
        accepted
            .iter()
            .map(|a| a.values().map(|r| r.len()).product::<usize>())
            .sum()
    }
}

crate::default_tests!(449531, 122756210763577);
crate::string_tests!(
    [(
        foo_sol1,
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        19114
    )],
    [(
        foo_sol2,
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        167409079868000
    )]
);
