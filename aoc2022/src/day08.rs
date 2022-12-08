use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, PartialEq)]
pub struct Grid {
    h: usize,
    w: usize,
    g: Vec<u32>,
}

impl Grid {
    fn rows(&self) -> impl Iterator<Item = &[u32]> {
        self.g.chunks_exact(self.w)
    }
    fn cols(&self) -> Vec<Vec<u32>> {
        (0..self.w)
            .map(|x| (0..self.h).map(|y| self.pt(x, y)).collect())
            .collect()
    }
    fn pt(&self, x: usize, y: usize) -> u32 {
        self.g[x + y * self.w]
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 8;
    type Input1 = Grid;
    type Input2 = Grid;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut width = None;
        let mut height = 0;
        let grid = s
            .lines()
            .flat_map(|l| {
                height += 1;
                width = Some(l.chars().count());
                l.chars().map(|i| i.to_digit(10).unwrap())
            })
            .collect();
        Grid {
            h: height,
            w: width.unwrap(),
            g: grid,
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        fn visible(iter: impl Iterator<Item = (usize, u32)>) -> impl Iterator<Item = (usize, u32)> {
            let mut height = None;
            iter.filter(move |(_p, x)| {
                let seen = height.map(|h| h < *x).unwrap_or(true);
                if seen {
                    height = Some(*x);
                }
                seen
            })
        }
        let lr: Vec<_> = v
            .rows()
            .enumerate()
            .flat_map(|(p1, r)| {
                visible(r.iter().copied().enumerate())
                    .chain(visible(r.iter().copied().enumerate().rev()))
                    .map(|(p, _)| (p, p1))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect();

        let ud: Vec<_> = v
            .cols()
            .iter()
            .enumerate()
            .flat_map(|(p1, r)| {
                visible(r.iter().copied().enumerate())
                    .chain(visible(r.iter().copied().enumerate().rev()))
                    .map(|(p, _)| (p1, p))
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect();

        let mut all = lr;
        all.extend(ud.iter());
        all.sort();
        all.dedup();

        all.len()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        fn visible(height: u32, iter: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
            let mut blocked = false;
            iter.take_while(move |x| {
                if blocked {
                    false
                } else if *x < height {
                    true
                } else {
                    blocked = true;
                    true
                }
            })
        }
        let cols = &v.cols();
        (0..v.h)
            .flat_map(|y| {
                (0..v.w).map(move |x| {
                    let h = v.pt(x, y);
                    let row = v.rows().nth(y).unwrap();
                    let col = &cols[x];
                    let (l, r) = (&row[..x], &row[x + 1..]);
                    let (u, d) = (&col[..y], &col[y + 1..]);
                    let score = visible(h, l.iter().rev().copied()).count()
                        * visible(h, r.iter().copied()).count()
                        * visible(h, u.iter().rev().copied()).count()
                        * visible(h, d.iter().copied()).count();
                    score
                })
            })
            .max()
            .unwrap()
    }
}

crate::default_tests!(1736, 268800);
crate::path_tests!(
    [(sol1, "test/day08.txt", 21)],
    [(sol2, "test/day08.txt", 8)]
);
