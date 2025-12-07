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

pub struct GridPoints<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
}

impl<'a, T> GridPoints<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
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

pub struct SubGrid<'a, T> {
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
    pub fn new(grid: Vec<T>, height: usize, width: usize) -> Grid<T> {
        Grid {
            grid,
            height,
            width,
        }
    }
    pub fn iter_pts(&self) -> GridPoints<'_, T> {
        GridPoints::new(self)
    }
    pub fn iter_subgrid(&self, x: Range<i32>, y: Range<i32>) -> SubGrid<'_, T> {
        SubGrid {
            grid: self,
            x_range: x.clone(),
            y_range: y.clone(),
            pt: Point2::new(x.start, y.start),
        }
    }
    pub fn get_mut(&mut self, v: &Point2<i32>) -> Option<&mut T> {
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

    pub fn get(&self, v: &Point2<i32>) -> Option<&T> {
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
