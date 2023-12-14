use crate::Day;
use nalgebra::DMatrix;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    RockRound,
    Rock,
    Space,
}

impl std::fmt::Display for Cell {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::RockRound => 'O',
            Cell::Rock => '#',
            Cell::Space => '.',
        };
        write!(f, "{}", c)
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 14;
    type Input1 = DMatrix<Cell>;
    type Input2 = DMatrix<Cell>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let y = lines.clone().count();
        let x = lines.clone().next().unwrap().len();

        let d = DMatrix::from_row_iterator(
            x,
            y,
            lines.by_ref().take_while(|s| !s.is_empty()).flat_map(|l| {
                l.chars().map(|c| match c {
                    '#' => Cell::Rock,
                    'O' => Cell::RockRound,
                    '.' => Cell::Space,
                    _ => panic!(),
                })
            }),
        );
        d
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        println!("{}", v);
        let mut v = v.clone();
        v.column_iter_mut()
            .map(|mut c| {
                let v: Vec<_> = c
                    .iter()
                    .enumerate()
                    .filter(|(_i, x)| **x == Cell::Rock)
                    .map(|(i, _c)| i)
                    .chain(std::iter::once(c.len()))
                    .collect();

                v.iter()
                    .scan(0, |state, e| {
                        let v = *state..*e;
                        *state = e + 1;
                        Some(v)
                    })
                    .for_each(|r| {
                        let count = c.as_slice()[r.clone()]
                            .iter()
                            .filter(|c| **c == Cell::RockRound)
                            .count();
                        let (roll, rest) = c.as_mut_slice()[r.clone()].split_at_mut(count);
                        roll.fill(Cell::RockRound);
                        rest.fill(Cell::Space);
                    });
            })
            .for_each(|_| {});
        println!("{}", v);
        v.column_iter()
            .flat_map(|c| {
                c.iter()
                    .zip((1..=c.len()).rev())
                    .map(|(c, i)| match c {
                        Cell::RockRound => i,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
            })
            .sum::<usize>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        fn rotate_90<T: nalgebra::Scalar>(v: &mut DMatrix<T>) {
            v.transpose_mut();
            for i in 0..v.ncols() / 2 {
                v.swap_columns(i, v.ncols() - i - 1);
            }
        }

        let mut seen: HashMap<DMatrix<Cell>, usize> = HashMap::new();

        let mut v = v.clone();
        let mut cycles = 0..1000000000;
        let mut skipped = false;
        while let Some(cycle) = cycles.by_ref().next() {
            if !skipped {
                if let Some(v) = seen.get(&v) {
                    println!("Cycle {}->{}", v, cycle);
                    cycles.end = (cycles.end - cycles.start) % (cycle - v) + cycles.start;
                    println!("{:?}", cycles);
                    skipped = true;
                }

                seen.insert(v.clone(), cycle);
            }
            for _ in 0..4 {
                v.column_iter_mut()
                    .map(|mut c| {
                        let v: Vec<_> = c
                            .iter()
                            .enumerate()
                            .filter(|(_i, x)| **x == Cell::Rock)
                            .map(|(i, _c)| i)
                            .chain(std::iter::once(c.len()))
                            .collect();

                        v.iter()
                            .scan(0, |state, e| {
                                let v = *state..*e;
                                *state = e + 1;
                                Some(v)
                            })
                            .for_each(|r| {
                                let count = c.as_slice()[r.clone()]
                                    .iter()
                                    .filter(|c| **c == Cell::RockRound)
                                    .count();
                                let (roll, rest) = c.as_mut_slice()[r.clone()].split_at_mut(count);
                                roll.fill(Cell::RockRound);
                                rest.fill(Cell::Space);
                            });
                    })
                    .for_each(|_| {});
                rotate_90(&mut v);
            }
        }

        v.column_iter()
            .flat_map(|c| {
                c.iter()
                    .zip((1..=c.len()).rev())
                    .map(|(c, i)| match c {
                        Cell::RockRound => i,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
            })
            .sum::<usize>()
    }
}

crate::default_tests!(108614, 96447);
crate::string_tests!(
    [(
        foo_sol1,
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        136
    )],
    [(
        foo_sol2,
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        64
    )]
);
