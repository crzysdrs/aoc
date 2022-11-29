use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use cgmath::{Point2, Vector2};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Position {
    #[allow(dead_code)]
    fn to_char(self) -> char {
        match self {
            Position::Floor => '.',
            Position::OccupiedSeat => '#',
            Position::EmptySeat => 'L',
        }
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 11;
    type Input = Vec<Position>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                Ok(l.chars()
                    .map(|c| match c {
                        'L' => Position::EmptySeat,
                        '.' => Position::Floor,
                        '#' => Position::OccupiedSeat,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>())
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let dirs = vec![
            (-1isize, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(x, y)| Vector2::new(x, y))
        .collect::<Vec<_>>();

        let mut v = v.to_vec();
        let mut next = v.to_vec();
        loop {
            v.iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(x, c)| (Point2::new(x as isize, y as isize), *c))
                })
                .filter(|(_, c)| !matches!(c, Position::Floor))
                .for_each(|(p, c)| {
                    let counts = dirs
                        .iter()
                        .map(|d| p + d)
                        .filter(|d| {
                            d.x >= 0
                                && d.y >= 0
                                && d.y < v.len() as isize
                                && d.x < v[0].len() as isize
                        })
                        //.inspect(|d| println!("{:?} {}", d, v[d.y as usize][d.x as usize].to_char()))
                        .map(|p| v[p.y as usize][p.x as usize])
                        .fold(HashMap::new(), |mut state, c| {
                            state.entry(c).and_modify(|x| *x += 1).or_insert(1);
                            state
                        });
                    next[p.y as usize][p.x as usize] = match c {
                        Position::EmptySeat
                            if counts.get(&Position::OccupiedSeat).unwrap_or(&0) == &0 =>
                        {
                            Position::OccupiedSeat
                        }
                        Position::OccupiedSeat
                            if counts.get(&Position::OccupiedSeat).unwrap_or(&0) >= &4 =>
                        {
                            Position::EmptySeat
                        }
                        c => c,
                    }
                });

            if next == v {
                return next
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|x| matches!(x, Position::OccupiedSeat))
                    .count();
            }
            std::mem::swap(&mut v, &mut next);
        }
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let dirs = vec![
            (-1isize, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(x, y)| Vector2::new(x, y))
        .collect::<Vec<_>>();

        let mut v = v.to_vec();
        let mut next = v.to_vec();
        loop {
            v.iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(x, c)| (Point2::new(x as isize, y as isize), *c))
                })
                .filter(|(_, c)| !matches!(c, Position::Floor))
                .for_each(|(p, c)| {
                    let counts = dirs
                        .iter()
                        .flat_map(|d| {
                            (1..v[0].len() as isize)
                                .map(|m| p + m * d)
                                .filter(|d| {
                                    d.x >= 0
                                        && d.y >= 0
                                        && d.y < v.len() as isize
                                        && d.x < v[0].len() as isize
                                })
                                .find(|p| !matches!(v[p.y as usize][p.x as usize], Position::Floor))
                        })
                        .map(|p| v[p.y as usize][p.x as usize])
                        .fold(HashMap::new(), |mut state, c| {
                            state.entry(c).and_modify(|x| *x += 1).or_insert(1);
                            state
                        });
                    next[p.y as usize][p.x as usize] = match c {
                        Position::EmptySeat
                            if counts.get(&Position::OccupiedSeat).unwrap_or(&0) == &0 =>
                        {
                            Position::OccupiedSeat
                        }
                        Position::OccupiedSeat
                            if counts.get(&Position::OccupiedSeat).unwrap_or(&0) >= &5 =>
                        {
                            Position::EmptySeat
                        }
                        c => c,
                    }
                });

            if next == v {
                return next
                    .iter()
                    .flat_map(|row| row.iter())
                    .filter(|x| matches!(x, Position::OccupiedSeat))
                    .count();
            }
            std::mem::swap(&mut v, &mut next);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 37);
        assert_eq!(Solution::p2(&v), 26);
    }
}
