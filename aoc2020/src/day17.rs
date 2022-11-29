use crate::Day;
use cgmath::{Point3, Vector3, Vector4};
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Copy, Clone)]
pub enum Cube {
    Active,
    Inactive,
}

/* why is there no 4-d Point in cgmath, because who would use it? */
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 17;
    type Input = HashMap<Point3<i64>, Cube>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let vec = r
            .lines()
            .map(|l| {
                let l = l?;
                Ok(l.chars()
                    .map(|x| match x {
                        '#' => Cube::Active,
                        '.' => Cube::Inactive,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>())
            })
            .collect::<IoResult<Vec<_>>>()?;

        let h = vec
            .into_iter()
            .enumerate()
            .flat_map(move |(y, v)| {
                v.into_iter().enumerate().map(move |(x, c)| {
                    (
                        Point3 {
                            x: x as i64,
                            y: y as i64,
                            z: 0,
                        },
                        c,
                    )
                })
            })
            .collect::<HashMap<_, _>>();
        Ok(vec![h])
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut neighbors = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !(x == 0 && y == 0 && z == 0) {
                        neighbors.push(Vector3 { x, y, z })
                    }
                }
            }
        }
        assert_eq!(neighbors.len(), 26);

        let mut current = v[0].clone();
        let mut next;
        for _g in 0..6 {
            let actives = current.iter().fold(HashMap::new(), |mut state, (p, c)| {
                if matches!(c, Cube::Active) {
                    neighbors.iter().map(|v| p + v).for_each(|p| {
                        state.entry(p).and_modify(|t| *t += 1).or_insert(1);
                    });
                }
                state
            });

            next = actives
                .iter()
                .filter(|(_, &count)| count == 2 || count == 3)
                .flat_map(|(p, &v)| match current.get(p).unwrap_or(&Cube::Inactive) {
                    Cube::Inactive if v == 3 => Some((*p, Cube::Active)),
                    Cube::Active if v == 2 || v == 3 => Some((*p, Cube::Active)),
                    _ => None,
                })
                .collect::<HashMap<_, _>>();

            std::mem::swap(&mut next, &mut current);
            next.clear();
        }

        current
            .values()
            .filter(|x| matches!(x, Cube::Active))
            .count()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut neighbors = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if !(x == 0 && y == 0 && z == 0 && w == 0) {
                            neighbors.push(Vector4 { x, y, z, w })
                        }
                    }
                }
            }
        }
        let current = v[0].clone();
        let mut current: HashMap<Point4, Cube> = current
            .into_iter()
            .map(|(p, v)| {
                (
                    Point4 {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                        w: 0,
                    },
                    v,
                )
            })
            .collect();
        let mut next;
        for _g in 0..6 {
            let actives = current.iter().fold(HashMap::new(), |mut state, (p, c)| {
                if matches!(c, Cube::Active) {
                    neighbors
                        .iter()
                        .map(|v| Point4 {
                            x: p.x + v.x,
                            y: p.y + v.y,
                            z: p.z + v.z,
                            w: p.w + v.w,
                        })
                        .for_each(|p| {
                            state.entry(p).and_modify(|t| *t += 1).or_insert(1);
                        });
                }
                state
            });

            next = actives
                .iter()
                .filter(|(_, &count)| count == 2 || count == 3)
                .flat_map(|(p, &v)| match current.get(p).unwrap_or(&Cube::Inactive) {
                    Cube::Inactive if v == 3 => Some((*p, Cube::Active)),
                    Cube::Active if v == 2 || v == 3 => Some((*p, Cube::Active)),
                    _ => None,
                })
                .collect::<HashMap<_, _>>();

            std::mem::swap(&mut next, &mut current);
            next.clear();
        }

        current
            .values()
            .filter(|x| matches!(x, Cube::Active))
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = ".#.
..#
###";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 112);
        assert_eq!(Solution::p2(&v), 848);
    }
}
