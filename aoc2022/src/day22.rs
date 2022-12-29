use crate::Day;
use cgmath::{Point2, Vector2};

#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone)]
enum Space {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Grid {
    size_x: usize,
    size_y: usize,
    grid: HashMap<Point2<i32>, Space>,
}

impl Grid {}

#[derive(Debug)]
pub enum Cmd {
    Forward(i32),
    Rotate,
    RotateCounter,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(usize)]
enum Dir {
    N = 0,
    E,
    S,
    W,
}
impl Dir {
    fn rev(&self) -> Self {
        [Self::S, Self::W, Self::N, Self::E][*self as usize]
    }
    fn vector(&self) -> Vector2<i32> {
        match self {
            Self::N => Vector2::new(0, -1),
            Self::S => Vector2::new(0, 1),
            Self::E => Vector2::new(1, 0),
            Self::W => Vector2::new(-1, 0),
        }
    }
    fn rotate_clock(&self) -> Dir {
        let rot = [Self::N, Self::E, Self::S, Self::W];
        rot[(*self as usize + 1) % 4]
    }

    fn rotate_counter(&self) -> Dir {
        let rot = [Self::N, Self::E, Self::S, Self::W];

        rot[(*self as usize + 4 - 1) % 4]
    }
    fn face(&self) -> usize {
        match self {
            Self::E => 0,
            Self::S => 1,
            Self::W => 2,
            Self::N => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    A = 0,
    B,
    C,
    D,
    E,
    F,
}
impl Tile {
    const TILE_SIZE: i32 = 50;
    fn top_left(&self) -> Point2<i32> {
        let (x, y) = match self {
            Self::A => (1, 0),
            Self::B => (2, 0),
            Self::C => (1, 1),
            Self::D => (0, 2),
            Self::E => (1, 2),
            Self::F => (0, 3),
        };
        Point2::new(x * Self::TILE_SIZE, y * Self::TILE_SIZE)
    }
    fn edge(&self, d: &Dir) -> Vec<Point2<i32>> {
        match d {
            Dir::N => {
                let tl = self.top_left();
                (0..Self::TILE_SIZE)
                    .map(|o| tl + o * Dir::E.vector())
                    .collect()
            }
            Dir::S => {
                let tl = self.top_left();
                let bl = tl + (Self::TILE_SIZE - 1) * Dir::S.vector();
                (0..Self::TILE_SIZE)
                    .map(|o| bl + o * Dir::E.vector())
                    .collect()
            }
            Dir::E => {
                let tl = self.top_left();
                let tr = tl + (Self::TILE_SIZE - 1) * Dir::E.vector();
                (0..Self::TILE_SIZE)
                    .map(|o| tr + o * Dir::S.vector())
                    .collect()
            }
            Dir::W => {
                let tl = self.top_left();
                (0..Self::TILE_SIZE)
                    .map(|o| tl + o * Dir::S.vector())
                    .collect()
            }
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 22;
    type Input1 = (Grid, Vec<Cmd>);
    type Input2 = (Grid, Vec<Cmd>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let grid: HashMap<_, _> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().flat_map(move |(x, c)| {
                    let space = match c {
                        ' ' => None,
                        '.' => Some(Space::Floor),
                        '#' => Some(Space::Wall),
                        _ => panic!(),
                    };
                    space.map(|s| (Point2::new(x as i32, y as i32), s))
                })
            })
            .collect();

        let max_x = grid
            .iter()
            .max_by_key(|(p, _)| p.x)
            .map(|(p, _)| p.x)
            .unwrap();
        let max_y = grid
            .iter()
            .max_by_key(|(p, _)| p.y)
            .map(|(p, _)| p.y)
            .unwrap();

        let grid = Grid {
            size_x: max_x as usize + 1,
            size_y: max_y as usize + 1,
            grid,
        };

        use regex::Regex;
        let re = Regex::new("([0-9]+)([LR])?").unwrap();

        let line = lines.next().unwrap();
        let cmds: Vec<_> = re
            .captures_iter(line)
            .flat_map(|c| {
                vec![
                    Some(Cmd::Forward(c[1].parse().unwrap())),
                    match c.get(2) {
                        Some(m) if m.as_str() == "L" => Some(Cmd::RotateCounter),
                        Some(m) if m.as_str() == "R" => Some(Cmd::Rotate),
                        Some(_) => panic!(),
                        None => None,
                    },
                ]
                .into_iter()
                .flatten()
            })
            .collect();

        (grid, cmds)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1((grid, cmds): &Self::Input1) -> Self::Sol1 {
        struct Walk {
            dir: Dir,
            pos: Point2<i32>,
        }

        impl Walk {
            fn password(&self) -> usize {
                1000 * (self.pos.y + 1) as usize + 4 * (self.pos.x + 1) as usize + self.dir.face()
            }
            fn walk(&mut self, grid: &Grid, cmd: &Cmd) {
                match cmd {
                    Cmd::Forward(f) => {
                        let v = self.dir.vector();
                        let mut cur = self.pos;
                        let next_pos = std::iter::from_fn(move || {
                            let mut next = cur + v;
                            if next.x < 0 {
                                next.x = grid.size_x as i32;
                            } else if next.x >= grid.size_x as i32 {
                                next.x = 0;
                            }

                            if next.y < 0 {
                                next.y = grid.size_y as i32;
                            } else if next.y >= grid.size_y as i32 {
                                next.y = 0;
                            }
                            cur = next;
                            Some(cur)
                        })
                        .filter_map(|p| grid.grid.get(&p).map(|s| (p, s)))
                        .take(*f as usize)
                        .take_while(|(_x, s)| matches!(s, Space::Floor))
                        .last();

                        if let Some((pos, _)) = next_pos {
                            self.pos = pos;
                        }
                    }
                    Cmd::Rotate => self.dir = self.dir.rotate_clock(),
                    Cmd::RotateCounter => self.dir = self.dir.rotate_counter(),
                }
            }
        }

        let start = grid
            .grid
            .keys()
            .filter(|p| p.y == 0)
            .min_by_key(|p| p.x)
            .cloned()
            .unwrap();

        let mut walker = Walk {
            dir: Dir::E,
            pos: start,
        };

        for c in cmds {
            walker.walk(grid, c);
        }
        walker.password()
    }
    fn p2((grid, cmds): &Self::Input2) -> Self::Sol2 {
        struct Walk {
            dir: Dir,
            pos: Point2<i32>,
        }

        impl Walk {
            fn password(&self) -> usize {
                1000 * (self.pos.y + 1) as usize + 4 * (self.pos.x + 1) as usize + self.dir.face()
            }
            fn walk(&mut self, grid: &Grid, cmd: &Cmd) {
                // AB
                // C
                //DE
                //F
                let remap = [
                    ((Tile::A, Dir::N), (Tile::F, Dir::W), false),
                    ((Tile::A, Dir::W), (Tile::D, Dir::W), true),
                    ((Tile::B, Dir::N), (Tile::F, Dir::S), false),
                    ((Tile::B, Dir::E), (Tile::E, Dir::E), true),
                    ((Tile::C, Dir::E), (Tile::B, Dir::S), false),
                    ((Tile::C, Dir::W), (Tile::D, Dir::N), false),
                    ((Tile::E, Dir::S), (Tile::F, Dir::E), false),
                ];

                match cmd {
                    Cmd::Forward(f) => {
                        let mut cur = self.pos;
                        let mut cur_dir = self.dir;
                        let next_pos = std::iter::from_fn(move || {
                            let (next, next_dir) = if let Some((f, t, flip)) = remap
                                .into_iter()
                                .flat_map(|r| vec![(r.0, r.1, r.2), (r.1, r.0, r.2)].into_iter())
                                .find(|(f, _t, _flip)| {
                                    f.1 == cur_dir && f.0.edge(&cur_dir).contains(&cur)
                                }) {
                                let f_points = f.0.edge(&f.1);
                                let mut t_points = t.0.edge(&t.1);
                                if flip {
                                    t_points.reverse();
                                }
                                let next = f_points
                                    .iter()
                                    .zip(t_points.iter())
                                    .find(|(f, _t)| **f == cur)
                                    .map(|(_f, t)| t)
                                    .cloned()
                                    .unwrap();
                                (next, t.1.rev())
                            } else {
                                (cur + cur_dir.vector(), cur_dir)
                            };

                            cur = next;
                            cur_dir = next_dir;
                            Some((cur, cur_dir))
                        })
                        .map(|cur| (cur, grid.grid.get(&cur.0).unwrap()))
                        .take(*f as usize)
                        .take_while(|(_x, s)| matches!(s, Space::Floor))
                        .last();

                        if let Some((pos, _)) = next_pos {
                            self.pos = pos.0;
                            self.dir = pos.1
                        }
                    }
                    Cmd::Rotate => self.dir = self.dir.rotate_clock(),
                    Cmd::RotateCounter => self.dir = self.dir.rotate_counter(),
                }
            }
        }

        let start = grid
            .grid
            .keys()
            .filter(|p| p.y == 0)
            .min_by_key(|p| p.x)
            .cloned()
            .unwrap();

        let mut walker = Walk {
            dir: Dir::E,
            pos: start,
        };

        for c in cmds {
            walker.walk(grid, c);
        }
        walker.password()
    }
}

crate::default_tests!(88226, 57305);
crate::path_tests!(
    [(t1, "test/day22.txt", 6032)],
    [
        // This solution is not generic enough to analyze tiles and connect them together.
        //(t2, "test/day22.txt", 5031)
    ]
);
