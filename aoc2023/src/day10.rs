use crate::Day;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input1 = ((i32, i32), NodeIndex, DiGraph<(i32, i32), ()>);
    type Input2 = ((i32, i32), NodeIndex, DiGraph<(i32, i32), ()>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut nodes = HashMap::<(i32, i32), NodeIndex>::default();
        let mut g = DiGraph::default();
        let mut max_x = None;
        let mut max_y = None;

        let mut start = None;
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                max_y = Some(std::cmp::max(max_y.unwrap_or(0), y as i32));
                l.chars().enumerate().for_each(|(x, c)| {
                    if c == 'S' {
                        let x = x as i32;
                        let y = y as i32;

                        start = Some((x, y));
                    }
                });
                max_x = Some(std::cmp::max(
                    max_x.unwrap_or(0),
                    l.chars().count() as i32 - 1,
                ));

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
                        '.' => vec![((x, y), (x, y))],
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

        let incoming: Vec<_> = g
            .edges_directed(start, petgraph::Direction::Incoming)
            .map(|e| e.source())
            .collect();
        assert_eq!(incoming.len(), 2);

        for s in incoming {
            g.add_edge(start, s, ());
        }
        ((max_x.unwrap(), max_y.unwrap()), start, g)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        petgraph::algo::dijkstra(&v.2, v.1, None, |_| 1)
            .values()
            .max()
            .copied()
            .unwrap()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        // 1424 Too High
        // 1473 Too High
        let all_nodes: HashMap<_, _> = v.2.node_weights().zip(v.2.node_indices()).collect();
        let path: HashSet<_> = petgraph::algo::dijkstra(&v.2, v.1, None, |_| 1)
            .keys()
            .copied()
            .collect();

        let mut outside = 0;
        let mut inside = 0;

        for x in 0..=v.0 .0 {
            let mut intersect = 0;
            for y in 0..=v.0 .1 {
                let n = all_nodes.get(&(x, y)).unwrap();
                let on_path = path.contains(n);
                if on_path {
                    let rightward =
                        v.2.edges_directed(*n, petgraph::Outgoing)
                            .any(|e| v.2.node_weight(e.target()).unwrap().0 > x);

                    if rightward {
                        intersect += 1;
                    }
                } else if (intersect % 2) == 0 {
                    outside += 1;
                } else {
                    inside += 1;
                }
            }
        }
        assert_eq!(
            outside + inside + path.len(),
            ((v.0 .0 + 1) * (v.0 .1 + 1)) as usize
        );

        inside
    }
}

crate::default_tests!(6812, 527);
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
    [
        (
            foo_sol2,
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
            4
        ),
        (
            foo2_sol2,
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
            8
        )
    ]
);
