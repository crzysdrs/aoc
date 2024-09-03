use crate::Day;
use cgmath::{Point2, Vector2};
use petgraph::algo::astar::astar;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Space {
    Wall,
    Empty,
    Blizzard,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
pub enum Dir {
    N = 0,
    E,
    S,
    W,
}
impl Dir {
    fn vector(&self) -> Vector2<i32> {
        match self {
            Self::N => Vector2::new(0, -1),
            Self::S => Vector2::new(0, 1),
            Self::E => Vector2::new(1, 0),
            Self::W => Vector2::new(-1, 0),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Basin {
    max_x: i32,
    max_y: i32,
    blizzards: Vec<(Point2<i32>, Dir)>,
    spaces: Vec<Space>,
}

impl std::fmt::Display for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.spaces.chunks(self.max_x as usize) {
            for i in line {
                write!(
                    f,
                    "{}",
                    match i {
                        Space::Wall => '#',
                        Space::Blizzard => 'B',
                        Space::Empty => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Basin {
    fn offset(&self, p: &Point2<i32>) -> Option<usize> {
        if p.x >= 0 && p.y >= 0 {
            Some(p.y as usize * self.max_x as usize + p.x as usize)
        } else {
            None
        }
    }
    fn lookup(&self, p: &Point2<i32>) -> Option<&Space> {
        let offset = self.offset(p)?;
        self.spaces.get(offset)
    }
    fn reverse(&self, p: usize) -> Point2<i32> {
        Point2::new(
            (p % self.max_x as usize) as i32,
            (p / self.max_x as usize) as i32,
        )
    }
    fn iter(&self) -> BasinIter {
        BasinIter {
            basin: self.clone(),
        }
    }
}
struct BasinIter {
    basin: Basin,
}

impl Iterator for BasinIter {
    type Item = Basin;
    fn next(&mut self) -> Option<Self::Item> {
        self.basin.blizzards.iter_mut().for_each(|(p, d)| {
            let mut next = *p + d.vector();
            if next.x == 0 {
                next.x = self.basin.max_x - 2;
            } else if next.x == self.basin.max_x - 1 {
                next.x = 1;
            }
            if next.y == 0 {
                next.y = self.basin.max_y - 2;
            } else if next.y == self.basin.max_y - 1 {
                next.y = 1;
            }
            assert!(next.x < self.basin.max_x);
            assert!(next.y < self.basin.max_y);
            *p = next;
        });
        self.basin.spaces.iter_mut().for_each(|x| {
            if matches!(*x, Space::Blizzard) {
                *x = Space::Empty;
            }
        });
        self.basin.blizzards.iter().for_each(|(p, _d)| {
            self.basin.spaces[(p.x + p.y * self.basin.max_x) as usize] = Space::Blizzard;
        });

        Some(self.basin.clone())
    }
}

fn build_graph(
    basin: &Basin,
) -> (
    petgraph::Graph<(), ()>,
    petgraph::graph::NodeIndex,
    HashSet<petgraph::graph::NodeIndex>,
    HashSet<petgraph::graph::NodeIndex>,
) {
    use petgraph::Graph;

    let finish = basin
        .spaces
        .iter()
        .rposition(|x| *x == Space::Empty)
        .unwrap();

    let start = basin
        .spaces
        .iter()
        .position(|x| *x == Space::Empty)
        .unwrap();

    let all_basins: Vec<_> = std::iter::once(basin.clone())
        .chain(basin.iter().take_while(|b| b != basin))
        .collect();

    println!("Generated {} basins", all_basins.len());

    let mut graph = Graph::new();

    let basin_indices: Vec<Vec<_>> = all_basins
        .iter()
        .map(|b| b.spaces.iter().map(|_x| graph.add_node(())).collect())
        .collect();

    let all: Vec<_> = all_basins.iter().zip(basin_indices.iter()).collect();

    println!("Building Graph");
    all.windows(2)
        .chain(std::iter::once([*all.last().unwrap(), all[0]].as_slice()))
        .for_each(|win| {
            let (b_0, idx_0) = win[0];
            let (b_1, idx_1) = win[1];
            b_0.spaces
                .iter()
                .enumerate()
                .zip(idx_0.iter())
                .filter(|((_i, b_space), _idx)| matches!(b_space, Space::Empty))
                .for_each(|((i, _b_space), _idx)| {
                    let dirs = [Dir::N, Dir::S, Dir::E, Dir::W];
                    std::iter::once(b_0.reverse(i))
                        .chain(dirs.iter().map(|d| b_0.reverse(i) + d.vector()))
                        .filter(|p| b_1.offset(p).is_some())
                        .for_each(|p| {
                            if matches!(b_1.lookup(&p), Some(Space::Empty)) {
                                graph.add_edge(idx_0[i], idx_1[b_1.offset(&p).unwrap()], ());
                            }
                        });
                });
        });

    let finishes: HashSet<_> = all.iter().map(|(_b, b_idx)| b_idx[finish]).collect();
    let starts: HashSet<_> = all.iter().map(|(_b, b_idx)| b_idx[start]).collect();
    let start_idx = all.iter().map(|(_b, b_idx)| b_idx[start]).next().unwrap();

    (graph, start_idx, starts, finishes)
}
impl BasinIter {}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 24;
    type Input1 = Basin;
    type Input2 = Basin;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut blizzards = vec![];
        let mut max_x = 0;
        let mut max_y = 0;
        let spaces: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, c)| {
                c.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
            })
            .inspect(|(p, _c)| {
                max_x = std::cmp::max(p.x + 1, max_x);
                max_y = std::cmp::max(p.y + 1, max_y);
            })
            .map(|(p, c)| match c {
                '#' => Space::Wall,
                '.' => Space::Empty,
                '>' => {
                    blizzards.push((p, Dir::E));
                    Space::Blizzard
                }
                '^' => {
                    blizzards.push((p, Dir::N));
                    Space::Blizzard
                }
                '<' => {
                    blizzards.push((p, Dir::W));
                    Space::Blizzard
                }
                'v' => {
                    blizzards.push((p, Dir::S));
                    Space::Blizzard
                }
                _ => panic!(),
            })
            .collect();

        assert_eq!((max_x * max_y) as usize, spaces.len());
        Basin {
            max_x,
            max_y,
            blizzards,
            spaces,
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(basin: &Self::Input2) -> Self::Sol2 {
        let (graph, start_idx, _starts, finishes) = build_graph(basin);

        println!("Computing path");
        let path = astar(
            &graph,
            start_idx,
            |finish| finishes.contains(&finish),
            |_e| 1,
            |_| 0,
        )
        .unwrap();

        path.0
    }
    fn p2(basin: &Self::Input1) -> Self::Sol1 {
        let (graph, start_idx, starts, finishes) = build_graph(basin);

        println!("Computing path");
        let path1 = astar(
            &graph,
            start_idx,
            |finish| finishes.contains(&finish),
            |_e| 1,
            |_| 0,
        )
        .unwrap();

        let path2 = astar(
            &graph,
            *path1.1.last().unwrap(),
            |finish| starts.contains(&finish),
            |_e| 1,
            |_| 0,
        )
        .unwrap();

        let path3 = astar(
            &graph,
            *path2.1.last().unwrap(),
            |finish| finishes.contains(&finish),
            |_e| 1,
            |_| 0,
        )
        .unwrap();

        path1.0 + path2.0 + path3.0
    }
}

crate::default_tests!(326, 976);
crate::path_tests!([(t1, "test/day24.txt", 18)], [(t2, "test/day24.txt", 54)]);
