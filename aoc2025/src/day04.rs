use crate::grid::Grid;
use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

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
