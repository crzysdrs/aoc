use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Pos {
    Wall,
    Guard,
}

pub const LEFT: Vector2<i32> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<i32> = Vector2::new(1, 0);
pub const UP: Vector2<i32> = Vector2::new(0, -1);
pub const DOWN: Vector2<i32> = Vector2::new(0, 1);
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Guard {
    pos: Point2<i32>,
    dir: Vector2<i32>,
}
struct GuardIter<'a> {
    guard: &'a mut Guard,
    start: bool,
    w: usize,
    v: &'a HashMap<Point2<i32>, Pos>,
}

impl Iterator for GuardIter<'_> {
    type Item = Guard;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;
            return Some(self.guard.clone());
        }
        let next = self.guard.next_pos();
        if next.x >= self.w as i32 || next.y >= self.w as i32 || next.x < 0 || next.y < 0 {
            return None;
        } else if self.v.get(&next) == Some(&Pos::Wall) {
            self.guard.turn_right();
        } else {
            self.guard.move_next();
        }
        Some(self.guard.clone())
    }
}

impl Guard {
    fn iter<'a>(&'a mut self, w: usize, v: &'a HashMap<Point2<i32>, Pos>) -> GuardIter<'a> {
        GuardIter {
            guard: self,
            start: true,
            v,
            w,
        }
    }
    fn next_pos(&self) -> Point2<i32> {
        self.pos + self.dir
    }
    fn move_next(&mut self) {
        self.pos = self.next_pos();
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!(),
        };
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = (usize, HashMap<Point2<i32>, Pos>);
    type Input2 = (usize, HashMap<Point2<i32>, Pos>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        (
            s.lines().count(),
            s.lines()
                .enumerate()
                .flat_map(|(y, s)| {
                    s.chars().enumerate().flat_map(move |(x, c)| {
                        let p = Point2::new(x as i32, y as i32);
                        match c {
                            '#' => Some((p, Pos::Wall)),
                            '^' => Some((p, Pos::Guard)),
                            '.' => None,
                            _ => panic!(),
                        }
                    })
                })
                .collect(),
        )
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1((w, v): &Self::Input1) -> Self::Sol1 {
        let (guard_pos, _) = v.iter().find(|(_k, v)| **v == Pos::Guard).unwrap();

        let mut g = Guard {
            pos: *guard_pos,
            dir: UP,
        };

        let seen: HashSet<_> = g.iter(*w, v).map(|g| g.pos).collect();
        seen.len()
    }
    fn p2((w, v): &Self::Input2) -> Self::Sol2 {
        let (guard_pos, _) = v.iter().find(|(_k, v)| **v == Pos::Guard).unwrap();

        let mut g = Guard {
            pos: *guard_pos,
            dir: UP,
        };

        let seen: Vec<_> = g.iter(*w, v).collect();

        let set: HashSet<_> = seen
            .iter()
            .enumerate()
            .flat_map(|(i, p)| {
                let mut new_g = (*p).clone();
                let maybe_obstruct = new_g.next_pos();
                if v.get(&maybe_obstruct) == Some(&Pos::Wall) {
                    return None;
                } else if seen
                    .iter()
                    .position(|v| v.pos == maybe_obstruct)
                    .map(|o| o < i)
                    .unwrap_or(false)
                {
                    return None;
                }
                let mut v = v.clone();
                v.insert(maybe_obstruct.clone(), Pos::Wall);
                let mut iter = new_g.iter(*w, &v);
                let mut seen = HashSet::new();
                let looped = loop {
                    if let Some(g) = iter.next() {
                        if seen.get(&g).is_some() {
                            break true;
                        }
                        seen.insert(g);
                    } else {
                        break false;
                    }
                };

                if looped {
                    Some(maybe_obstruct)
                } else {
                    None
                }
            })
            .collect();
        //println!("{:?}", set);
        set.len()
    }
}

crate::default_tests!(5162, 1909);
crate::string_tests!(
    [(
        foo_sol1,
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        41
    )],
    [(
        foo_sol2,
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        6
    )]
);
