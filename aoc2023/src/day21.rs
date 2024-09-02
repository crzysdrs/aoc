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
        //let seen = HashMap::new();
        //let tiles = HashMap::new();

        let old_g = &v.1;

        let s = v.0;
        let start = *old_g.node_weight(s).unwrap();

        let max_x = old_g.node_weights().map(|p| p.x).max().unwrap();
        let max_y = old_g.node_weights().map(|p| p.y).max().unwrap();

        let mut nodes: HashMap<Point2<i32>, NodeIndex<u32>> = HashMap::new();

        #[derive(Debug)]
        struct Tile {
            start: NodeIndex<u32>,
            dist: usize,
            offset: Vector2<i32>,
        }

        let mut worklist = VecDeque::from([Tile {
            start: s,
            dist: 0,
            offset: Vector2::new(0, 0),
        }]);

        let left_edge: Vec<_> = old_g.node_indices().filter(|n| old_g[*n].x == 0).collect();

        let right_edge: Vec<_> = old_g
            .node_indices()
            .filter(|n| old_g[*n].x == max_x)
            .collect();

        let top_edge: Vec<_> = old_g.node_indices().filter(|n| old_g[*n].y == 0).collect();

        let bottom_edge: Vec<_> = old_g
            .node_indices()
            .filter(|n| old_g[*n].y == max_y)
            .collect();
        let tile_size = Vector2::new(max_x + 1, max_y + 1);

        let node_map: HashMap<_, _> = old_g.node_indices().map(|n| (old_g[n], n)).collect();

        let num_steps = 10;

        let mut dist_seen = HashMap::new();
        let mut tile_seen = HashSet::new();
        let mut total = 0;
        while let Some(tile) = worklist.pop_front() {
            println!("{:?}", tile);
            if tile_seen.contains(&tile.offset) {
                println!("Seen");
                continue;
            } else {
                tile_seen.insert(tile.offset);
            }

            use std::collections::hash_map::Entry;
            //println!("{:?}", tile);
            let e = dist_seen.entry(tile.start);
            let (dist, nexts) = match e {
                Entry::Occupied(ref o) => o.get(),
                Entry::Vacant(v) => {
                    let dist: HashMap<NodeIndex<_>, _> =
                        petgraph::algo::dijkstra(&old_g, tile.start, None, |_| 1);
                    let mut nexts = vec![];
                    for (d, e) in &[
                        (LEFT, &left_edge),
                        (RIGHT, &right_edge),
                        (UP, &top_edge),
                        (DOWN, &bottom_edge),
                    ] {
                        let best_dist = e
                            .iter()
                            .map(|n| (n, dist[n]))
                            .min_by_key(|(n, w)| *w)
                            .unwrap();

                        let best_pos = old_g[*best_dist.0];
                        let best_pos = best_pos + d;
                        let best_pos = Point2::new(
                            (best_pos.x + tile_size.x) % tile_size.x,
                            (best_pos.y + tile_size.y) % tile_size.y,
                        );
                        nexts.push((node_map[&best_pos], *d, best_dist.1));
                    }
                    v.insert((dist, nexts))
                }
            };

            if tile.dist >= num_steps {
                println!("Complete");
                continue;
            }

            let count = dist
                .values()
                .filter(|v| **v + tile.dist <= num_steps && **v % 2 == num_steps % 2)
                .count();

            println!("{:?}", count);
            total += count;
            for (start, d, best_dist) in nexts {
                worklist.push_back(Tile {
                    start: *start,
                    dist: tile.dist + best_dist + 1,
                    offset: Vector2::new(tile_size.x * d.x, tile_size.y * d.y) + tile.offset,
                });
            }
        }
        total
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
