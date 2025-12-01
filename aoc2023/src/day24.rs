use crate::Day;
use cgmath::{Point2, Point3, Vector3};

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
    #[allow(unused)]
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

#[allow(unused)]
fn truncate_x<T>(p: Point3<T>) -> Point2<T> {
    Point2::new(p.y, p.z)
}

#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 24;
    type Input1 = Vec<(Point3<i64>, Vector3<i64>)>;
    type Input2 = Vec<(Point3<i64>, Vector3<i64>)>;
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
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
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
    fn p2(vs: &Self::Input2) -> Self::Sol2 {
        #[derive(Debug, PartialEq)]
        struct Line3d {
            b: Point3<i64>,
            m: Vector3<i64>,
        }

        impl Line3d {
            #[allow(unused)]
            fn intersect(&self, other: &Line3d) -> bool {
                let l_xy_1 = Line::from_points(truncate_z(self.b), truncate_z(self.b + self.m));
                let l_xy_2 = Line::from_points(truncate_z(other.b), truncate_z(other.b + other.m));

                let l_yz_1 = Line::from_points(truncate_x(self.b), truncate_x(self.b + self.m));
                let l_yz_2 = Line::from_points(truncate_x(other.b), truncate_x(other.b + other.m));

                match (l_xy_1.intersect(&l_xy_2), l_yz_1.intersect(&l_yz_2)) {
                    (Intersect::AtX(x), Intersect::AtX(y)) if x == y => true,
                    _ => false,
                }
            }
            #[allow(unused)]
            fn at_t(&self, t: i64) -> Point3<i64> {
                self.b + self.m * t
            }
            fn solve_t(&self, other: &Line3d) -> Option<i64> {
                // Mt + b = M2t + b2
                // Mt - M2t = b2 - b
                // t(M - M2) = b2 - b
                // t = (b2 - b) / (M - M2)

                if self == other {
                    println!("Same line?");
                }
                if AsRef::<[_; 3]>::as_ref(&(self.m - other.m))
                    .iter()
                    .any(|v| *v == 0)
                {
                    return None;
                }
                let t_x = (other.b.x - self.b.x) / (self.m.x - other.m.x);
                let t_y = (other.b.y - self.b.y) / (self.m.y - other.m.y);
                let t_z = (other.b.z - self.b.z) / (self.m.z - other.m.z);

                //println!("{:?}", (t_x, t_y, t_z));
                //println!("{:?}", (self.at_t(t_x), self.at_t(t_y), self.at_t(t_z)));

                if t_x == t_y && t_y == t_z {
                    //println!("All same");
                    return Some(t_x);
                }
                None
            }
        }
        let vs: Vec<_> = vs.iter().map(|b| Line3d { b: b.0, m: b.1 }).collect();

        //let mut pts = HashMap::new();

        let mut lines = vec![];
        for (i, a) in vs.iter().enumerate() {
            for (j, b) in vs.iter().enumerate() {
                use cgmath::EuclideanSpace;
                let line_diff = Line3d {
                    m: b.m - a.m,
                    b: Point3::from_vec(b.b - a.b + b.m),
                };

                if i != j {
                    //for k in 1..vs.len() + 1 {
                    //*pts.entry(line_diff.at_t(k as i64)).or_insert(0) += 1;
                    //}
                    lines.push(line_diff);
                }
            }
        }
        for (i, a) in lines.iter().enumerate() {
            for (j, b) in lines.iter().enumerate() {
                if i != j {
                    if let Some(v) = a.solve_t(b) {
                        println!("{}:{} {:?}", i, j, v);
                    }
                }
            }
        }
        // let mut all: Vec<_> = pts.iter().collect();
        // all.sort_by_key(|(k, v)| *v);
        // println!("{:?}", &all[all.len() - 10..]);

        // [5,2,1] t + [3,2,1] = pos
        // 5t + 3 = x
        // t = (x - 3) /5
        // 2t + 2 = y
        // 1t + 1 = z

        // 2(x-3)/5 + 2 = y
        // 2/5x - 6/5 + 2 = y
        // 2/5x - 1y + 0z - (6/5 + 2) = 0

        // (t+1)hail_bm + hail_bb - (t(hail_am) + hail_ab)
        // thail_bm + hail_bm + hail_bb - thail_am - hail_ab
        // t(hail_bm - hail_am) + hail_bm + hail_b - hail_ab

        // t(hail_am) + hail_ab = (t + 1)(hail_bm) + hail_bb
        //  t(hail_am) + hail_ab = t hail_bm + hail_bm + hail_bb
        //  thail_am - t_hail_bm = hail_bm + hail_bb - hail_ab
        // t = ( hail_bm + hail_bb - hail_ab) / (hail_am - t hail_bm)
        // t(hail_am) + hail_ab = 0

        // t(rock_m) + rock_b = t(hail_m) + hail_b
        // t(rock_m) - t(hail_m) = hail_b - rock_b
        // t = (hail_b - rock_b) / (rockm - hail_m)
        // [xd, yd, zd]t + [x0, y0, z0] = pos
        // xdt + x0 = x_pos
        // t = (x_pos - x_0) / xd

        // yd((x_pos - x_0) / xd) = y_pos
        // (x_pos - x0) / xd = y_pos / yd
        //  x_pos - x0 = xd * y_pos / yd
        // x_pos = xd * y_pos / yd  + x0

        // ydt + y0 = y pos
        // zdt + z0 = z pos
        // zdt = zpos - z0

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
    [(
        foo_sol2,
        "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
",
        47
    )]
);
