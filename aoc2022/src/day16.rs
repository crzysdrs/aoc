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
#[derive(Debug, PartialEq, Hash, Eq)]
struct Best<const N: usize> {
    locs: [usize; N],
    minute: u8,
    open: u64,
}

fn release_pressure<const N: usize>(v: &[Valve], minute: u32) -> u32 {
    let mut best = HashMap::new();
    let mut highest_score = 0;

    use petgraph::Graph;
    let mut graph = Graph::new_undirected();
    let nodes: Vec<_> = v.iter().map(|_node| graph.add_node(v)).collect();
    for (valve, n) in v.iter().zip(nodes.iter()) {
        for e in &valve.tunnels {
            let e = v.iter().find(|v| v.name == *e).unwrap();
            graph.add_edge(*n, nodes[e.id], ());
        }
    }

    let mut best_path = HashMap::new();
    for valve in v {
        for valve2 in v {
            if valve.id == valve2.id {
                continue;
            }

            let path = petgraph::algo::astar(
                &graph,
                nodes[valve.id],
                |finish| finish == nodes[valve2.id],
                |_e| 1,
                |_| 0,
            )
            .unwrap();
            best_path.insert(
                (valve.id, valve2.id),
                (
                    nodes.iter().position(|id| *id == path.1[1]).unwrap(),
                    path.0,
                ),
            );
        }
    }

    let score = release_pressure_inner::<N>(
        &mut highest_score,
        &best_path,
        &mut best,
        v,
        minute,
        [0; N],
        0,
        0,
    );

    println!("{:?}", highest_score);
    score
}

fn release_pressure_inner<'a, const N: usize>(
    highest: &mut u32,
    best_path: &HashMap<(usize, usize), (usize, usize)>,
    best: &mut HashMap<Best<N>, u32>,
    v: &'a [Valve],
    minute: u32,
    mut locs: [usize; N],
    open: u64,
    cur_score: u32,
) -> u32 {
    locs.sort();
    let search = Best {
        minute: minute as u8,
        locs,
        open,
    };
    let best_possible = cur_score
        + v.iter()
            .map(|v| {
                if open & (1 << v.id) == 0 && v.flow > 0 {
                    v.flow
                        * (minute.saturating_sub(1).saturating_sub(
                            locs.iter()
                                .map(|l| best_path.get(&(*l, v.id)).map(|v| v.1).unwrap_or(0))
                                .min()
                                .unwrap_or(0) as u32,
                        ))
                } else {
                    0
                }
            })
            .sum::<u32>();

    if best_possible <= *highest {
        //println!("{} {} {} Culled", best_possible, highest, minute);
        return best_possible;
    } else if minute == 0 || v.iter().all(|v| open & (1 << v.id) != 0 || v.flow == 0) {
        *highest = std::cmp::max(*highest, cur_score);
        return cur_score;
    }

    let mut maybe_better_score = true;
    best.entry(search)
        .and_modify(|v| {
            if *v < cur_score {
                *v = cur_score;
            } else {
                maybe_better_score = false;
            }
        })
        .or_insert(cur_score);

    if !maybe_better_score {
        return cur_score;
    }

    #[derive(Debug, Clone, PartialEq, Ord, Eq, PartialOrd)]
    enum Move {
        Open,
        Tunnel(usize),
    }
    fn moves<'a>(
        v: &'a [Valve],
        loc: &'a Valve,
        open: u64,
        best_path: &HashMap<(usize, usize), (usize, usize)>,
    ) -> impl Iterator<Item = Move> + Clone + 'a {
        let mut choices: Vec<_> = v
            .iter()
            .filter(|v| open & (1 << v.id) == 0 && v.flow > 0)
            .map(|v| {
                if v.id == loc.id {
                    Move::Open
                } else {
                    Move::Tunnel(best_path.get(&(loc.id, v.id)).copied().unwrap().0)
                }
            })
            .collect();

        choices.sort();
        choices.dedup();
        choices.into_iter()
    }
    use itertools::Itertools;
    let score = locs
        .iter()
        .map(|loc| moves(v, &v[*loc], open, best_path))
        .multi_cartesian_product()
        .map(|moves| {
            let mut open = open;
            let mut score = cur_score;
            let mut new_locs = [0; N];
            for (i, (mov, loc)) in moves.iter().zip(locs.iter()).enumerate() {
                new_locs[i] = match mov {
                    Move::Open => {
                        if (open & (1 << loc)) == 0 {
                            open |= 1 << loc;
                            score += v[*loc].flow * (minute - 1);
                        }
                        *loc
                    }
                    Move::Tunnel(u) => *u,
                };
            }

            release_pressure_inner(
                highest,
                best_path,
                best,
                v,
                minute - 1,
                new_locs,
                open,
                score,
            )
        })
        .max()
        .unwrap_or(cur_score);

    score
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
        release_pressure::<1>(v, 30)
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        release_pressure::<2>(v, 26)
    }
}

crate::default_tests!(1641, 2261);
crate::path_tests!(
    [(t1, "test/day16.txt", 1651)],
    [(t2, "test/day16.txt", 1707)]
);
