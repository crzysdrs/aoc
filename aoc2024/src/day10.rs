use crate::Day;
use cgmath::{Point2, Vector2};
use petgraph::graph::DiGraph;
#[allow(unused_imports)]
use std::collections::*;

pub const LEFT: Vector2<i32> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<i32> = Vector2::new(1, 0);
pub const UP: Vector2<i32> = Vector2::new(0, 1);
pub const DOWN: Vector2<i32> = Vector2::new(0, -1);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input1 = HashMap<Point2<i32>, u32>;
    type Input2 = HashMap<Point2<i32>, u32>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter(|(_x, c)| *c != '.')
                    .map(move |(x, c)| (Point2::new(x as i32, y as i32), c.to_digit(10).unwrap()))
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut d = DiGraph::new();
        let nodes: HashMap<_, _> = v.iter().map(|(p, _v)| (p, d.add_node(*p))).collect();

        for (p, val) in v.iter() {
            let dirs = [UP, DOWN, LEFT, RIGHT];
            let p_node = nodes.get(&p).unwrap();
            for dir in dirs {
                let pt = p + dir;
                if let (Some(n), Some(v)) = (nodes.get(&pt), v.get(&pt)) {
                    if val < v && v - val == 1 {
                        d.add_edge(*p_node, *n, 1);
                    }
                }
            }
        }

        v.iter()
            .filter(|(_p, v)| **v == 0)
            .map(|(p, _v)| {
                let start = nodes.get(p).unwrap();
                let results = petgraph::algo::dijkstra::dijkstra(&d, *start, None, |_| 1);
                results
                    .iter()
                    .filter(|(g, _v)| {
                        let w = d.node_weight(**g).unwrap();
                        let w = v.get(w).unwrap();
                        *w == 9
                    })
                    .count()
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut d = DiGraph::new();
        let nodes: HashMap<_, _> = v.iter().map(|(p, _v)| (p, d.add_node(*p))).collect();

        for (p, val) in v.iter() {
            let dirs = [UP, DOWN, LEFT, RIGHT];
            let p_node = nodes.get(&p).unwrap();
            for dir in dirs {
                let pt = p + dir;
                if let (Some(n), Some(v)) = (nodes.get(&pt), v.get(&pt)) {
                    if val < v && v - val == 1 {
                        d.add_edge(*p_node, *n, 1);
                    }
                }
            }
        }

        v.iter()
            .filter(|(_p, v)| **v == 0)
            .map(|(p, _v)| {
                let start = nodes.get(p).unwrap();
                use petgraph::graph::NodeIndex;

                let mut worklist = VecDeque::from_iter([*start].into_iter());
                let mut seen = HashMap::<NodeIndex, usize>::new();
                while let Some(node) = worklist.pop_front() {
                    for v in d.neighbors(node) {
                        let path = seen.get(&node).copied().unwrap_or(1usize);
                        use std::collections::hash_map::Entry;
                        match seen.entry(v) {
                            Entry::Occupied(mut o) => {
                                *o.get_mut() += path;
                            }
                            Entry::Vacant(vacant) => {
                                worklist.push_back(v);
                                vacant.insert(path);
                            }
                        }
                    }
                }
                seen.iter()
                    .filter(|(n, _v)| *v.get(d.node_weight(**n).unwrap()).unwrap() == 9)
                    .map(|(_n, v)| *v)
                    .sum::<usize>()
            })
            .sum()
    }
}

crate::default_tests!(811, 1794);
crate::string_tests!(
    [(
        foo_sol1,
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        2
    )],
    [(
        foo_sol2,
        ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        3
    )]
);
