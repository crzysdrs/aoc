use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub enum Cell {
    Space,
    MirrorUp,
    MirrorDown,
    SplitterH,
    SplitterV,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Beam {
    pos: Point2<i32>,
    dir: Vector2<i32>,
}
impl Beam {
    fn hit(&mut self, cell: &Cell) -> Option<Beam> {
        let mut new_beam = None;
        let new_dir = match (cell, self.dir.as_ref()) {
            (Cell::Space, d) => *d,
            (Cell::SplitterH, d @ ((1, 0) | (-1, 0))) => *d,
            (Cell::SplitterV, d @ ((0, 1) | (0, -1))) => *d,
            (Cell::SplitterH | Cell::SplitterV, _) => {
                new_beam = Some(self.split(cell));
                *self.dir.as_ref()
            }
            (Cell::MirrorUp, d) => match d {
                (0, 1) => (1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!(),
            },
            (Cell::MirrorDown, d) => match d {
                (0, 1) => (-1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, 1),
                _ => panic!(),
            },
        };
        self.dir = new_dir.into();
        new_beam
    }
    fn split(&mut self, cell: &Cell) -> Beam {
        self.dir = match cell {
            Cell::SplitterH => Vector2::new(1, 0),
            Cell::SplitterV => Vector2::new(0, 1),
            _ => panic!(),
        };

        Beam {
            pos: self.pos,
            dir: -self.dir,
        }
    }
}

fn beam_sim(beam: Beam, v: &HashMap<Point2<i32>, Cell>) -> usize {
    let mut beams = vec![beam];
    let mut energized = HashSet::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    //println!("{:?}", beams);
    while let Some(mut beam) = beams.pop() {
        if seen.contains(&beam) {
            continue;
        }
        seen.insert(beam.clone());
        //println!("{:?}", beam.pos);
        if let Some(cell) = v.get(&beam.pos) {
            energized.insert(beam.pos);
            //println!("{:?}", cell);
            if let Some(mut new) = beam.hit(cell) {
                new.pos += new.dir;
                beams.push(new);
            }
            beam.pos += beam.dir;
            beams.push(beam);
        }
        //println!("{:?}", beams);
    }
    energized.len()
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 16;
    type Input1 = HashMap<Point2<i32>, Cell>;
    type Input2 = HashMap<Point2<i32>, Cell>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    let cell = match c {
                        '.' => Cell::Space,
                        '/' => Cell::MirrorUp,
                        '\\' => Cell::MirrorDown,
                        '|' => Cell::SplitterV,
                        '-' => Cell::SplitterH,
                        _ => panic!(),
                    };
                    (Point2::new(x as i32, -(y as i32)), cell)
                })
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let beam = Beam {
            pos: Point2::new(0, 0),
            dir: Vector2::new(1, 0),
        };

        beam_sim(beam, v)
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let max_x = v.keys().max_by_key(|k| k.x).unwrap().x;
        let max_y = v.keys().min_by_key(|k| k.y).unwrap().y;
        let mut beams = vec![];
        for x in 0..=max_x {
            beams.push(Beam {
                pos: Point2::new(x, 0),
                dir: Vector2::new(0, -1),
            });

            beams.push(Beam {
                pos: Point2::new(x, max_y),
                dir: Vector2::new(0, 1),
            });
        }

        for y in max_y..=0 {
            beams.push(Beam {
                pos: Point2::new(0, y),
                dir: Vector2::new(1, 0),
            });

            beams.push(Beam {
                pos: Point2::new(max_y, y),
                dir: Vector2::new(-1, 0),
            });
        }

        beams.into_iter().map(|b| beam_sim(b, v)).max().unwrap()
    }
}

crate::default_tests!(7392, 7665);
crate::string_tests!(
    [(
        foo_sol1,
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        46
    )],
    [(
        foo_sol2,
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        51
    )]
);
