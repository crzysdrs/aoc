use crate::Day;
use petgraph::graph::{DiGraph, NodeIndex};
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input1 = (NodeIndex, DiGraph<(i32, i32), ()>);
    type Input2 = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut nodes = HashMap::<(i32, i32), NodeIndex>::default();
        let mut g = DiGraph::default();

        let mut start = None;
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().for_each(|(x, c)| {
                    if c == 'S' {
                        let x = x as i32;
                        let y = y as i32;

                        start = Some((x, y));
                    }
                });

                l.chars().enumerate().flat_map(move |(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    match c {
                        '|' => vec![((x, y), (x, y - 1)), ((x, y), (x, y + 1))],
                        '-' => vec![((x, y), (x - 1, y)), ((x, y), (x + 1, y))],
                        'L' => vec![((x, y), (x, y - 1)), ((x, y), (x + 1, y))],
                        'J' => vec![((x, y), (x, y - 1)), ((x, y), (x - 1, y))],
                        '7' => vec![((x, y), (x, y + 1)), ((x, y), (x - 1, y))],
                        'F' => vec![((x, y), (x, y + 1)), ((x, y), (x + 1, y))],
                        '.' => vec![],
                        'S' => {
                            // vec![
                            //     ((x, y), (x + 1, y)),
                            //     ((x, y), (x - 1, y)),
                            //     ((x, y), (x, y - 1)),
                            //     ((x, y), (x, y + 1)),
                            // ]
                            vec![]
                        }
                        _ => panic!(),
                    }
                })
            })
            .for_each(|((x1, y1), (x2, y2))| {
                let n1 = *nodes
                    .entry((x1, y1))
                    .or_insert_with(|| g.add_node((x1, y1)));
                let n2 = *nodes
                    .entry((x2, y2))
                    .or_insert_with(|| g.add_node((x2, y2)));

                g.add_edge(n1, n2, ());
            });

        let start = *nodes
            .entry(start.unwrap())
            .or_insert_with(|| g.add_node(start.unwrap()));
        use petgraph::visit::EdgeRef;
        let incoming: Vec<_> = g
            .edges_directed(start, petgraph::Direction::Incoming)
            .map(|e| e.source())
            .collect();
        assert_eq!(incoming.len(), 2);

        for s in incoming {
            g.add_edge(start, s, ());
        }
        (start, g)
    }
    fn process_input2(_s: &str) -> Self::Input2 {
        unimplemented!()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        petgraph::algo::dijkstra(&v.1, v.0, None, |_| 1)
            .values()
            .max()
            .copied()
            .unwrap()
    }
    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        unimplemented!()
    }
}

crate::default_tests!(6812, 0);
crate::string_tests!(
    [
        (
            foo_sol1,
            ".....
.S-7.
.|.|.
.L-J.
.....",
            4
        ),
        (
            foo2_sol1,
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
            4
        )
    ],
    [(foo_sol2, "hi2", 1)]
);
