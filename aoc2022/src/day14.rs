use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

enum Particle {
    StillSand,
    Stone,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 14;
    type Input1 = Vec<Vec<Point2<i32>>>;
    type Input2 = Vec<Vec<Point2<i32>>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|coord| {
                        let (x, y) = coord.split_once(',').unwrap();
                        Point2::new(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect()
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut sandbox = HashMap::new();

        v.iter().for_each(|r| {
            r.windows(2).for_each(|w| {
                let mut diff = w[1] - w[0];
                use std::cmp::Ordering;
                diff.x = match diff.x.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                };

                diff.y = match diff.y.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                };
                let mut cur = w[0];
                while cur != w[1] {
                    sandbox.insert(cur, Particle::Stone);
                    cur += diff;
                }
                sandbox.insert(cur, Particle::Stone);
            });
        });

        let lowest_stone = sandbox.keys().max_by_key(|s| s.y).cloned().unwrap();
        let mut sand_inserted = 0;
        'sand_done: loop {
            let mut new_sand = Point2::new(500, 0);
            let down = Vector2::new(0, 1);
            let down_left = down + Vector2::new(-1, 0);
            let down_right = down + Vector2::new(1, 0);

            let options = [down, down_left, down_right];

            'insert_sand: loop {
                if let Some(pos) = options
                    .iter()
                    .map(|dir| new_sand + dir)
                    .find(|pos| !sandbox.contains_key(pos))
                {
                    if pos.y > lowest_stone.y {
                        break 'sand_done;
                    }
                    new_sand = pos;
                } else {
                    sandbox.insert(new_sand, Particle::StillSand);
                    sand_inserted += 1;
                    break 'insert_sand;
                }
            }
        }
        sand_inserted
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut sandbox = HashMap::new();

        v.iter().for_each(|r| {
            r.windows(2).for_each(|w| {
                let mut diff = w[1] - w[0];
                use std::cmp::Ordering;
                diff.x = match diff.x.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                };

                diff.y = match diff.y.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                };
                let mut cur = w[0];
                while cur != w[1] {
                    sandbox.insert(cur, Particle::Stone);
                    cur += diff;
                }
                sandbox.insert(cur, Particle::Stone);
            });
        });

        let lowest_stone = sandbox.keys().max_by_key(|s| s.y).cloned().unwrap();
        let mut sand_inserted = 0;
        'sand_done: loop {
            let mut new_sand = Point2::new(500, 0);
            let down = Vector2::new(0, 1);
            let down_left = down + Vector2::new(-1, 0);
            let down_right = down + Vector2::new(1, 0);

            let options = [down, down_left, down_right];

            'insert_sand: loop {
                let next_pos = options.iter().map(|dir| new_sand + dir).find(|pos| {
                    if pos.y >= lowest_stone.y + 2 {
                        false
                    } else {
                        !sandbox.contains_key(pos)
                    }
                });
                if let Some(pos) = next_pos {
                    new_sand = pos;
                } else {
                    sandbox.insert(new_sand, Particle::StillSand);
                    sand_inserted += 1;
                    if new_sand == Point2::new(500, 0) {
                        break 'sand_done;
                    }
                    break 'insert_sand;
                }
            }
        }
        sand_inserted
    }
}

crate::default_tests!(1406, 20870);
crate::path_tests!([(t1, "test/day14.txt", 24)], [(t2, "test/day14.txt", 93)]);
