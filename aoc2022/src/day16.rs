use crate::Day;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub struct Valve {
    id: usize,
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 16;
    type Input1 = Vec<Valve>;
    type Input2 = Vec<Valve>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        let re = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ((?:[A-Z]{2}, )*[A-Z]{2})",
        )
        .unwrap();

        let mut id = 0;
        s.lines()
            .map(|l| {
                let caps = re.captures(l).unwrap();
                let valve = Valve {
                    id,
                    name: caps[1].to_string(),
                    flow: caps[2].parse().unwrap(),
                    tunnels: caps[3].split(", ").map(|t| t.to_string()).collect(),
                };
                id += 1;
                valve
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        #[derive(Debug, PartialEq, Hash, Eq)]
        struct Best {
            loc: usize,
            minute: u32,
            open: u64,
            do_open: bool,
        }
        let mut best = HashMap::new();

        fn release_pressure<'a>(
            best: &mut HashMap<Best, u32>,
            v: &'a [Valve],
            minute: u32,
            loc: &'a Valve,
            do_open: bool,
            mut open: u64,
        ) -> u32 {
            if minute == 0 {
                return 0;
            } else if let Some(b) = best.get(&Best {
                minute,
                loc: loc.id,
                open,
                do_open,
            }) {
                return *b;
            }
            let orig_open = open;
            let score = if do_open && (open & (1 << loc.id)) == 0 {
                open |= 1 << loc.id;
                loc.flow * minute + release_pressure(best, v, minute - 1, loc, false, open)
            } else {
                loc.tunnels
                    .iter()
                    .map(|tunnel| {
                        let new_valve = v.iter().find(|x| x.name == *tunnel).unwrap();
                        if new_valve.flow == 0 || (open & (1 << new_valve.id)) != 0 {
                            release_pressure(best, v, minute - 1, new_valve, false, open)
                        } else {
                            std::cmp::max(
                                release_pressure(best, v, minute - 1, new_valve, false, open),
                                release_pressure(best, v, minute - 1, new_valve, true, open),
                            )
                        }
                    })
                    .max()
                    .unwrap()
            };

            let new_score = score;
            let new_best = Best {
                minute,
                loc: loc.id,
                open: orig_open,
                do_open,
            };
            //println!("{:?} {}", new_best, new_score);
            assert!(best.get(&new_best).is_none());
            best.insert(new_best, new_score);
            new_score
        }

        let total = std::cmp::max(
            release_pressure(&mut best, v, 29, &v[0], true, 0),
            release_pressure(&mut best, v, 29, &v[0], false, 0),
        );
        total
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        #[derive(Debug, PartialEq, Hash, Eq)]
        struct Best {
            loc: u8,
            el_loc: u8,
            minute: u8,
            open: u64,
        }
        let mut best = HashMap::new();
        let mut highest_score = 0;

        fn release_pressure<'a>(
            highest: &mut u32,
            best: &mut HashMap<Best, u32>,
            v: &'a [Valve],
            minute: u32,
            loc: &'a Valve,
            el_loc: &'a Valve,
            open: u64,
        ) -> u32 {
            let search = Best {
                minute: minute as u8,
                loc: std::cmp::min(loc.id, el_loc.id) as u8,
                el_loc: std::cmp::max(loc.id, el_loc.id) as u8,
                open,
            };
            if minute == 0 {
                return 0;
            } else if let Some(b) = best.get(&search) {
                return *b;
            } else if v.iter().all(|v| open & (1 << v.id) != 0 || v.flow == 0) {
                return 0;
            }

            #[derive(Clone)]
            enum Move {
                Open,
                Tunnel(usize),
            }
            fn moves<'a>(
                v: &'a [Valve],
                loc: &'a Valve,
                open: u64,
            ) -> impl Iterator<Item = Move> + Clone + 'a {
                loc.tunnels
                    .iter()
                    .map(|tunnel| {
                        let new_valve = v.iter().find(|x| x.name == *tunnel).unwrap();
                        Move::Tunnel(new_valve.id)
                    })
                    .chain(std::iter::once(Move::Open).flat_map(move |o| {
                        if loc.flow == 0 || (open & (1 << loc.id)) != 0 {
                            None
                        } else {
                            Some(o)
                        }
                    }))
            }
            let human = moves(v, loc, open);
            let elephant = moves(v, el_loc, open);
            use itertools::Itertools;
            let score = human
                .cartesian_product(elephant)
                .scan(0, |state, (human, elephant)| {
                    let mut open = open;
                    let mut score = 0;
                    let hum_pos = match human {
                        Move::Open => {
                            if (open & (1 << loc.id)) == 0 {
                                open |= 1 << loc.id;
                                score += loc.flow * minute;
                            }
                            loc
                        }
                        Move::Tunnel(u) => &v[u],
                    };

                    let el_pos = match elephant {
                        Move::Open => {
                            if (open & (1 << el_loc.id)) == 0 {
                                open |= 1 << el_loc.id;
                                score += el_loc.flow * minute;
                            }
                            el_loc
                        }
                        Move::Tunnel(u) => &v[u],
                    };

                    *state = std::cmp::max(
                        *state,
                        score
                            + release_pressure(highest, best, v, minute - 1, hum_pos, el_pos, open),
                    );
                    Some(*state)
                })
                .max()
                .unwrap_or(0);

            //println!("{:?} {}", new_best, new_score);
            assert!(best.get(&search).is_none());
            best.insert(search, score);
            score
        }

        let total = release_pressure(&mut highest_score, &mut best, v, 25, &v[0], &v[0], 0);
        total
    }
}

//crate::default_tests!((), ());
crate::path_tests!(
    [(t1, "test/day16.txt", 1651)],
    [(t2, "test/day16.txt", 1707)]
);
