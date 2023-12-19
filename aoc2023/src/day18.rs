use crate::Day;
use cgmath::{Point2, Vector2};
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
            fn dig(&mut self, d: &Dig) -> impl Iterator<Item = (Point2<i32>, Vector2<i32>)> {
                self.dir = d.dir;
                let mut pos = self.pos;
                let dir = self.dir;
                self.pos += self.dir * d.num;

                (0..d.num).map(move |_| {
                    pos += dir;
                    (pos, dir)
                })
            }
        }

        let mut digger = Digger {
            pos: (0, 0).into(),
            dir: UP,
        };

        let pts: HashMap<_, _> = v.iter().flat_map(|dig| digger.dig(dig)).collect();

        let min_x = pts.keys().min_by_key(|p| p.x).unwrap().x;
        let max_x = pts.keys().max_by_key(|p| p.x).unwrap().x;

        let min_y = pts.keys().min_by_key(|p| p.y).unwrap().y;
        let max_y = pts.keys().max_by_key(|p| p.y).unwrap().y;

        println!("{:?}", (min_x, min_y));
        println!("{:?}", (max_x, max_y));

        let mut ys = pts.keys().fold(HashMap::new(), |mut h, p| {
            h.entry(p.x).or_insert(vec![]).push(p.y);
            h
        });

        let goto: HashMap<_, _> = pts.iter().map(|(k, v)| (k + v, k)).collect();

        ys.values_mut().for_each(|v| v.sort());

        let mut inside_count: usize = 0;
        for x in min_x..=max_x {
            let ys = ys.get(&x).unwrap();
            let mut inside = false;
            let mut last_line = None;
            for y in ys.iter().copied() {
                let pt = Point2::new(x, y);
                let target = pts.get(&pt).copied().unwrap();
                let real_next = Point2::new(0, 0) + v[0].dir;
                let _next = goto.get(&pt).copied().unwrap_or(&real_next);
                let _prev = pt + target;
                let line = match pts.get(&(pt + LEFT)).copied() {
                    Some(LEFT) => true,
                    _ => false,
                } || match pts.get(&(pt)).copied() {
                    Some(RIGHT) => true,
                    _ => false,
                };
                //&& (pts.contains(&(x - 1, y).into()) || pts.contains(&(x + 1, y).into()));
                if line {
                    inside = !inside;
                }
                if line && !inside {
                    //println!("{:?}", (x, y));
                    inside_count += 1 + usize::try_from(y - last_line.unwrap()).unwrap();
                    println!(
                        "# {:?} {}",
                        (x, last_line.unwrap()..y),
                        1 + y - last_line.unwrap(),
                    );
                }
                if line {
                    last_line = Some(y);
                }
            }
        }

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
