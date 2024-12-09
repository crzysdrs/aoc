use crate::Day;
use cgmath::Point2;
#[allow(unused_imports)]
use std::collections::*;

pub enum Pos {
    Antenna(char),
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 8;
    type Input1 = (usize, HashMap<Point2<i32>, Pos>);
    type Input2 = (usize, HashMap<Point2<i32>, Pos>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let w = s.lines().count();
        (
            w,
            s.lines()
                .enumerate()
                .flat_map(|(y, s)| {
                    s.chars().enumerate().flat_map(move |(x, c)| {
                        let pos = Point2::new(x as i32, y as i32);
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' => Some((pos, Pos::Antenna(c))),
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
    fn p1((size, map): &Self::Input1) -> Self::Sol1 {
        let nodes = map
            .iter()
            .flat_map(|(k, v)| match v {
                Pos::Antenna(n) => Some((n, k)),
            })
            .fold(HashMap::<char, Vec<Point2<i32>>>::new(), |mut state, v| {
                state
                    .entry(*v.0)
                    .and_modify(|l| l.push(*v.1))
                    .or_insert_with(|| vec![*v.1]);
                state
            });

        let mut pairs = vec![];
        for (_n, l) in &nodes {
            for n1 in l {
                for n2 in l {
                    if n1 == n2 {
                        continue;
                    }
                    pairs.push((n1, n2));
                }
            }
        }

        let mut antinode = HashSet::new();
        for (p1, p2) in pairs {
            let diff = p1 - p2;
            antinode.insert(p1 + diff);
            antinode.insert(p2 - diff);
        }
        antinode
            .iter()
            .filter(|p| (0..*size as i32).contains(&p.x) && (0..*size as i32).contains(&p.y))
            .count()
    }
    fn p2((size, map): &Self::Input2) -> Self::Sol2 {
        let nodes = map
            .iter()
            .flat_map(|(k, v)| match v {
                Pos::Antenna(n) => Some((n, k)),
            })
            .fold(HashMap::<char, Vec<Point2<i32>>>::new(), |mut state, v| {
                state
                    .entry(*v.0)
                    .and_modify(|l| l.push(*v.1))
                    .or_insert_with(|| vec![*v.1]);
                state
            });

        let mut pairs = vec![];
        for (_n, l) in &nodes {
            for n1 in l {
                for n2 in l {
                    if n1 == n2 {
                        continue;
                    }
                    pairs.push((n1, n2));
                }
            }
        }

        let valid_pt =
            |p: &Point2<i32>| (0..*size as i32).contains(&p.x) && (0..*size as i32).contains(&p.y);

        let mut antinode = HashSet::new();
        for (p1, p2) in pairs {
            let diff = p1 - p2;
            {
                let mut node = p1 + diff;
                while valid_pt(&node) {
                    antinode.insert(node);
                    node += diff;
                }
            }
            {
                let mut node = p1 - diff;
                while valid_pt(&node) {
                    antinode.insert(node);
                    node -= diff;
                }
            }
        }
        antinode
            .iter()
            .filter(|p| (0..*size as i32).contains(&p.x) && (0..*size as i32).contains(&p.y))
            .count()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        14
    )],
    [(
        foo_sol2,
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
        34
    )]
);
