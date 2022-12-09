use crate::Day;
use cgmath::{Point2, Vector2};
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn offset(&self, v: i32) -> Vector2<i32> {
        match self {
            Dir::Up => Vector2::new(0, v),
            Dir::Down => Vector2::new(0, -v),
            Dir::Left => Vector2::new(-v, 0),
            Dir::Right => Vector2::new(v, 0),
        }
    }
}

fn movement(_old_head: Point2<i32>, head: Point2<i32>, mut tail: Point2<i32>) -> Point2<i32> {
    let diff = head - tail;

    let lr = diff.x.abs() > 1;
    let ud = diff.y.abs() > 1;

    if lr && diff.y == 0 {
        tail.x += if diff.x > 0 { 1 } else { -1 }
    } else if ud && diff.x == 0 {
        tail.y += if diff.y > 0 { 1 } else { -1 }
    } else if diff.y.abs() >= 2 {
        tail.x += if diff.x > 0 { 1 } else { -1 };
        tail.y += if diff.y > 0 { 1 } else { -1 }
    } else if diff.x.abs() >= 2 {
        tail.y += if diff.y > 0 { 1 } else { -1 };
        tail.x += if diff.x > 0 { 1 } else { -1 }
    }

    tail
}

#[allow(unused)]
fn disp_knots(v: &[Point2<i32>]) {
    let (min_x, max_x) = v.iter().map(|k| k.x).minmax().into_option().unwrap();
    let (min_y, max_y) = v.iter().map(|k| k.y).minmax().into_option().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if let Some(p) = v.iter().position(|p| *p == Point2::new(x, y)) {
                let c = if p == 0 {
                    'H'
                } else {
                    char::from_digit(p as u32, 10).unwrap()
                };
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9;
    type Input1 = Vec<(Dir, i32)>;
    type Input2 = Vec<(Dir, i32)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let (dir, count) = l.split_once(' ').unwrap();
                let dir = match dir {
                    "U" => Dir::Up,
                    "D" => Dir::Down,
                    "L" => Dir::Left,
                    "R" => Dir::Right,
                    _ => panic!(),
                };

                (dir, count.parse().unwrap())
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut head = Point2::new(0, 0);
        let mut tail = Point2::new(0, 0);
        let mut tail_pos: Vec<_> = v
            .iter()
            .flat_map(|(d, v)| (0..*v).map(move |_new_v| (d, 1)))
            .map(|(d, _)| {
                let old_head = head;
                head += d.offset(1);
                //println!("{:?}", head);
                tail = movement(old_head, head, tail);
                tail
            })
            .collect();

        tail_pos.sort_by_key(|p| (p.x, p.y));
        tail_pos.dedup();

        tail_pos.len()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut knots = vec![Point2::new(0, 0); 10];

        let mut tail_pos: Vec<_> = v
            .iter()
            .flat_map(|(d, v)| {
                println!("{:?}", (d, v));
                (0..*v).map(move |_new_v| (d, 1))
            })
            .map(|(d, _)| {
                let old_head = knots[0];
                knots[0] += d.offset(1);
                let new_knots: Vec<_> = knots
                    .iter()
                    .skip(1)
                    .scan((old_head, knots[0]), |(old_head, head), tail| {
                        let new_tail = movement(*old_head, *head, *tail);
                        assert!((head.x - new_tail.x).abs() <= 1);
                        assert!((head.y - new_tail.y).abs() <= 1);
                        *old_head = *tail;
                        *head = new_tail;
                        Some(new_tail)
                    })
                    .collect();
                let new_tail = *new_knots.last().unwrap();
                knots.resize(1, Point2::new(0, 0));
                knots.extend(new_knots.iter());
                //disp_knots(&knots);
                //println!("==================");
                new_tail
            })
            .collect();

        tail_pos.sort_by_key(|p| (p.x, p.y));
        tail_pos.dedup();

        tail_pos.len()
    }
}

crate::default_tests!(5907, 2303);
crate::path_tests!(
    [(t1, "test/day09.txt", 13)],
    [(t2, "test/day09.txt", 1), (t3, "test/day09_2.txt", 36)]
);
