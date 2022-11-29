use crate::Day;
use cgmath::{Point2, Vector2};
use std::io::Result as IoResult;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Space {
    Open,
    Tree,
}

fn trees(v: &[Vec<Space>], offset: &Vector2<usize>) -> usize {
    v.iter()
        .enumerate()
        .scan(Point2::new(0, 0), |state, (i, r)| {
            if state.y == i {
                let old = *state;
                *state += *offset;
                Some(Some(r[old.x % r.len()]))
            } else {
                Some(None)
            }
        })
        .flatten()
        .filter(|x| *x == Space::Tree)
        .count()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 3;
    type Input = Vec<Space>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|s| {
                let v = s?
                    .chars()
                    .map(|c| match c {
                        '.' => Space::Open,
                        '#' => Space::Tree,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();
                Ok(v)
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let offset = Vector2::new(3, 1);

        trees(v, &offset)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let slopes = [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| Vector2::new(*x, *y))
            .collect::<Vec<_>>();

        slopes.iter().map(|s| trees(v, s)).product()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let slope = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let v = Solution::process_input(std::io::BufReader::new(slope.as_bytes())).unwrap();
        assert_eq!(trees(&v, &Vector2::new(3, 1)), 7);
        let slopes = [(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(x, y)| Vector2::new(*x, *y))
            .collect::<Vec<_>>();
        let tree_count = slopes.iter().map(|s| trees(&v, s)).collect::<Vec<_>>();
        assert_eq!(&tree_count, &[2, 7, 3, 4, 2]);
    }
}
