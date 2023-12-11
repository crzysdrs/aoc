use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    L,
    R,
}
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Turn {
    l: usize,
    r: usize,
}

#[derive(Debug)]
pub struct Network {
    cmd: Vec<Dir>,
    names: HashMap<String, usize>,
    path: HashMap<usize, Turn>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 8;
    type Input1 = Network;
    type Input2 = Network;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let lr = lines.by_ref().next().unwrap();

        let dirs = lr
            .chars()
            .map(|lr| if lr == 'L' { Dir::L } else { Dir::R })
            .collect();

        let mut paths = HashMap::default();

        let mut hs: HashMap<String, usize> = HashMap::default();

        let mut cur = 0;
        let mut get_name = |n: &str| {
            if let Some(v) = hs.get(n) {
                *v
            } else {
                hs.insert(n.to_string(), cur);
                let v = cur;
                cur += 1;
                v
            }
        };
        let _ = lines.by_ref().next();
        while let Some(l) = lines.by_ref().next() {
            let (src, dsts) = l.split_once(" = ").unwrap();
            let (mut l, mut r) = dsts.split_once(", ").unwrap();
            l = &l[1..];
            r = &r[..r.len() - 1];

            paths.insert(
                get_name(src),
                Turn {
                    l: get_name(l),
                    r: get_name(r),
                },
            );
        }

        Network {
            cmd: dirs,
            path: paths,
            names: hs,
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        //println!("{:?}", v);
        let mut steps = 0;
        let start = *v.names.get("AAA").unwrap();
        let target = *v.names.get("ZZZ").unwrap();

        let mut cur = v.path.get(&start).unwrap();
        let dirs = v.cmd.iter().cycle();
        for c in dirs {
            steps += 1;
            let next = match c {
                Dir::L => cur.l,
                Dir::R => cur.r,
            };
            if next == target {
                break;
            }
            cur = v.path.get(&next).unwrap();
        }
        steps
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let starts: HashSet<_> = v
            .names
            .iter()
            .filter(|(k, _v)| k.ends_with('A'))
            .map(|(_k, v)| v)
            .copied()
            .collect();
        let targets: HashSet<_> = v
            .names
            .iter()
            .filter(|(k, _v)| k.ends_with('Z'))
            .map(|(_k, v)| v)
            .copied()
            .collect();

        let mut loops = vec![];
        for start in starts {
            let dirs = v.cmd.iter().enumerate().cycle();
            let mut seen: HashMap<(usize, usize), usize> = HashMap::default();
            let mut cur = start;

            for (steps, (i, c)) in dirs.enumerate() {
                if let Some(last) = seen.get(&(i, cur)) {
                    println!("{:?}", (steps, i, cur, last));
                    break;
                }
                seen.insert((i, cur), steps);

                let cur_turn = v.path.get(&cur).unwrap();
                let next = match c {
                    Dir::L => cur_turn.l,
                    Dir::R => cur_turn.r,
                };
                cur = next;
            }

            let end = targets
                .iter()
                .flat_map(|e| {
                    seen.iter()
                        .filter(|(k, _v)| k.1 == *e)
                        .max_by_key(|(_k, v)| *v)
                })
                .next()
                .unwrap();
            loops.push(*end.1)
        }

        lcm(&loops)
    }
}

fn lcm(items: &[usize]) -> usize {
    items.iter().fold(1, |state, v| lcm2(state, *v))
}

fn lcm2(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

crate::default_tests!(12643, 13133452426987);
crate::string_tests!(
    [(
        foo_sol1,
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    )],
    [(
        foo_sol2,
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        6
    )]
);
