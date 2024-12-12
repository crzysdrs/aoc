use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

pub const LEFT: Vector2<i32> = Vector2::new(-1, 0);
pub const RIGHT: Vector2<i32> = Vector2::new(1, 0);
pub const UP: Vector2<i32> = Vector2::new(0, 1);
pub const DOWN: Vector2<i32> = Vector2::new(0, -1);

#[derive(Debug)]
struct Region {
    plant: char,
    start: Point2<i32>,
    points: Vec<Point2<i32>>,
}

impl Region {
    fn area(&self) -> usize {
        self.points.len()
    }
    fn perimeter(&self, pts: &HashMap<Point2<i32>, char>) -> usize {
        self.points
            .iter()
            .map(|p| {
                let dirs = [UP, DOWN, LEFT, RIGHT];
                let mut edge = 0;
                for d in dirs {
                    match pts.get(&(p + d)) {
                        Some(c) if *c != self.plant => {
                            edge += 1;
                        }
                        Some(_) => {}
                        None => {
                            edge += 1;
                        }
                    }
                }
                edge
            })
            .sum()
    }

    fn sides(&self, pts: &HashMap<Point2<i32>, char>) -> usize {
        #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
        enum EdgeDir {
            LR,
            UD,
        }

        let edges: HashSet<_> = self
            .points
            .iter()
            .flat_map(|p| {
                let dirs = [
                    (UP, EdgeDir::LR),
                    (DOWN, EdgeDir::LR),
                    (LEFT, EdgeDir::UD),
                    (RIGHT, EdgeDir::UD),
                ];

                let mut edges = vec![];
                for (d, ed) in dirs {
                    let new_p = p + d;
                    match pts.get(&new_p) {
                        Some(c) if *c != self.plant => {
                            edges.push((new_p, ed, d));
                        }
                        Some(_) => {}
                        None => {
                            edges.push((new_p, ed, d));
                        }
                    }
                }
                edges.into_iter()
            })
            .collect();

        let mut seen = HashSet::new();

        let mut sides = 0;
        for e1 in &edges {
            if seen.get(e1).is_none() {
                sides += 1;
                let mut worklist = VecDeque::new();
                worklist.push_front(e1.clone());
                seen.insert(e1.clone());

                let dirs = match e1.1 {
                    EdgeDir::LR => [RIGHT, LEFT],
                    EdgeDir::UD => [UP, DOWN],
                };
                while let Some(w) = worklist.pop_front() {
                    for d in &dirs {
                        let new_p = w.0 + d;
                        let search = (new_p, e1.1, e1.2);
                        if seen.get(&search).is_none() && edges.get(&search).is_some() {
                            seen.insert(search);
                            worklist.push_back(search.clone());
                        }
                    }
                }
            }
        }
        sides
    }
    fn flood_fill(&mut self, pts: &HashMap<Point2<i32>, char>, seen: &mut HashSet<Point2<i32>>) {
        let dirs = [UP, DOWN, LEFT, RIGHT];
        let mut worklist = VecDeque::new();
        worklist.push_front(self.start);
        self.points.push(self.start);
        seen.insert(self.start);
        while let Some(p) = worklist.pop_front() {
            for d in dirs {
                let new_p = p + d;
                if pts.get(&new_p) == Some(&self.plant) && seen.get(&new_p).is_none() {
                    seen.insert(new_p);
                    worklist.push_back(new_p);
                    self.points.push(new_p);
                }
            }
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 12;
    type Input1 = HashMap<Point2<i32>, char>;
    type Input2 = HashMap<Point2<i32>, char>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter(|(_x, c)| *c != '.')
                    .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut seen = HashSet::new();

        let mut regions = vec![];
        for (p, c) in v {
            if seen.get(p).is_none() {
                let mut r = Region {
                    plant: *c,
                    start: *p,
                    points: vec![],
                };

                r.flood_fill(v, &mut seen);
                regions.push(r);
            }
        }
        regions.iter().map(|r| r.perimeter(&v) * r.area()).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut seen = HashSet::new();

        let mut regions = vec![];
        for (p, c) in v {
            if seen.get(p).is_none() {
                let mut r = Region {
                    plant: *c,
                    start: *p,
                    points: vec![],
                };

                r.flood_fill(v, &mut seen);
                regions.push(r);
            }
        }

        regions.iter().map(|r| r.sides(&v) * r.area()).sum()
    }
}

crate::default_tests!(1494342, 893676);
crate::string_tests!(
    [(
        foo_sol1,
        "AAAA
BBCD
BBCC
EEEC",
        140
    )],
    [(
        foo_sol2,
        "AAAA
BBCD
BBCC
EEEC",
        80
    )]
);
