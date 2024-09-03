use crate::Day;
use cgmath::{Point3, Vector3};
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}

#[derive(Debug, PartialEq)]
pub struct Faces(usize, usize);

impl core::fmt::Display for Faces {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Face A: {} Face B: {}", self.0, self.1)
    }
}
impl Day for Solution {
    const DAY: u32 = 18;
    type Input1 = Vec<Point3<i32>>;
    type Input2 = Vec<Point3<i32>>;
    type Sol1 = usize;
    type Sol2 = Faces;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                let mut items = x.split(',').map(|x| x.parse().unwrap());
                Point3::new(
                    items.next().unwrap(),
                    items.next().unwrap(),
                    items.next().unwrap(),
                )
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let dirs = [
            Vector3::new(1, 0, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, 0, 1),
            Vector3::new(-1, 0, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 0, -1),
        ];

        let set = v.iter().cloned().collect::<HashSet<Point3<_>>>();

        set.iter()
            .flat_map(|x| {
                let set = &set;
                dirs.iter()
                    .map(move |d| usize::from(set.get(&(x + d)).is_none()))
            })
            .sum::<usize>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let dirs = [
            Vector3::new(1, 0, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, 0, 1),
            Vector3::new(-1, 0, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 0, -1),
        ];

        let set = v.iter().cloned().collect::<HashSet<Point3<_>>>();

        let edges: HashSet<_> = set
            .iter()
            .flat_map(|x| {
                let set = &set;
                dirs.iter().flat_map(move |d| {
                    if set.get(&(x + d)).is_none() {
                        Some(x + d)
                    } else {
                        None
                    }
                })
            })
            .collect();

        let mut worklist = vec![edges.iter().next().cloned().unwrap()];
        let mut side = HashSet::new();
        use itertools::Itertools;
        let all_dirs: Vec<_> = (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|((x, y), z)| Vector3::new(x, y, z))
            .filter(|v| !(v.x == 0 && v.y == 0 && v.z == 0))
            .collect();

        while let Some(w) = worklist.pop() {
            if side.contains(&w) {
                continue;
            }
            side.insert(w);
            if all_dirs.iter().all(|d| !set.contains(&(w + d))) {
                continue;
            }
            worklist.extend(dirs.iter().flat_map(|d| {
                if edges.contains(&(w + d)) || !set.contains(&(w + d)) {
                    Some(w + d)
                } else {
                    None
                }
            }));
        }

        let face_a = set
            .iter()
            .flat_map(|x| {
                let side = &side;
                dirs.iter()
                    .map(move |d| usize::from(side.get(&(x + d)).is_some()))
            })
            .sum::<usize>();

        let face_b = Self::p1(v) - face_a;
        Faces(std::cmp::min(face_a, face_b), std::cmp::max(face_a, face_b))
    }
}

crate::default_tests!(3494, Faces(3494 - 2062, 2062));
crate::path_tests!(
    [(t1, "test/day18.txt", 64)],
    [(t2, "test/day18.txt", Faces(6, 58))]
);
