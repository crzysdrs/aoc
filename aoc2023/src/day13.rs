use crate::Day;
use nalgebra::DMatrix;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Cell {
    Lava,
    Space,
}
impl Day for Solution {
    const DAY: u32 = 13;
    type Input1 = Vec<DMatrix<Cell>>;
    type Input2 = Vec<DMatrix<Cell>>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let mut matrices = vec![];

        loop {
            if lines.clone().next().is_none() {
                break;
            }
            let y = lines.clone().take_while(|s| !s.is_empty()).count();
            let x = lines.clone().next().unwrap().len();

            println!("{:?}", (y, x));
            let d = DMatrix::from_iterator(
                x,
                y,
                lines.by_ref().take_while(|s| !s.is_empty()).flat_map(|l| {
                    l.chars().map(|c| match c {
                        '#' => Cell::Lava,
                        '.' => Cell::Space,
                        _ => panic!(),
                    })
                }),
            );
            matrices.push(d);
        }
        matrices
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut row_mirror = vec![];
        let mut col_mirror = vec![];
        v.iter().for_each(|m| {
            let c: Vec<_> = m
                .row_iter()
                .zip(m.row_iter().skip(1))
                .enumerate()
                .filter(|(_i, (x, y))| x == y)
                .map(|(i, _)| i + 1)
                .collect();

            println!("{:?}", c);

            for split in c {
                let matches = (0..split)
                    .rev()
                    .zip(split..m.nrows())
                    .all(|(l, r)| m.row(l) == m.row(r));
                if matches {
                    row_mirror.push(split);
                }
            }
            let c: Vec<_> = m
                .column_iter()
                .zip(m.column_iter().skip(1))
                .enumerate()
                .filter(|(_i, (x, y))| x == y)
                .map(|(i, _)| i + 1)
                .collect();

            println!("{:?}", c);
            for split in c {
                let matches = (0..split)
                    .rev()
                    .zip(split..m.ncols())
                    .all(|(l, r)| m.column(l) == m.column(r));
                if matches {
                    col_mirror.push(split);
                }
            }
        });

        100 * col_mirror.iter().sum::<usize>() + row_mirror.iter().sum::<usize>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut row_mirror = vec![];
        let mut col_mirror = vec![];
        v.clone().iter_mut().for_each(|m| {
            let c: Vec<_> = m
                .row_iter()
                .zip(m.row_iter().skip(1))
                .enumerate()
                .filter(|(_i, (x, y))| {
                    x.iter()
                        .zip(y.iter())
                        .map(|(x, y)| (x != y) as usize)
                        .sum::<usize>()
                        <= 1
                })
                .map(|(i, _)| i + 1)
                .collect();

            for split in c {
                let matches = (0..split)
                    .rev()
                    .zip(split..m.nrows())
                    .map(|(l, r)| {
                        m.row(l)
                            .iter()
                            .zip(m.row(r).iter())
                            .map(|(x, y)| (x != y) as usize)
                            .sum::<usize>()
                    })
                    .sum::<usize>()
                    == 1;
                if matches {
                    row_mirror.push(split);
                }
            }
            let c: Vec<_> = m
                .column_iter()
                .zip(m.column_iter().skip(1))
                .enumerate()
                .filter(|(_i, (x, y))| {
                    x.iter()
                        .zip(y.iter())
                        .map(|(x, y)| (x != y) as usize)
                        .sum::<usize>()
                        <= 1
                })
                .map(|(i, _)| i + 1)
                .collect();

            for split in c {
                let matches = (0..split)
                    .rev()
                    .zip(split..m.ncols())
                    .map(|(l, r)| {
                        m.column(l)
                            .iter()
                            .zip(m.column(r).iter())
                            .map(|(x, y)| (x != y) as usize)
                            .sum::<usize>()
                    })
                    .sum::<usize>()
                    == 1;

                if matches {
                    col_mirror.push(split);
                }
            }
        });

        100 * col_mirror.iter().sum::<usize>() + row_mirror.iter().sum::<usize>()
    }
}

crate::default_tests!(31265, 39359);
crate::string_tests!(
    [(
        foo_sol1,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        405
    )],
    [(
        foo_sol2,
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        400
    )]
);
