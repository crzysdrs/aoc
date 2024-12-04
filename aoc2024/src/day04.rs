use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

pub const LEFT: Vector2<i32> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<i32> = Vector2::new(1, 0);
pub const UP: Vector2<i32> = Vector2::new(0, 1);
pub const DOWN: Vector2<i32> = Vector2::new(0, -1);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 4;
    type Input1 = Vec<Vec<Letter>>;
    type Input2 = Vec<Vec<Letter>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        'X' => Letter::X,
                        'M' => Letter::M,
                        'A' => Letter::A,
                        'S' => Letter::S,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let dirs = [
            UP,
            LEFT,
            RIGHT,
            DOWN,
            UP + LEFT,
            UP + RIGHT,
            DOWN + LEFT,
            DOWN + RIGHT,
        ];

        let mut count = 0;
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if v[y][x] == Letter::X {
                    let pos = Point2::new(x as i32, y as i32);
                    for d in dirs {
                        let mut all_seen = true;
                        for (l, i) in [Letter::M, Letter::A, Letter::S].iter().zip(1..) {
                            let new = pos + d.map(|c| i * c as i32);
                            if new.x < 0 || new.y < 0 {
                                all_seen = false;
                            } else {
                                let new = Point2::new(new.x as usize, new.y as usize);
                                if v.get(new.y).and_then(|v| v.get(new.x)) == Some(l) {
                                } else {
                                    all_seen = false;
                                }
                            }
                        }
                        if all_seen {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let dirs = [UP + LEFT, UP + RIGHT, DOWN + LEFT, DOWN + RIGHT];

        let mut letter_a = HashMap::new();

        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if v[y][x] == Letter::M {
                    let pos = Point2::new(x as i32, y as i32);
                    for d in dirs {
                        let mut all_seen = true;
                        for (l, i) in [Letter::A, Letter::S].iter().zip(1..) {
                            let new = pos + d.map(|c| i * c as i32);
                            if new.x < 0 || new.y < 0 {
                                all_seen = false;
                            } else {
                                let new = Point2::new(new.x as usize, new.y as usize);
                                if v.get(new.y).and_then(|v| v.get(new.x)) == Some(l) {
                                } else {
                                    all_seen = false;
                                }
                            }
                        }

                        if all_seen {
                            letter_a.entry(pos + d).and_modify(|a| *a += 1).or_insert(1);
                        }
                    }
                }
            }
        }

        letter_a.values().filter(|v| *v / 2 > 0).count()
    }
}

//crate::default_tests!((), ());
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
