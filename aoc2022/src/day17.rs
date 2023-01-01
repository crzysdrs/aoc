use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Obj {
    Rock,
    Empty,
}

#[derive(PartialEq, Debug)]
struct Grid {
    size_x: usize,
    size_y: usize,
    vals: Vec<Obj>,
}

impl core::str::FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Grid, ()> {
        let size_y = s.lines().count();
        let size_x = s.lines().map(|x| x.chars().count()).max().unwrap();
        assert!(s.lines().all(|x| x.chars().count() == size_x));
        Ok(Grid {
            size_x,
            size_y,
            vals: s
                .lines()
                .rev()
                .flat_map(|s| {
                    s.chars().filter_map(|c| match c {
                        '#' => Some(Obj::Rock),
                        '.' => Some(Obj::Empty),
                        _ => None,
                    })
                })
                .collect(),
        })
    }
}

impl core::fmt::Display for Grid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for y in (0..self.size_y).rev() {
            for x in 0..self.size_x {
                write!(
                    f,
                    "{}",
                    match self.vals[self.offset(&Point2::new(x as i32, y as i32))] {
                        Obj::Rock => '#',
                        Obj::Empty => '.',
                    }
                )?;
            }
            writeln!(f)?
        }
        write!(f, "=================")?;
        Ok(())
    }
}
impl Grid {
    fn offset(&self, p: &Point2<i32>) -> usize {
        p.y as usize * self.size_x + p.x as usize
    }
    fn reverse(&self, p: usize) -> Point2<i32> {
        Point2::new((p % self.size_x) as i32, (p / self.size_x) as i32)
    }

    fn resize_y(&mut self, size_y: usize) {
        assert!(self.size_y <= size_y);
        if self.size_y == size_y {
            return;
        }
        self.vals
            .extend((0..self.size_x * (size_y - self.size_y)).map(|_| Obj::Empty));
        self.size_y = size_y;
        assert_eq!(self.vals.len(), self.size_y * self.size_x);
    }
    fn test_place(&self, obj: &Grid, bot_left: Vector2<i32>) -> bool {
        if bot_left.x < 0
            || bot_left.y < 0
            || bot_left.x as usize + obj.size_x > self.size_x
            || bot_left.y as usize + obj.size_y > self.size_y
        {
            false
        } else {
            obj.iter()
                .filter_map(|(p, v)| {
                    if *v == Obj::Empty {
                        None
                    } else {
                        Some(p + bot_left)
                    }
                })
                .all(|p| {
                    //println!("{:?}", p);
                    assert!(p.x < self.size_x as i32);
                    assert!(p.y < self.size_y as i32);
                    assert!(p.x >= 0 && p.y >= 0);
                    self.vals[self.offset(&p)] == Obj::Empty
                })
        }
    }
    fn place(&mut self, obj: &Grid, bot_left: Vector2<i32>) {
        assert!(self.test_place(obj, bot_left));

        obj.iter()
            .filter_map(|(p, v)| {
                if *v == Obj::Empty {
                    None
                } else {
                    Some(p + bot_left)
                }
            })
            .for_each(|p| {
                //println!("Place: {:?}", p);
                let offset = self.offset(&p);
                self.vals[offset] = Obj::Rock
            });
    }
    fn iter(&self) -> impl Iterator<Item = (Point2<i32>, &Obj)> {
        self.vals
            .iter()
            .enumerate()
            .map(|(i, v)| (self.reverse(i), v))
    }
}

