use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new(grid: Vec<T>, height: usize, width: usize) -> Grid<T> {
        Grid {
            grid,
            height,
            width,
        }
    }
    fn get_mut(&mut self, v: &Point2<i32>) -> Option<&mut T> {
        let v: Point2<usize> = if v.x >= 0 && v.y >= 0 {
            Some(Point2::new(v.x as usize, v.y as usize))
        } else {
            None
        }?;

        if v.x < self.width && v.y < self.height {
            self.grid.get_mut(v.y * self.width + v.x)
        } else {
            None
        }
    }

    fn get(&self, v: &Point2<i32>) -> Option<&T> {
        let v: Point2<usize> = if v.x >= 0 && v.y >= 0 {
            Some(Point2::new(v.x as usize, v.y as usize))
        } else {
            None
        }?;

        if v.x < self.width && v.y < self.height {
            self.grid.get(v.y * self.width + v.x)
        } else {
            None
        }
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.grid
            .chunks(self.width)
            .map(|c| {
                for v in c {
                    write!(f, "{}", v)?;
                }
                writeln!(f)?;
                Ok(())
            })
            .collect::<std::fmt::Result>()?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Space {
    Empty,
    Paper,
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Space::Empty => '.',
            Space::Paper => '@',
        };
        write!(f, "{}", c)?;
        Ok(())
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 4;
    type Input1 = Grid<Space>;
    type Input2 = Grid<Space>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let vs = s
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Space::Empty,
                        '@' => Space::Paper,
                        _ => panic!(),
                    })
                    .into_iter()
            })
            .collect();

        let height = s.lines().count();
        let width = s.lines().map(|l| l.len()).max().unwrap_or(0);
        Grid::new(vs, height, width)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut count = 0;
        for x in 0..v.width {
            for y in 0..v.height {
                let p = Point2::new(x as i32, y as i32);
                if let Some(Space::Paper) = v.get(&p) {
                    let mut paper = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            let vec = Vector2::new(i, j);
                            let p2 = p + vec;
                            if let Some(Space::Paper) = v.get(&p2) {
                                paper += 1;
                            }
                        }
                    }
                    if paper < 5 {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        let mut count = 0;
        loop {
            let mut pass = 0;
            let mut new = v.clone();
            for x in 0..v.width {
                for y in 0..v.height {
                    let p = Point2::new(x as i32, y as i32);
                    if let Some(Space::Paper) = v.get(&p) {
                        let mut paper = 0;
                        for i in -1..=1 {
                            for j in -1..=1 {
                                let vec = Vector2::new(i, j);
                                let p2 = p + vec;
                                if let Some(Space::Paper) = v.get(&p2) {
                                    paper += 1;
                                }
                            }
                        }
                        if paper < 5 {
                            pass += 1;
                            count += 1;
                            if let Some(v) = new.get_mut(&p) {
                                *v = Space::Empty;
                            }
                        }
                    }
                }
            }
            if pass == 0 {
                break;
            }
            std::mem::swap(&mut v, &mut new);
        }
        count
    }
}

crate::default_tests!(1547, 8948);
crate::string_tests!(
    [(
        foo_sol1,
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
        13
    )],
    [(
        foo_sol2,
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        43
    )]
);
