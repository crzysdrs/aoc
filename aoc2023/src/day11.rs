use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 11;
    type Input1 = Vec<(i32, i32)>;
    type Input2 = Vec<(i64, i64)>;
    type Sol1 = i32;
    type Sol2 = i64;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut galaxies: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().flat_map(move |(x, c)| match c {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
            })
            .collect();

        let max_x = galaxies.iter().map(|g| g.0).max().unwrap();
        let max_y = galaxies.iter().map(|g| g.1).max().unwrap();

        let mut expand_x = vec![];
        let mut expand_y = vec![];

        for x in 0..max_x {
            if !galaxies.iter().any(|g| g.0 == x) {
                expand_x.push(x);
            }
        }
        for y in 0..max_y {
            if !galaxies.iter().any(|g| g.1 == y) {
                expand_y.push(y);
            }
        }

        //println!("{:?}", galaxies);
        galaxies.iter_mut().for_each(|g| {
            let grow_x = expand_x.partition_point(|x| g.0 > *x);
            g.0 += grow_x as i32;

            let grow_y = expand_y.partition_point(|y| g.1 > *y);
            g.1 += grow_y as i32;
        });
        //println!("{:?}", galaxies);
        galaxies
    }
    fn process_input2(s: &str) -> Self::Input2 {
        let mut galaxies: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().flat_map(move |(x, c)| match c {
                    '#' => Some((x as i64, y as i64)),
                    _ => None,
                })
            })
            .collect();

        let max_x = galaxies.iter().map(|g| g.0).max().unwrap();
        let max_y = galaxies.iter().map(|g| g.1).max().unwrap();

        let mut expand_x = vec![];
        let mut expand_y = vec![];

        for x in 0..max_x {
            if !galaxies.iter().any(|g| g.0 == x) {
                expand_x.push(x);
            }
        }
        for y in 0..max_y {
            if !galaxies.iter().any(|g| g.1 == y) {
                expand_y.push(y);
            }
        }

        let scale = 1000000 - 1;
        //println!("{:?}", galaxies);
        galaxies.iter_mut().for_each(|g| {
            let grow_x = expand_x.partition_point(|x| g.0 > *x);
            g.0 += grow_x as i64 * scale;

            let grow_y = expand_y.partition_point(|y| g.1 > *y);
            g.1 += grow_y as i64 * scale;
        });
        //println!("{:?}", galaxies);
        galaxies
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut dists = HashMap::new();
        for g1 in v {
            for g2 in v {
                if g1 == g2 {
                    continue;
                }

                let mh_dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
                dists.insert((std::cmp::min(g1, g2), std::cmp::max(g1, g2)), mh_dist);
            }
        }
        //println!("{:?}", dists);
        dists.values().sum::<i32>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut dists = HashMap::new();
        for g1 in v {
            for g2 in v {
                if g1 == g2 {
                    continue;
                }

                let mh_dist = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
                dists.insert((std::cmp::min(g1, g2), std::cmp::max(g1, g2)), mh_dist);
            }
        }
        //println!("{:?}", dists);
        dists.values().sum::<i64>()
    }
}

crate::default_tests!(10490062, 382979724122);
crate::string_tests!(
    [(
        foo_sol1,
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        374
    )],
    []
);
