use crate::Day;
use cgmath::{Point2, Vector2};

#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Path {
    Forest,
    Clear,
    Up,
    Left,
    Right,
    Down,
}

const UP: Vector2<i32> = Vector2::new(0, -1);
const DOWN: Vector2<i32> = Vector2::new(0, 1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 23;
    type Input1 = HashMap<Point2<i32>, Path>;
    type Input2 = HashMap<Point2<i32>, Path>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars().enumerate().map(move |(x, c)| {
                    let p = Point2::new(x as i32, y as i32);
                    let e = match c {
                        '#' => Path::Forest,
                        '.' => Path::Clear,
                        '^' => Path::Up,
                        '>' => Path::Right,
                        '<' => Path::Left,
                        'v' => Path::Down,
                        _ => panic!(),
                    };
                    (p, e)
                })
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let start = v
            .iter()
            .find(|(k, v)| k.y == 0 && **v == Path::Clear)
            .map(|(k, _v)| k)
            .cloned()
            .unwrap();

        let max_y = v.keys().max_by_key(|v| v.y).copied().unwrap().y;

        let exit = v
            .iter()
            .find(|(k, v)| k.y == max_y && **v == Path::Clear)
            .map(|(k, _v)| k)
            .cloned()
            .unwrap();

        fn depth_first(
            path: &mut Vec<Point2<i32>>,
            locs: &HashMap<Point2<i32>, Path>,
            pos: Point2<i32>,
            exit: Point2<i32>,
            dist: usize,
        ) -> Option<usize> {
            //println!("{:?}", (dist, pos));
            if exit == pos {
                return Some(dist);
            }
            let dirs = [UP, DOWN, LEFT, RIGHT];
            let cur = locs.get(&pos).unwrap();

            dirs.iter()
                .flat_map(|d| {
                    match (cur, *d) {
                        (Path::Clear, _)
                        | (Path::Left, LEFT)
                        | (Path::Right, RIGHT)
                        | (Path::Down, DOWN)
                        | (Path::Up, UP) => {}
                        _ => return None,
                    }
                    let new_pos = pos + d;
                    match locs.get(&new_pos) {
                        Some(Path::Left | Path::Right | Path::Up | Path::Down | Path::Clear) => {
                            if !path.iter().any(|p| *p == new_pos) {
                                path.push(new_pos);
                                let new = depth_first(path, locs, new_pos, exit, dist + 1)
                                    .map(|dist| (d, dist));
                                path.pop();
                                new
                            } else {
                                None
                            }
                        }
                        Some(Path::Forest) | None => None,
                    }
                })
                .max_by_key(|(_, v)| *v)
                .map(|(_, v)| v)
        }

        depth_first(&mut vec![start], v, start, exit, 0).unwrap()
    }

    fn p2(locs: &Self::Input2) -> Self::Sol2 {
        let start = locs
            .iter()
            .find(|(k, v)| k.y == 0 && **v == Path::Clear)
            .map(|(k, _v)| k)
            .cloned()
            .unwrap();

        let max_y = locs.keys().max_by_key(|v| v.y).copied().unwrap().y;

        let exit = locs
            .iter()
            .find(|(k, v)| k.y == max_y && **v == Path::Clear)
            .map(|(k, _v)| k)
            .cloned()
            .unwrap();

        enum Command {
            Visit(Vec<usize>, usize, usize),
            MaxN(usize),
        }

        let node_index: HashMap<_, _> = locs.iter().enumerate().map(|(i, v)| (v.0, i)).collect();

        let start_id = node_index.get(&start).copied().unwrap();
        let exit_id = node_index.get(&exit).copied().unwrap();

        let mut data_stack: Vec<Option<usize>> = vec![];
        let mut command_stack = vec![Command::Visit(vec![start_id], start_id, 0)];

        #[derive(Copy, Clone)]
        struct Edge {
            target: usize,
            weight: usize,
        }
        struct Node {
            adj: Vec<Edge>,
        }

        let walkable = |v: Path| {
            matches!(
                v,
                Path::Clear | Path::Left | Path::Right | Path::Up | Path::Down
            )
        };

        let mut nodes: HashMap<_, _> = locs
            .iter()
            .filter(|(_p, v)| walkable(**v))
            .map(|(p, _v)| {
                let dirs = [UP, DOWN, LEFT, RIGHT];
                (
                    node_index.get(p).unwrap(),
                    Node {
                        adj: dirs
                            .iter()
                            .filter(|d| locs.get(&(*p + *d)).map(|p| walkable(*p)).unwrap_or(false))
                            .flat_map(|d| node_index.get(&(p + d)).copied())
                            .map(|n| Edge {
                                target: n,
                                weight: 1,
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        let node_keys = nodes.keys().copied().collect::<Vec<_>>();
        for i in node_keys {
            if nodes[i].adj.len() == 2 {
                let v = nodes[i].adj.clone();
                let left = nodes.get_mut(&v[0].target).unwrap();
                left.adj.retain(|p| p.target != *i);
                left.adj.push(Edge {
                    weight: v[1].weight + v[0].weight,
                    ..v[1]
                });

                let right = nodes.get_mut(&v[1].target).unwrap();

                right.adj.retain(|p| p.target != *i);
                right.adj.push(Edge {
                    weight: v[0].weight + v[1].weight,
                    ..v[0]
                });

                nodes.get_mut(i).unwrap().adj = vec![];
            }
        }

        nodes.retain(|_k, v| !v.adj.is_empty());

        //let mut dist = HashMap::new();
        let mut best_dist = 0;
        while let Some(stack) = command_stack.pop() {
            match stack {
                Command::Visit(mut path, pos, dist) => {
                    //println!("{:?}", (dist, pos));
                    if exit_id == pos {
                        if dist > best_dist {
                            println!("Dist {}", dist);
                            best_dist = dist;
                        }
                        data_stack.push(Some(dist));
                        continue;
                    }
                    let cur = nodes.get(&pos).unwrap();
                    let cmds: Vec<_> = cur
                        .adj
                        .iter()
                        .flat_map(|idx| {
                            if !path.iter().any(|p| *p == idx.target) {
                                path.push(idx.target);

                                let new = Some(Command::Visit(
                                    path.to_vec(),
                                    idx.target,
                                    dist + idx.weight,
                                ));
                                path.pop();
                                new
                            } else {
                                None
                            }
                        })
                        .collect();
                    let max_n = cmds.len();
                    command_stack.push(Command::MaxN(max_n));
                    command_stack.extend(cmds);
                }
                Command::MaxN(v) => {
                    let max = (0..v).map(|_| data_stack.pop().unwrap()).max().flatten();
                    data_stack.push(max);
                }
            }
        }
        assert_eq!(data_stack.len(), 1);
        data_stack.pop().unwrap().unwrap_or(0)
    }
}

crate::default_tests!(2282, 6646);
crate::string_tests!(
    [(
        foo_sol1,
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        94
    )],
    [(
        foo_sol2,
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        154
    )]
);
