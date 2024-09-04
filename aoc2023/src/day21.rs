use crate::Day;
use cgmath::{Point2, Vector2};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::binary_heap::BinaryHeap;

#[allow(unused_imports)]
use std::collections::*;

const UP: Vector2<i32> = Vector2::new(0, 1);
const DOWN: Vector2<i32> = Vector2::new(0, -1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);

pub struct Solution {}

fn p1_generic(v: &(NodeIndex<u32>, DiGraph<Point2<i32>, ()>), num_steps: usize) -> usize {
    petgraph::algo::dijkstra(&v.1, v.0, None, |_| 1)
        .values()
        .filter(|v| **v <= num_steps && **v % 2 == 0)
        .count()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct FrontierNode {
    dist: usize,
    point: Point2<i32>,
}

impl Ord for FrontierNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for FrontierNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn p2_generic2(v: &(NodeIndex<u32>, DiGraph<Point2<i32>, ()>), num_steps: usize) -> usize {
    let old_g = &v.1;
    let pts: HashSet<_> = v.1.node_weights().collect();
    let s = v.0;
    let start = *old_g.node_weight(s).unwrap();
    let mut frontier = BinaryHeap::<FrontierNode>::new();
    let mut next_frontier = BinaryHeap::<FrontierNode>::new();
    frontier.push(FrontierNode {
        dist: 0,
        point: start,
    });
    let mut dist: HashMap<Point2<i32>, usize> = HashMap::new();

    let max_x = old_g.node_weights().map(|p| p.x).max().unwrap();
    let max_rows = (max_x + 1) as usize;

    let half = max_rows / 2;
    let frontier_increase = max_rows;
    //let frontier_mod = (frontier_increase + half) % 2;
    let mut frontiers: Vec<_> = (0..10).map(|c| half + c * frontier_increase).collect();

    if max_rows < 50 /* example test case doesn't follow this rule */ || (num_steps - half) % max_rows != 0
    {
        frontiers = vec![num_steps];
    }
    let mut y = vec![];
    for frontier_dist in frontiers {
        while let Some(frontier_node) = frontier.pop() {
            if dist.contains_key(&frontier_node.point) {
                continue;
            }
            dist.insert(frontier_node.point, frontier_node.dist);

            for d in &[UP, LEFT, RIGHT, DOWN] {
                let next = frontier_node.point + d;
                let modulo_next = Point2::new(
                    next.x.rem_euclid(max_rows as i32),
                    next.y.rem_euclid(max_rows as i32),
                );
                //println!("{} {:?} {:?}", cur + 1, next, modulo_next);
                if dist.contains_key(&next) {
                    // do nothing
                } else if pts.contains(&modulo_next) {
                    let new = FrontierNode {
                        dist: frontier_node.dist + 1,
                        point: next,
                    };
                    if new.dist <= frontier_dist {
                        frontier.push(new);
                    } else {
                        next_frontier.push(new);
                    }
                }
            }
        }
        let count = dist
            .values()
            .filter(|v| *v % 2 == frontier_dist % 2)
            .count();
        println!("{:?} => {}", frontier_dist, count,);
        std::mem::swap(&mut frontier, &mut next_frontier);
        y.push(count as isize);
    }
    if y.len() == 1 {
        return y[0] as usize;
    }
    // If we assume y = ax^2 + bx + c;
    // y(0) = c
    // y(1) = a + b + c
    // y(2) = 4a + 2b + c;

    // c = y(0)
    // b = y(1) - a - c => y(1) - a - y(0)
    // a = (y(2) - 2b - c) / 4

    // b = y(1) - ((y(2) - 2b - c) / 4) - y(0)
    // => 4b = 4y(1) - y(2) + 2b + y(0) - 4y(0)
    // => 2b = 4y(1) - y(2) -3y(0)
    // => b = 2y(1) - y(2) / 2 - 3y(0) / 2

    // a = y(1) - b - y(0)

    let c = y[0];
    let b = (4 * y[1] - y[2] - 3 * y[0]) / 2;
    let a = y[1] - b - y[0];

    assert_eq!((num_steps - half) % frontier_increase, 0);
    assert_eq!(y[0], c);
    assert_eq!(y[1], a + b + c);
    assert_eq!(y[2], 4 * a + 2 * b + c);

    for (x, y) in y.iter().enumerate() {
        let x = x as isize;
        assert_eq!(*y, a * x * x + b * x + c);
    }
    let x = ((num_steps - half) / frontier_increase) as isize;
    (a * x * x + b * x + c).try_into().unwrap()
}

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
        p1_generic(v, 64)
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        p2_generic2(v, 26501365)
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let input = <Solution as Day>::process_input1(input);
        assert_eq!(p1_generic(&input, 6), 16);
    }

    #[test]
    fn part2_2() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let input = <Solution as Day>::process_input2(input);

        //assert_eq!(p2_generic2(&input, 6), 16);

        assert_eq!(p2_generic2(&input, 5), 13);
        assert_eq!(p2_generic2(&input, 16), 129);
        assert_eq!(p2_generic2(&input, 27), 427);
        assert_eq!(p2_generic2(&input, 38), 894);
        assert_eq!(p2_generic2(&input, 49), 1528);
        assert_eq!(p2_generic2(&input, 55), 1914);
    }
    #[test]
    fn part2() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let input = <Solution as Day>::process_input2(input);
        assert_eq!(p2_generic2(&input, 6), 16);
        assert_eq!(p2_generic2(&input, 10), 50);
        assert_eq!(p2_generic2(&input, 50), 1594);
        assert_eq!(p2_generic2(&input, 100), 6536);
        assert_eq!(p2_generic2(&input, 500), 167004);
        assert_eq!(p2_generic2(&input, 1000), 668697);
        assert_eq!(p2_generic2(&input, 5000), 16733044);
    }
}
crate::default_tests!(3782, 630661863455116);
