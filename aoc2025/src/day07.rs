use crate::grid::Grid;
use crate::Day;
use cgmath::Vector2;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Space {
    Start,
    Empty,
    Splitter,
    Beam,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Space::Empty => '.',
            Space::Start => 'S',
            Space::Splitter => '^',
            Space::Beam => '|',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input1 = Grid<Space>;
    type Input2 = Grid<Space>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let height = s.lines().count();
        let width = s.lines().map(|x| x.len()).max().unwrap();

        let v = s
            .lines()
            .flat_map(|s| {
                s.chars().map(|v| match v {
                    'S' => Space::Start,
                    '.' => Space::Empty,
                    '^' => Space::Splitter,
                    '|' => Space::Beam,
                    _ => panic!(),
                })
            })
            .collect::<Vec<_>>();

        Grid::new(v, height, width)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut v = v.clone();
        let start = v
            .iter_pts()
            .find_map(|(pt, v)| match v {
                Space::Start => Some(pt),
                _ => None,
            })
            .unwrap();

        const DOWN: Vector2<i32> = Vector2::new(0, 1);
        const LEFT: Vector2<i32> = Vector2::new(-1, 0);
        const RIGHT: Vector2<i32> = Vector2::new(1, 0);

        let mut splits = 0;
        let mut beams: VecDeque<_> = VecDeque::new();
        beams.push_front(start);

        while let Some(beam) = beams.pop_front() {
            //println!("{} {:?} {:?}", v, beam, beams);
            match v.get_mut(&beam) {
                Some(val @ (Space::Empty | Space::Start)) => {
                    *val = Space::Beam;
                    beams.push_back(beam + DOWN);
                }
                Some(Space::Beam) => {}
                Some(Space::Splitter) => {
                    splits += 1;
                    if let Some(_v @ Space::Empty) = v.get_mut(&(beam + LEFT)) {
                        beams.push_front(beam + LEFT);
                    }
                    if let Some(_v @ Space::Empty) = v.get_mut(&(beam + RIGHT)) {
                        beams.push_front(beam + RIGHT);
                    }
                }
                None => {}
            }
        }

        splits
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        let start = v
            .iter_pts()
            .find_map(|(pt, v)| match v {
                Space::Start => Some(pt),
                _ => None,
            })
            .unwrap();

        const DOWN: Vector2<i32> = Vector2::new(0, 1);
        const LEFT: Vector2<i32> = Vector2::new(-1, 0);
        const RIGHT: Vector2<i32> = Vector2::new(1, 0);

        let mut timelines = 0;
        let mut beams: VecDeque<_> = VecDeque::new();

        beams.push_front((start, 1));

        loop {
            let mut next_beams = HashMap::new();
            while let Some((beam, count)) = beams.pop_front() {
                match v.get_mut(&beam) {
                    Some(val @ (Space::Beam | Space::Empty | Space::Start)) => {
                        *val = Space::Beam;
                        next_beams
                            .entry(beam + DOWN)
                            .and_modify(|v| *v += count)
                            .or_insert(count);
                    }

                    Some(Space::Splitter) => {
                        let l_beam = beam + LEFT;

                        if let Some(_val) = v.get_mut(&l_beam) {
                            next_beams
                                .entry(l_beam)
                                .and_modify(|v| *v += count)
                                .or_insert(count);
                        }

                        let r_beam = beam + RIGHT;

                        if let Some(_val) = v.get_mut(&r_beam) {
                            next_beams
                                .entry(r_beam)
                                .and_modify(|v| *v += count)
                                .or_insert(count);
                        }
                    }
                    None => timelines += count,
                }
            }

            if next_beams.is_empty() {
                break;
            } else {
                beams = next_beams.into_iter().collect();
            }
        }

        timelines
    }
}

crate::default_tests!(1566, 5921061943075);
crate::string_tests!(
    [(
        foo_sol1,
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        21
    )],
    [(
        foo_sol2,
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        40
    )]
);
