use crate::Day;
use cgmath::prelude::*;
use cgmath::Point3;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 8;
    type Input1 = Vec<Point3<i64>>;
    type Input2 = Vec<Point3<i64>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let mut p = s.split(',').map(|p| p.parse().unwrap());

                Point3::new(p.next().unwrap(), p.next().unwrap(), p.next().unwrap())
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut dist = Vec::new();
        for (i, p) in v.iter().enumerate() {
            for (j, p2) in v[i + 1..].iter().enumerate() {
                dist.push(((*p, *p2), (p - p2).magnitude2()));
            }
        }
        dist.sort_by_key(|v| std::cmp::Reverse(v.1));

        let mut conns = HashSet::new();
        for _i in 0..1000 {
            let pts = dist.pop().unwrap();
            println!("{:?}", pts);
            conns.insert(pts.0);
        }

        let visit: HashSet<_> = conns.iter().flat_map(|(a, b)| [a, b].into_iter()).collect();
        let mut visit: Vec<_> = visit.into_iter().collect();

        let mut visited = HashSet::new();
        let mut circuit_sizes = vec![];
        while let Some(v) = visit.pop() {
            if visited.contains(v) {
                continue;
            }
            let mut circuit = HashSet::new();
            let mut search = vec![v];
            while let Some(v) = search.pop() {
                conns
                    .iter()
                    .filter(|(p1, p2)| p1 == v || p2 == v)
                    .for_each(|(p1, p2)| {
                        if !circuit.contains(p1) {
                            circuit.insert(p1);
                            search.push(p1);
                        }
                        if !circuit.contains(p2) {
                            circuit.insert(p2);
                            search.push(p2);
                        }
                    });
            }

            for c in &circuit {
                visited.insert(*c);
            }
            circuit_sizes.push(circuit.len());
        }
        circuit_sizes.sort();
        circuit_sizes.reverse();

        println!("{:?}", circuit_sizes);
        circuit_sizes.iter().take(3).product()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut dist = Vec::new();
        for (i, p) in v.iter().enumerate() {
            for (j, p2) in v[i + 1..].iter().enumerate() {
                dist.push(((*p, *p2), (p - p2).magnitude2()));
            }
        }
        dist.sort_by_key(|v| std::cmp::Reverse(v.1));

        fn bin_search(
            v: &Vec<Point3<i64>>,
            dist: &Vec<((Point3<i64>, Point3<i64>), i64)>,
            size: usize,
        ) -> Option<(Point3<i64>, Point3<i64>)> {
            let mut dist = dist.clone();
            let mut conns = HashSet::new();
            let mut last_conn = None;
            for _i in 0..std::cmp::min(size, dist.len()) {
                let pts = dist.pop().unwrap();
                //println!("{:?}", pts);
                last_conn = Some(pts.0);
                conns.insert(pts.0);
            }

            let visit: HashSet<_> = conns.iter().flat_map(|(a, b)| [a, b].into_iter()).collect();
            let mut visit: Vec<_> = visit.into_iter().collect();
            if visit.len() != v.len() {
                return None;
            }
            let mut visited = HashSet::new();
            let mut circuit_sizes = vec![];
            while let Some(v) = visit.pop() {
                if visited.contains(v) {
                    continue;
                }
                let mut circuit = HashSet::new();
                let mut search = vec![v];
                while let Some(v) = search.pop() {
                    conns
                        .iter()
                        .filter(|(p1, p2)| p1 == v || p2 == v)
                        .for_each(|(p1, p2)| {
                            if !circuit.contains(p1) {
                                circuit.insert(p1);
                                search.push(p1);
                            }
                            if !circuit.contains(p2) {
                                circuit.insert(p2);
                                search.push(p2);
                            }
                        });
                }

                for c in &circuit {
                    visited.insert(*c);
                }
                circuit_sizes.push(circuit.len());
            }
            circuit_sizes.sort();
            circuit_sizes.reverse();

            //println!("{:?}", circuit_sizes);
            if circuit_sizes.len() == 1 {
                last_conn
            } else {
                None
            }
        }

        let mut l = 0;
        let mut r = dist.len();

        while l <= r {
            let mid = (l + r) / 2;
            println!("{:?}", (l, r, mid));
            if bin_search(&v, &dist, mid).is_some() {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        //println!("{:?}", bin_search(&v, &dist, l - 1));
        let last = bin_search(&v, &dist, l).unwrap();
        (last.0.x * last.1.x) as usize
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        40
    )],
    [(
        foo_sol2,
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        25272
    )]
);
