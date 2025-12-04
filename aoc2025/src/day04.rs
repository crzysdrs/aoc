use crate::Day;
use cgmath::Point2;
#[allow(unused_imports)]
use std::collections::*;
use std::ops::Range;

#[derive(Clone, PartialEq)]
pub struct Grid<T> {
    grid: Vec<T>,
    height: usize,
    width: usize,
}

struct GridPoints<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
}

impl<'a, T> GridPoints<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, idx: 0 }
    }
}

impl<'a, T> Iterator for GridPoints<'a, T> {
    type Item = (Point2<i32>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.grid.grid.get(self.idx)?;
        let old_idx = self.idx;
        self.idx += 1;

        Some((
            Point2::new(
                (old_idx % self.grid.width) as i32,
                (old_idx / self.grid.width) as i32,
            ),
            v,
        ))
    }
}

struct SubGrid<'a, T> {
    grid: &'a Grid<T>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    pt: Point2<i32>,
}

impl<'a, T> Iterator for SubGrid<'a, T> {
    type Item = (Point2<i32>, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        let old_pt = self.pt;
        let next = if self.x_range.contains(&self.pt.x) && self.y_range.contains(&self.pt.y) {
            Some(self.grid.get(&self.pt))
        } else {
            None
        }?;
        self.pt.x += 1;
        if !self.x_range.contains(&self.pt.x) {
            self.pt.x = self.x_range.start;
            self.pt.y += 1;
        }

        Some((old_pt, next))
    }
}

impl<T> Grid<T> {
    fn new(grid: Vec<T>, height: usize, width: usize) -> Grid<T> {
        Grid {
            grid,
            height,
            width,
        }
    }
    fn iter_pts(&self) -> GridPoints<'_, T> {
        GridPoints::new(self)
    }
    fn iter_subgrid(&self, x: Range<i32>, y: Range<i32>) -> SubGrid<'_, T> {
        SubGrid {
            grid: self,
            x_range: x.clone(),
            y_range: y.clone(),
            pt: Point2::new(x.start, y.start),
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

#[derive(Copy, Clone, PartialEq)]
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
        v.iter_pts()
            .filter(|(_, v)| matches!(v, Space::Paper))
            .map(|(p, _)| {
                v.iter_subgrid(p.x - 1..p.x + 2, p.y - 1..p.y + 2)
                    .filter(|(_, val)| matches!(val, Some(Space::Paper)))
                    .count()
            })
            .filter(|v| *v < 5)
            .count()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        let mut count = 0;
        loop {
            let mut new = v.clone();
            v.iter_pts()
                .filter(|(_, v)| matches!(v, Space::Paper))
                .map(|(p, _)| {
                    let paper = v
                        .iter_subgrid(p.x - 1..p.x + 2, p.y - 1..p.y + 2)
                        .filter(|(_, val)| matches!(val, Some(Space::Paper)))
                        .count();

                    (p, paper)
                })
                .filter(|(_p, paper)| *paper < 5)
                .for_each(|(p, _)| {
                    if let Some(v) = new.get_mut(&p) {
                        count += 1;
                        *v = Space::Empty;
                    }
                });
            if new == v {
                break;
            }
            std::mem::swap(&mut new, &mut v);
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