fn tetrominoes() -> Vec<Grid> {
    [
        "####",
        ".#.\n\
         ###\n\
         .#.",
        "..#\n\
         ..#\n\
         ###",
        "#\n\
         #\n\
         #\n\
         #",
        "##\n\
         ##",
    ]
    .into_iter()
    .map(|t| t.parse().unwrap())
    .collect()
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 17;
    type Input1 = Vec<Jet>;
    type Input2 = Vec<Jet>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '>' => Jet::Right,
                    '<' => Jet::Left,
                    _ => panic!(),
                })
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut g = Grid {
            size_x: 7,
            size_y: 1,
            vals: vec![Obj::Empty; 7],
        };

        let t = tetrominoes();
        //println!("{:?}", t);
        let t = t.iter().cycle();

        let mut jets = v.iter().cycle();

        for tetris in t.take(2022) {
            let top_y = g
                .iter()
                .filter(|(_p, t)| **t == Obj::Rock)
                .max_by_key(|(p, _)| p.y)
                .map(|(p, _)| p.y + 1)
                .unwrap_or(0);

            let mut top_left = Vector2::new(2, top_y + 3);
            g.resize_y(top_left.y as usize + 5);
            assert!(g.test_place(tetris, top_left));

            //println!("Start {:?}", top_left);
            'stuck: loop {
                let push = match jets.next().unwrap() {
                    Jet::Left => Vector2::new(-1, 0),
                    Jet::Right => Vector2::new(1, 0),
                };
                let new_top_left = top_left + push;
                top_left = if g.test_place(tetris, new_top_left) {
                    new_top_left
                } else {
                    top_left
                };
                //println!("Jet {:?}", top_left);

                let new_top_left = top_left + Vector2::new(0, -1);
                top_left = if g.test_place(tetris, new_top_left) {
                    new_top_left
                } else {
                    //println!("Down blocked");
                    g.place(tetris, top_left);
                    break 'stuck;
                };
                //println!("Down {:?}", top_left);
            }

            //println!("Grid {}\n{}", i, g);
        }

        g.iter()
            .filter(|(_p, t)| **t == Obj::Rock)
            .max_by_key(|(p, _)| p.y)
            .map(|(p, _)| p.y + 1)
            .unwrap_or(0) as usize
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut g = Grid {
            size_x: 7,
            size_y: 1,
            vals: vec![Obj::Empty; 7],
        };

        let pieces = tetrominoes();
        //println!("{:?}", t);
        let mut jets = v.iter().cycle();
        let mut jet_count = 0;

        let mut prev_seen = HashMap::new();
        let mut pattern = vec![];

        let max_pieces = 1000000000000;
        let mut skip_done = None;

        let mut tetris_idx = 0usize;
        loop {
            if tetris_idx == max_pieces {
                break;
            }
            let tetris = &pieces[tetris_idx % pieces.len()];
            tetris_idx += 1;
            let top_y = g
                .iter()
                .filter(|(_p, t)| **t == Obj::Rock)
                .max_by_key(|(p, _)| p.y)
                .map(|(p, _)| p.y + 1)
                .unwrap_or(0);

            let mut top_left = Vector2::new(2, top_y + 3);
            g.resize_y(top_left.y as usize + 5);

            // if top_y > 0 && (0..7).all(|v| g.lookup(&Point2::new(v, top_y - 1)) == Obj::Rock) {
            //     println!("Fresh Start {i}");
            // }
            let entry = prev_seen
                .entry((jet_count % v.len(), tetris_idx % pieces.len()))
                .or_insert((tetris_idx, top_y));
            if skip_done.is_none() && jet_count > v.len() {
                //println!("{i}: {top_y} {}", top_y - *entry);
                pattern.push((tetris_idx - entry.0, top_y - entry.1));
                for i in (2..pattern.len() / 2).rev() {
                    let (a, b) = pattern.split_at(pattern.len() - i);
                    let (_, a) = a.split_at(a.len() - i);
                    if a == b && a.len() == 10 && a[0].0 != 0 {
                        println!("{:?}", a);
                        let skip_times = (max_pieces - tetris_idx) / a[0].0;
                        println!(
                            "Skipping {} {} at {}",
                            a[0].0,
                            skip_times * a[0].0,
                            tetris_idx
                        );

                        //tetris_idx += skip_times * v.len() - 1;
                        tetris_idx += skip_times * a[0].0;
                        //let new_t = &pieces[tetris_idx % pieces.len()];
                        //assert_eq!(new_t, tetris);
                        skip_done = Some(skip_times * usize::try_from(a[0].1).unwrap());
                        println!("Skipped {:?}", skip_done);
                        break;
                    };
                }
            }
            *entry = (tetris_idx, top_y);

            assert!(g.test_place(tetris, top_left));

            //println!("Start {:?}", top_left);
            'stuck: loop {
                let push = match jets.next().unwrap() {
                    Jet::Left => Vector2::new(-1, 0),
                    Jet::Right => Vector2::new(1, 0),
                };
                jet_count += 1;
                let new_top_left = top_left + push;
                top_left = if g.test_place(tetris, new_top_left) {
                    new_top_left
                } else {
                    top_left
                };
                //println!("Jet {:?}", top_left);

                let new_top_left = top_left + Vector2::new(0, -1);
                top_left = if g.test_place(tetris, new_top_left) {
                    new_top_left
                } else {
                    //println!("Down blocked");
                    g.place(tetris, top_left);
                    break 'stuck;
                };
                //println!("Down {:?}", top_left);
            }

            //println!("Grid {}\n{}", i, g);
        }

        g.iter()
            .filter(|(_p, t)| **t == Obj::Rock)
            .max_by_key(|(p, _)| p.y)
            .map(|(p, _)| p.y + 1)
            .unwrap_or(0) as usize
            + skip_done.unwrap()
    }
}

crate::default_tests!(3206, 1602881844347);
crate::string_tests!(
    [(t1, ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 3068)],
    [(
        t2,
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
        1514285714288
    )]
);
