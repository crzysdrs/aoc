use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq)]
pub enum Pos {
    Wall,
    Guard,
}

pub const LEFT: Vector2<i32> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<i32> = Vector2::new(1, 0);
pub const UP: Vector2<i32> = Vector2::new(0, -1);
pub const DOWN: Vector2<i32> = Vector2::new(0, 1);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 6;
    type Input1 = HashMap<Point2<i32>, Pos>;
    type Input2 = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
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
            .collect()
    }
    fn process_input2(_s: &str) -> Self::Input2 {
        unimplemented!()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let (guard_pos, _) = v.iter().find(|(k, v)| **v == Pos::Guard).unwrap();
        struct Guard {
            pos: Point2<i32>,
            dir: Vector2<i32>,
        }

        let mut g = Guard {
            pos: *guard_pos,
            dir: UP,
        };

        impl Guard {
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

        let mut seen = HashSet::new();
        seen.insert(*guard_pos);

        loop {
            let next = g.next_pos();
            if next.x >= 130 || next.y >= 130 || next.x < 0 || next.y < 0 {
                break;
            } else if v.get(&next) == Some(&Pos::Wall) {
                g.turn_right();
            } else {
                seen.insert(next);
                g.move_next();
            }
        }

        seen.iter().count()
    }
    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        unimplemented!()
    }
}

//crate::default_tests!((), ());
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
