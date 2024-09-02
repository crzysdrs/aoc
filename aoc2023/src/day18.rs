use crate::Day;
use cgmath::{Point2, Vector2};
use itertools::Itertools;
use rayon::prelude::*;

#[allow(unused_imports)]
use std::collections::*;

pub struct Dig {
    dir: Vector2<i32>,
    num: i32,
    _hex: String,
}

const UP: Vector2<i32> = Vector2::new(0, 1);
const DOWN: Vector2<i32> = Vector2::new(0, -1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 18;
    type Input1 = Vec<Dig>;
    type Input2 = Vec<Dig>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let (dir, rest) = l.split_once(' ').unwrap();
                let (num, hex) = rest.split_once(' ').unwrap();
                let dir = match dir {
                    "U" => UP,
                    "D" => DOWN,
                    "L" => LEFT,
                    "R" => RIGHT,
                    _ => panic!(),
                };
                let num = num.parse().unwrap();
                Dig {
                    dir,
                    num,
                    _hex: hex.to_string(),
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        s.lines()
            .map(|l| {
                let (_dir, rest) = l.split_once(' ').unwrap();
                let (_num, mut hex) = rest.split_once(' ').unwrap();
                hex = &hex[2..hex.len() - 1];

                let (num, dir) = hex.split_at(5);
                let num = i32::from_str_radix(num, 16).unwrap();
                let dir = match dir {
                    "3" => UP,
                    "1" => DOWN,
                    "2" => LEFT,
                    "0" => RIGHT,
                    v => panic!("Bogus number {}", v),
                };
                Dig {
                    dir,
                    num,
                    _hex: hex.to_string(),
                }
            })
            .collect()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        struct Digger {
            pos: Point2<i32>,
            dir: Vector2<i32>,
        }

        impl Digger {
            fn dig(&mut self, d: &Dig) -> impl Iterator<Item = Point2<i32>> {
                assert!(self.dir != d.dir);
                self.dir = d.dir;
                let mut pos = self.pos;
                let dir = self.dir;
                self.pos += self.dir * d.num;
                (0..d.num).map(move |_| {
                    pos += dir;
                    pos
                })
            }
        }

        let mut digger = Digger {
            pos: (0, 0).into(),
            dir: UP,
        };

        #[derive(Debug, Copy, Clone)]
        struct LineSegment {
            input: Vector2<i32>,
            output: Vector2<i32>,
        }
        //let pts: HashMap<_, _> = v.iter().flat_map(|dig| digger.dig(dig)).collect();
        let ordered_pts: Vec<_> = v.iter().flat_map(|dig| digger.dig(dig)).collect();
        //println!("{:#?}", ordered_pts);

        println!("Ordered points: {}", ordered_pts.len());

        let goto: Vec<_> = ordered_pts
            .iter()
            .circular_tuple_windows()
            .map(|(x, y, z)| {
                (
                    y,
                    LineSegment {
                        input: y - x,
                        output: y - z,
                    },
                )
            })
            .filter(|(p, l)| match l {
                LineSegment {
                    input: UP | DOWN,
                    output: UP | DOWN,
                } => false,
                _ => true,
            })
            .collect();

        println!("Goto: {}", goto.len());
        let goto: HashMap<_, _> = goto.into_iter().collect();

        let min_x = ordered_pts.iter().min_by_key(|p| p.x).unwrap().x;
        let max_x = ordered_pts.iter().max_by_key(|p| p.x).unwrap().x;
        let mut ys = goto.keys().fold(HashMap::new(), |mut h, p| {
            h.entry(p.x).or_insert(vec![]).push(p.y);
            h
        });

        //let goto: HashMap<_, _> = pts.iter().map(|(k, v)| (k + v, k)).collect();
        ys.par_iter_mut().for_each(|(_, v)| v.sort());

        let inside_count = (min_x..=max_x)
            .into_par_iter()
            .map(|x| {
                let mut inside_count: usize = 0;
                let ys = ys.get(&x).unwrap();

                let mut inside = false;
                let mut last_corner_dir = None;
                let mut last_cross = None;

                for y in ys.iter().copied() {
                    let pt = Point2::new(x, y);
                    //let target = pts.get(&pt).copied().unwrap();
                    let next = goto.get(&pt).copied().unwrap();
                    let old_inside = inside;
                    match next {
                        LineSegment {
                            input: LEFT | RIGHT,
                            output: LEFT | RIGHT,
                        } => {
                            //hit horizontal line
                            inside = !inside;
                            last_corner_dir = None;
                            //println!("Horizontal {:?}", pt);
                        }
                        LineSegment {
                            input: UP | DOWN,
                            output: UP | DOWN,
                        } => {
                            //on vertical line
                            //println!("Vertical {:?}", pt);
                        }
                        LineSegment {
                            input: UP | DOWN,
                            output: output @ (LEFT | RIGHT),
                        } => {
                            //println!("Corner {:?} : {:?}", pt, output);
                            //corner
                            match last_corner_dir {
                                None => {
                                    last_corner_dir = Some((old_inside, output));
                                    inside = true
                                }
                                Some((old, last_corner)) => {
                                    inside = if last_corner == output { old } else { !old };
                                    last_corner_dir = None;
                                }
                            }
                        }
                        LineSegment {
                            input: input @ (LEFT | RIGHT),
                            output: UP | DOWN,
                        } => {
                            //println!("Corner {:?} : {:?}", pt, input);
                            //corner
                            match last_corner_dir {
                                None => {
                                    last_corner_dir = Some((old_inside, input));
                                    inside = true;
                                }
                                Some((old, last_corner)) => {
                                    inside = if last_corner == input { old } else { !old };
                                    last_corner_dir = None;
                                }
                            }
                        }
                        d => panic!("Invalid vector {:?}", d),
                    };
                    if old_inside != inside {
                        //println!("Line Transition {:?}", pt);
                        match last_cross {
                            None => {
                                last_cross = Some(pt.y);
                                //if !inside {
                                //inside = !inside;
                                //}
                            }
                            Some(last) => {
                                let new = usize::try_from(pt.y - last).unwrap() + 1;
                                inside_count += new;
                                //println!("Counted {}", new);
                                last_cross = None;
                            }
                        }
                    }
                }
                inside_count
            })
            .sum();

        inside_count
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        Self::p1(v)
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        62
    )],
    [(
        foo_sol2,
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        952408144115
    )]
);
