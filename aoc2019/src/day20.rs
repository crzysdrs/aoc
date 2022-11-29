use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::io::Result as IoResult;

#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Copy, Clone)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    #[allow(unused)]
    fn from_vec(v: Vector2<i32>) -> Dir {
        match v {
            Vector2 { x: 0, y: 1 } => Dir::North,
            Vector2 { x: 0, y: -1 } => Dir::South,
            Vector2 { x: 1, y: 0 } => Dir::East,
            Vector2 { x: -1, y: 0 } => Dir::West,
            _ => panic!("Bad Direction"),
        }
    }
    #[allow(dead_code)]
    fn rotate(&self, left: bool) -> Dir {
        let dirs = &[Dir::North, Dir::West, Dir::South, Dir::East];
        let cur = dirs.iter().position(|x| *x == *self).unwrap();
        let next = if left { cur + 1 } else { dirs.len() + cur - 1 } % dirs.len();
        dirs[next]
    }
}

fn point_dir(p: &Point2<i32>, d: &Dir) -> Point2<i32> {
    let mut p = p.clone();
    match d {
        Dir::North => {
            p.y += 1;
        }
        Dir::South => {
            p.y -= 1;
        }
        Dir::East => {
            p.x += 1;
        }
        Dir::West => {
            p.x -= 1;
        }
    }
    p
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Tile {
    Portal(String),
    Empty,
}
pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day20.txt")?;
    //     let s = "         A
    //          A
    //   #######.#########
    //   #######.........#
    //   #######.#######.#
    //   #######.#######.#
    //   #######.#######.#
    //   #####  B    ###.#
    // BC...##  C    ###.#
    //   ##.##       ###.#
    //   ##...DE  F  ###.#
    //   #####    G  ###.#
    //   #########.#####.#
    // DE..#######...###.#
    //   #.#########.###.#
    // FG..#########.....#
    //   ###########.#####
    //              Z
    //              Z  ".to_string();
    //     let s = "                   A
    //                    A
    //   #################.#############
    //   #.#...#...................#.#.#
    //   #.#.#.###.###.###.#########.#.#
    //   #.#.#.......#...#.....#.#.#...#
    //   #.#########.###.#####.#.#.###.#
    //   #.............#.#.....#.......#
    //   ###.###########.###.#####.#.#.#
    //   #.....#        A   C    #.#.#.#
    //   #######        S   P    #####.#
    //   #.#...#                 #......VT
    //   #.#.#.#                 #.#####
    //   #...#.#               YN....#.#
    //   #.###.#                 #####.#
    // DI....#.#                 #.....#
    //   #####.#                 #.###.#
    // ZZ......#               QG....#..AS
    //   ###.###                 #######
    // JO..#.#.#                 #.....#
    //   #.#.#.#                 ###.#.#
    //   #...#..DI             BU....#..LF
    //   #####.#                 #.#####
    // YN......#               VT..#....QG
    //   #.###.#                 #.###.#
    //   #.#...#                 #.....#
    //   ###.###    J L     J    #.#.###
    //   #.....#    O F     P    #.#...#
    //   #.###.#####.#.#####.#####.###.#
    //   #...#.#.#...#.....#.....#.#...#
    //   #.#####.###.###.#.#.#########.#
    //   #...#.#.....#...#.#.#.#.....#.#
    //   #.###.#####.###.###.#.#.#######
    //   #.#.........#...#.............#
    //   #########.###.###.#############
    //            B   J   C
    //            U   P   P               ".to_string();

    let grid = s
        .split("\n")
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    use petgraph::Graph;
    let mut deps = Graph::<Tile, _, petgraph::Undirected>::new_undirected();

    let mut portals = grid
        .iter()
        .flat_map(|(p, v)| {
            if v.is_alphabetic() {
                let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];

                for d in &dirs {
                    let new_p = point_dir(p, d);
                    if let Some('.') = grid.get(&new_p) {
                        let opposite = p - (new_p - p);
                        if let Some(n) = grid.get(&opposite) {
                            if n.is_alphabetic() {
                                let mut s = [(opposite, *n), (*p, *v)];
                                s.sort_by(|a, b| {
                                    let a = a.0;
                                    let b = b.0;
                                    match a.x.cmp(&b.x) {
                                        std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                                        e => e,
                                    }
                                });
                                let s = s.iter().map(|(_p, c)| c).collect::<String>();
                                return Some((Tile::Portal(s), vec![new_p]));
                            }
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    //println!("Portals {:?}", portals);

    use itertools::Itertools;
    portals.sort_by_key(|x| x.0.clone());
    let portal_edges = portals
        .into_iter()
        .coalesce(|mut x, y| {
            if x.0 == y.0 {
                x.1.extend(y.1);
                Ok(x)
            } else {
                Err((x, y))
            }
        })
        .collect::<Vec<_>>();

    let empty = grid
        .iter()
        .filter(|(_k, v)| **v == '.')
        .map(|(k, _v)| (k, (deps.add_node(Tile::Empty))))
        .collect::<HashMap<_, _>>();

    let all_nodes = empty;

    //println!("{:?}", all_nodes);
    for (k, _v) in grid.iter() {
        let east = point_dir(k, &Dir::East);
        let north = point_dir(k, &Dir::North);

        if let Some(n) = all_nodes.get(k) {
            for d in [east, north].iter() {
                if let Some(n2) = all_nodes.get(d) {
                    deps.add_edge(*n, *n2, 1);
                }
            }
        }
    }

    for (_k, v) in &portal_edges {
        if v.len() == 2 {
            deps.add_edge(
                *all_nodes.get(&v[0]).unwrap(),
                *all_nodes.get(&v[1]).unwrap(),
                1,
            );
        }
    }

    //println!("{} {}", deps.node_count(), deps.edge_count());

    let aa = portal_edges
        .iter()
        .find_map(|(p, v)| {
            if *p == Tile::Portal("AA".to_string()) {
                Some(*all_nodes.get(&v[0]).unwrap())
            } else {
                None
            }
        })
        .unwrap();

    let zz = portal_edges
        .iter()
        .find_map(|(p, v)| {
            if *p == Tile::Portal("ZZ".to_string()) {
                Some(*all_nodes.get(&v[0]).unwrap())
            } else {
                None
            }
        })
        .unwrap();

    //let zz = &all_nodes[&Point2 {x:9, y:6}];

    use petgraph::algo::astar;
    println!(
        "{:?}",
        astar(&deps, aa, |x| x == zz, |e| *e.weight(), |_| 0)
            .unwrap()
            .0
    );
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day20.txt")?;
    //     let s = "         A
    //          A
    //   #######.#########
    //   #######.........#
    //   #######.#######.#
    //   #######.#######.#
    //   #######.#######.#
    //   #####  B    ###.#
    // BC...##  C    ###.#
    //   ##.##       ###.#
    //   ##...DE  F  ###.#
    //   #####    G  ###.#
    //   #########.#####.#
    // DE..#######...###.#
    //   #.#########.###.#
    // FG..#########.....#
    //   ###########.#####
    //              Z
    //              Z  ".to_string();
    //     let s = "                   A
    //                    A
    //   #################.#############
    //   #.#...#...................#.#.#
    //   #.#.#.###.###.###.#########.#.#
    //   #.#.#.......#...#.....#.#.#...#
    //   #.#########.###.#####.#.#.###.#
    //   #.............#.#.....#.......#
    //   ###.###########.###.#####.#.#.#
    //   #.....#        A   C    #.#.#.#
    //   #######        S   P    #####.#
    //   #.#...#                 #......VT
    //   #.#.#.#                 #.#####
    //   #...#.#               YN....#.#
    //   #.###.#                 #####.#
    // DI....#.#                 #.....#
    //   #####.#                 #.###.#
    // ZZ......#               QG....#..AS
    //   ###.###                 #######
    // JO..#.#.#                 #.....#
    //   #.#.#.#                 ###.#.#
    //   #...#..DI             BU....#..LF
    //   #####.#                 #.#####
    // YN......#               VT..#....QG
    //   #.###.#                 #.###.#
    //   #.#...#                 #.....#
    //   ###.###    J L     J    #.#.###
    //   #.....#    O F     P    #.#...#
    //   #.###.#####.#.#####.#####.###.#
    //   #...#.#.#...#.....#.....#.#...#
    //   #.#####.###.###.#.#.#########.#
    //   #...#.#.....#...#.#.#.#.....#.#
    //   #.###.#####.###.###.#.#.#######
    //   #.#.........#...#.............#
    //   #########.###.###.#############
    //            B   J   C
    //            U   P   P               ".to_string();

    let grid = s
        .split("\n")
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    let _min_x = grid.keys().map(|p| p.x).min().unwrap();
    let _min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    use petgraph::Graph;
    let mut deps = Graph::<Tile, usize, petgraph::Undirected>::new_undirected();

    let mut portals = grid
        .iter()
        .flat_map(|(p, v)| {
            if v.is_alphabetic() {
                let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
                for d in &dirs {
                    let new_p = point_dir(p, d);
                    if let Some('.') = grid.get(&new_p) {
                        let opposite = p - (new_p - p);
                        if let Some(n) = grid.get(&opposite) {
                            if n.is_alphabetic() {
                                let mut s = [(opposite, *n), (*p, *v)];
                                s.sort_by(|a, b| {
                                    let a = a.0;
                                    let b = b.0;
                                    match a.x.cmp(&b.x) {
                                        std::cmp::Ordering::Equal => a.y.cmp(&b.y),
                                        e => e,
                                    }
                                });
                                let s = s.iter().map(|(_p, c)| c).collect::<String>();
                                return Some((Tile::Portal(s), vec![new_p]));
                            }
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    //println!("Portals {:?}", portals);

    println!("{} {}", max_x, max_y);
    let outer = move |p: &Point2<_>| p.x < 4 || p.y < 4 || p.x > max_x - 4 || p.y > max_y - 4;

    use itertools::Itertools;
    portals.sort_by_key(|x| x.0.clone());
    let portal_edges = portals
        .into_iter()
        .coalesce(|mut x, y| {
            if x.0 == y.0 {
                x.1.extend(y.1);
                Ok(x)
            } else {
                Err((x, y))
            }
        })
        .collect::<Vec<_>>();
    use petgraph::graph::NodeIndex;

    fn recurse(
        outer: &dyn Fn(&Point2<i32>) -> bool,
        deps: &mut Graph<Tile, usize, petgraph::Undirected>,
        grid: &HashMap<Point2<i32>, char>,
        portal_edges: &[(Tile, Vec<Point2<i32>>)],
        all_nodes: &mut HashMap<(u32, Point2<i32>), NodeIndex<u32>>,
        depth: u32,
        max_depth: u32,
    ) {
        if depth > max_depth {
            return;
        }

        all_nodes.extend(
            grid.iter()
                .filter(|(_k, v)| **v == '.')
                .map(|(k, _v)| ((depth, *k), (deps.add_node(Tile::Empty)))),
        );

        //println!("{:?}", all_nodes);
        for (k, _v) in grid.iter() {
            let east = point_dir(k, &Dir::East);
            let north = point_dir(k, &Dir::North);

            if let Some(n) = all_nodes.get(&(depth, *k)) {
                for d in [east, north].iter() {
                    if let Some(n2) = all_nodes.get(&(depth, *d)) {
                        deps.add_edge(*n, *n2, 1);
                    }
                }
            }
        }

        let lower = depth + 1;
        recurse(outer, deps, grid, portal_edges, all_nodes, lower, max_depth);
        for (_k, v) in portal_edges.iter() {
            if v.len() == 2 {
                let (outer, inner): (Vec<_>, Vec<_>) = v.iter().map(|x| *x).partition(outer);
                match (
                    all_nodes.get(&(lower, outer[0])),
                    all_nodes.get(&(depth, inner[0])),
                ) {
                    (Some(lower), Some(this)) => {
                        deps.add_edge(*lower, *this, 1);
                    }
                    _ => {}
                }
            }
        }
    }

    let mut all_nodes = HashMap::new();
    recurse(
        &outer,
        &mut deps,
        &grid,
        &portal_edges,
        &mut all_nodes,
        0,
        256,
    );

    let aa = portal_edges
        .iter()
        .find_map(|(p, v)| {
            if *p == Tile::Portal("AA".to_string()) {
                Some(*all_nodes.get(&(0, v[0])).unwrap())
            } else {
                None
            }
        })
        .unwrap();

    let zz = portal_edges
        .iter()
        .find_map(|(p, v)| {
            if *p == Tile::Portal("ZZ".to_string()) {
                Some(*all_nodes.get(&(0, v[0])).unwrap())
            } else {
                None
            }
        })
        .unwrap();

    use petgraph::algo::astar;
    if let Some((v, _)) = astar(&deps, aa, |x| x == zz, |e| *e.weight(), |_| 0) {
        println!("Part 2: {:?}", v);
    } else {
        println!("No Path");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(true);
    }
}
