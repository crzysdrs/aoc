use crate::Day;
use cgmath::{Point2, Vector2};
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Data {
    sensor: Point2<i32>,
    beacon: Point2<i32>,
}
#[derive(PartialEq)]
enum Map {
    Beacon,
    Sensor,
    Nothing,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 15;
    type Input1 = Vec<Data>;
    type Input2 = Vec<Data>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let re = Regex::new(
            r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
        )
        .unwrap();

        s.lines()
            .map(|l| {
                let caps = re.captures(l).unwrap();
                Data {
                    sensor: Point2::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                    beacon: Point2::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut map = HashMap::new();

        fn manhattan(p1: Vector2<i32>) -> i32 {
            p1.x.abs() + p1.y.abs()
        }
        v.iter().for_each(|d| {
            map.insert(d.beacon, Map::Beacon);
            map.insert(d.sensor, Map::Sensor);
        });

        let target = 2000000;
        //let target = 10;
        for e in v {
            let dist = manhattan(e.sensor - e.beacon);

            let y_range = e.sensor.y - dist..=e.sensor.y + dist;
            for (y, i) in y_range
                .clone()
                .zip((0..dist).chain((0..=dist).rev()))
                .filter(|(y, _)| *y == target)
            {
                let x_range = e.sensor.x - i as i32..=e.sensor.x + i as i32;
                //println!("{:?} {:?}", x_range, y_range);
                for x in x_range {
                    map.entry(Point2::new(x, y)).or_insert(Map::Nothing);
                }
            }
        }

        map.iter()
            .filter(|(p, v)| p.y == target && **v == Map::Nothing)
            .count()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        // This may not be a fully general solution.
        // It computes the shells of the diameonds made by sensors
        // and then effectively computes the intersections by adding the shells together.
        // The location with the most intersectiosn is effectively the "least seen" location
        // therefore the location that no one can see. If it's not on the border, it will
        // not be seen by 4 sensors.

        // I suspect this would fail if the location to find was on the border of the searched
        // region.

        let mut map = HashMap::new();

        fn manhattan(p1: Vector2<i32>) -> i32 {
            p1.x.abs() + p1.y.abs()
        }
        // v.iter().for_each(|d| {
        //     map.insert(d.beacon, 1);
        //     map.insert(d.sensor, 1);
        // });

        //let target = 10;
        for e in v {
            let dist = manhattan(e.sensor - e.beacon);

            let border_dist = dist + 1;
            let offsets: Vec<_> = [
                Vector2::new(0, border_dist),
                Vector2::new(border_dist, 0),
                Vector2::new(0, -border_dist),
                Vector2::new(-border_dist, 0),
                Vector2::new(0, border_dist),
            ]
            .into_iter()
            .map(|v| e.sensor + v)
            .collect();

            offsets.windows(2).for_each(|win| {
                let mut line = win[1] - win[0];
                let mut cur = win[0];
                line.x = match line.x.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };
                line.y = match line.y.cmp(&0) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };
                while cur != win[1] {
                    *map.entry(cur).or_insert(0) += 1;
                    cur += line;
                }
            });
        }

        let max = 4000000;
        let found = map
            .iter()
            .filter(|(p, _v)| p.y <= max && p.x <= max && p.y >= 0 && p.x >= 0)
            .max_by_key(|(_p, v)| **v)
            .unwrap();
        found.0.x as usize * max as usize + found.0.y as usize
    }
}

crate::default_tests!(4985193, 11583882601918);
