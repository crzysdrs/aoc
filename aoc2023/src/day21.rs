use crate::Day;
use cgmath::{Point2, Vector2};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

#[allow(unused_imports)]
use std::collections::*;

const UP: Vector2<i32> = Vector2::new(0, 1);
const DOWN: Vector2<i32> = Vector2::new(0, -1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 21;
    type Input1 = (NodeIndex<u32>, DiGraph<Point2<i32>, ()>);
    type Input2 = (NodeIndex<u32>, DiGraph<Point2<i32>, ()>);

    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut g = DiGraph::default();
        let mut max_x = None;
        let mut max_y = None;

        let mut start = None;
        let nodes: HashMap<_, _> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                max_y = Some(std::cmp::max(max_y.unwrap_or(0), y as i32));
                l.chars().enumerate().for_each(|(x, c)| {
                    if c == 'S' {
                        let x = x as i32;
                        let y = y as i32;

                        start = Some(Point2::new(x, y));
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
                        '.' | 'S' => Some(Point2::new(x, y)),
                        '#' => None,
                        _ => panic!(),
                    }
                })
            })
            .map(|v| (v, g.add_node(v)))
            .collect();

        for (n, k) in &nodes {
            for d in [UP, DOWN, LEFT, RIGHT] {
                if let Some(n2) = nodes.get(&(n + d)) {
                    g.add_edge(*k, *n2, ());
                }
            }
        }

        (*nodes.get(&start.unwrap()).unwrap(), g)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        petgraph::algo::dijkstra(&v.1, v.0, None, |_| 1)
            .values()
            .filter(|v| **v <= 64 && **v % 2 == 0)
            .count()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let old_g = &v.1;

        let s = v.0;
        let start = *old_g.node_weight(s).unwrap();

        let max_x = old_g.node_weights().map(|p| p.x).max().unwrap();
        let max_y = old_g.node_weights().map(|p| p.y).max().unwrap();

        let mut g = DiGraph::default();

        let mut nodes: HashMap<Point2<i32>, NodeIndex<u32>> = HashMap::new();

        let num_steps = 1000;
        let range = num_steps / max_x;
        for x in -range..=range {
            for y in -range..=range {
                let offset = Vector2::new(x * (max_x + 1), y * (max_y + 1));
                //println!("{:?} -> {:?}", (x, y), offset);
                nodes.extend(old_g.raw_nodes().iter().map(|n| {
                    let pos = n.weight + offset;
                    //println!("{:?}", pos);
                    (pos, g.add_node(pos))
                }));
            }
        }
        for x in -range..=range {
            for y in -range..=range {
                let offset = Vector2::new(x * (max_x + 1), y * (max_y + 1));
                //println!("{:?}, {:?}", (x, y), offset);
                old_g.raw_edges().iter().for_each(|e| {
                    g.add_edge(
                        nodes[&(*old_g.node_weight(e.source()).unwrap() + offset)],
                        nodes[&(*old_g.node_weight(e.target()).unwrap() + offset)],
                        (),
                    );
                });
                for x_edge in 0..=max_x {
                    let top_edge = Point2::new(x_edge, max_y) + offset;
                    //println!("{:?}", top_edge);
                    if let (Some(n1), Some(n2)) =
                        (nodes.get(&top_edge), nodes.get(&(top_edge + UP)))
                    {
                        //println!("Hit");
                        g.add_edge(*n1, *n2, ());
                    }

                    let bottom_edge = Point2::new(x_edge, 0) + offset;
                    //println!("{:?}", bottom_edge);
                    if let (Some(n1), Some(n2)) =
                        (nodes.get(&bottom_edge), nodes.get(&(bottom_edge + DOWN)))
                    {
                        g.add_edge(*n1, *n2, ());
                    }
                }
                for y_edge in 0..=max_y {
                    let left_edge = Point2::new(0, y_edge) + offset;
                    if let (Some(n1), Some(n2)) =
                        (nodes.get(&left_edge), nodes.get(&(left_edge + LEFT)))
                    {
                        g.add_edge(*n1, *n2, ());
                    }
                    let right_edge = Point2::new(max_x, y_edge) + offset;
                    if let (Some(n1), Some(n2)) =
                        (nodes.get(&right_edge), nodes.get(&(right_edge + RIGHT)))
                    {
                        g.add_edge(*n1, *n2, ());
                    }
                }
            }
        }
        // println!(
        //     "{:#?}",
        //     petgraph::algo::dijkstra(&g, *nodes.get(&start).unwrap(), None, |_| 1)
        // );

        petgraph::algo::dijkstra(&g, *nodes.get(&start).unwrap(), None, |_| 1)
            .values()
            .filter(|v| **v <= num_steps && **v % 2 == num_steps % 2)
            .count()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        16
    )],
    [(
        foo_sol2,
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        1
    )]
);
