use crate::Day;
use cgmath::{Point2, Point3, Vector2, Vector3};

#[derive(PartialEq, Debug)]
struct Line {
    m: f32,
    b: f32,
}

#[derive(PartialEq, Debug)]
enum Intersect {
    Parallel,
    AtX(f32),
    Equal,
}
impl Line {
    fn from_points(p1: Point2<i64>, p2: Point2<i64>) -> Line {
        let m = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
        // y = mx + b
        // y - b = mx
        // b = mx - y
        // b = y - mx

        let b = p2.y as f32 - m * p2.x as f32;
        Line { m, b }
    }
    fn intersect(&self, p: &Line) -> Intersect {
        // m1 x + b1 = m2 x + b2
        // m1 x + b1 - b2 = m2 x
        // b1 - b2 = m2 x - m1 x
        // (b1 - b2) / (m2 - m1) = x
        if self == p {
            Intersect::Equal
        } else if self.m == p.m {
            Intersect::Parallel
        } else {
            Intersect::AtX((p.b - self.b) / (self.m - p.m))
        }
    }

    fn y_value(&self, x: f32) -> f32 {
        self.m * x + self.b
    }
}

fn truncate_z<T>(p: Point3<T>) -> Point2<T> {
    Point2::new(p.x, p.y)
}

#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 24;
    type Input1 = Vec<(Point3<i64>, Vector3<i64>)>;
    type Input2 = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let (pos, vel) = s.split_once('@').unwrap();
                fn tup3(s: &str) -> (i64, i64, i64) {
                    let mut t = s.trim().split(',').map(|v| v.trim().parse().unwrap());
                    (t.next().unwrap(), t.next().unwrap(), t.next().unwrap())
                }

                (Point3::from(tup3(pos)), Vector3::from(tup3(vel)))
            })
            .collect()
    }
    fn process_input2(_s: &str) -> Self::Input2 {
        unimplemented!()
    }
    fn p1(vs: &Self::Input1) -> Self::Sol1 {
        let mut count = 0;
        for (i, v) in vs.iter().enumerate() {
            for v2 in vs[i + 1..].iter() {
                let l1 = Line::from_points(truncate_z(v.0), truncate_z(v.0 + v.1));
                let l2 = Line::from_points(truncate_z(v2.0), truncate_z(v2.0 + v2.1));

                //println!("{:?}", v);
                //println!("{:?}", v2);

                //let range = 7.0..=27.0;
                let range = 200000000000000f32..=400000000000000f32;

                count += match l1.intersect(&l2) {
                    Intersect::Parallel => {
                        println!("Parallel");
                        0
                    }
                    Intersect::Equal => 0,
                    Intersect::AtX(x) => {
                        let y = l2.y_value(x);
                        println!("Intersect at {}, {}", x, y);
                        let past_a =
                            (v.1.x < 0 && x > v.0.x as f32) || (v.1.x > 0 && x < v.0.x as f32);
                        let past_b =
                            (v2.1.x < 0 && x > v2.0.x as f32) || (v2.1.x > 0 && x < v2.0.x as f32);
                        if past_a && past_b {
                            println!("In the past for both");
                            0
                        } else if past_a {
                            println!("In the past for A");
                            0
                        } else if past_b {
                            println!("In the past for B");
                            0
                        } else if range.contains(&x) && range.contains(&y) {
                            println!("Inside");
                            1
                        } else {
                            println!("Outside");
                            0
                        }
                    }
                };
            }
        }
        //9904 too low
        //27820 too high
        count
    }
    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        unimplemented!()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
",
        2
    )],
    [
//(foo_sol2, "hi2", 1)
]
);
