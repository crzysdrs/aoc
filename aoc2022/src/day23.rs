use crate::Day;
use cgmath::{Point2, Vector2};
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Dir {
    N,
    NW,
    NE,
    E,
    S,
    SW,
    SE,
    W,
}
impl Dir {
    fn vector(&self) -> Vector2<i32> {
        match self {
            Self::N => Vector2::new(0, -1),
            Self::S => Vector2::new(0, 1),
            Self::E => Vector2::new(1, 0),
            Self::W => Vector2::new(-1, 0),
            Self::NE => Self::N.vector() + Self::E.vector(),
            Self::NW => Self::N.vector() + Self::W.vector(),
            Self::SW => Self::S.vector() + Self::W.vector(),
            Self::SE => Self::S.vector() + Self::E.vector(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    map: HashSet<Point2<i32>>,
}

impl Grid {
    fn empties(&self) -> usize {
        let xs = self.map.iter().map(|p| p.x).minmax().into_option().unwrap();
        let ys = self.map.iter().map(|p| p.y).minmax().into_option().unwrap();

        (xs.1 - xs.0 + 1) as usize * (ys.1 - ys.0 + 1) as usize - self.map.len()
    }
}

fn compute(grid: &mut Grid, max_iterations: Option<usize>) -> usize {
    let all = [
        Dir::N,
        Dir::NW,
        Dir::NE,
        Dir::E,
        Dir::S,
        Dir::SW,
        Dir::SE,
        Dir::W,
    ];
    let mut cardinals = [
        (Dir::N, [Dir::N, Dir::NE, Dir::NW]),
        (Dir::S, [Dir::S, Dir::SE, Dir::SW]),
        (Dir::W, [Dir::W, Dir::SW, Dir::NW]),
        (Dir::E, [Dir::E, Dir::SE, Dir::NE]),
    ];

    let mut i = 0;
    'no_move: loop {
        if let Some(max) = &max_iterations {
            if i == *max {
                break 'no_move *max;
            }
        }
        i += 1;
        let proposals = grid
            .map
            .iter()
            .map(|e| {
                if all
                    .iter()
                    .all(|d| grid.map.get(&(e + d.vector())).is_none())
                {
                    None
                } else {
                    cardinals.iter().find_map(|(c, choices)| {
                        if !choices
                            .iter()
                            .any(|choice| grid.map.get(&(*e + choice.vector())).is_some())
                        {
                            Some((*e, e + c.vector()))
                        } else {
                            None
                        }
                    })
                }
            })
            .flatten()
            .fold(HashMap::new(), |mut state, elf| {
                state
                    .entry(elf.1)
                    .and_modify(|s| *s = None)
                    .or_insert(Some(elf));
                state
            });

        if proposals.values().flatten().count() == 0 {
            break 'no_move i;
        }
        for (f, t) in proposals.values().flatten() {
            grid.map.remove(f);
            grid.map.insert(*t);
        }

        cardinals.rotate_left(1);
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 23;
    type Input1 = Grid;
    type Input2 = Grid;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let map = s
            .lines()
            .enumerate()
            .flat_map(|(y, c)| {
                c.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
            })
            .filter(|(_p, c)| *c == '#')
            .map(|(p, _c)| p)
            .collect();

        Grid { map }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(grid: &Self::Input1) -> Self::Sol1 {
        let mut grid = grid.clone();
        compute(&mut grid, Some(10));
        grid.empties()
    }
    fn p2(grid: &Self::Input2) -> Self::Sol2 {
        let mut grid = grid.clone();
        compute(&mut grid, None)
    }
}

crate::default_tests!(4075, 950);
crate::path_tests!([(t1, "test/day23.txt", 110)], [(t2, "test/day23.txt", 20)]);
