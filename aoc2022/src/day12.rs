use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Height {
    Start,
    End,
    Elev(u8),
}
impl Height {
    fn height(&self) -> u8 {
        match self {
            Height::Start => 0,
            Height::End => b'z' - b'a',
            Height::Elev(c) => *c,
        }
    }
}

#[derive(Debug)]
pub struct HeightMap {
    size_x: usize,
    size_y: usize,
    grid: Vec<Height>,
}
impl HeightMap {
    fn offset(&self, p: &Point2<i32>) -> usize {
        p.y as usize * self.size_x + p.x as usize
    }
    fn lookup(&self, p: &Point2<i32>) -> Height {
        self.grid[self.offset(p)]
    }
    fn reverse(&self, p: usize) -> Point2<i32> {
        Point2::new((p % self.size_x) as i32, (p / self.size_x) as i32)
    }

    fn adj(&self, p: &Point2<i32>) -> Vec<Point2<i32>> {
        let adj = [
            Vector2::new(-1, 0),
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
        ];
        adj.into_iter()
            .map(|v| p + v)
            .filter(|v| {
                v.x >= 0 && v.y >= 0 && v.x < self.size_x as i32 && v.y < self.size_y as i32
            })
            .collect()
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 12;
    type Input1 = HeightMap;
    type Input2 = HeightMap;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut height = 0;
        let mut size_x = 0;

        let heights = s
            .lines()
            .flat_map(|l| {
                height += 1;
                size_x = l.chars().count();
                l.chars().map(|c| match c {
                    'S' => Height::Start,
                    'E' => Height::End,
                    c @ 'a'..='z' => Height::Elev(c as u8 - b'a'),
                    _ => panic!(),
                })
            })
            .collect();

        HeightMap {
            size_y: height,
            size_x,
            grid: heights,
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let offset = v.grid.iter().position(|v| *v == Height::Start).unwrap();
        let p = v.reverse(offset);

        let mut worklist = vec![(p, 0)];

        let mut visited = HashMap::new();

        while let Some((pt, steps)) = worklist.pop() {
            let visited = visited.entry(pt).or_insert(None);
            if let Some(vsteps) = visited {
                if steps < *vsteps {
                    *visited = Some(steps)
                } else {
                    continue;
                }
            } else {
                *visited = Some(steps);
            }

            let cur_h = v.lookup(&pt);
            worklist.extend(
                v.adj(&pt)
                    .iter()
                    .filter(|x| v.lookup(x).height() <= 1 + cur_h.height())
                    .map(|x| (*x, steps + 1)),
            );
        }
        let offset = v.grid.iter().position(|v| *v == Height::End).unwrap();
        let p = v.reverse(offset);

        visited[&p].unwrap()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let offset = v.grid.iter().position(|v| *v == Height::End).unwrap();
        let p = v.reverse(offset);

        let mut worklist = vec![(p, 0)];

        let mut visited = HashMap::new();

        while let Some((pt, steps)) = worklist.pop() {
            let visited = visited.entry(pt).or_insert(None);
            if let Some(vsteps) = visited {
                if steps < *vsteps {
                    *visited = Some(steps)
                } else {
                    continue;
                }
            } else {
                *visited = Some(steps);
            }

            let cur_h = v.lookup(&pt);
            worklist.extend(
                v.adj(&pt)
                    .iter()
                    .filter(|x| {
                        let h = v.lookup(x).height();
                        h >= cur_h.height().saturating_sub(1)
                    })
                    .map(|x| (*x, steps + 1)),
            );
        }
        let offset = v
            .grid
            .iter()
            .enumerate()
            .filter(|(_pos, v)| v.height() == 0)
            .min_by_key(|(pos, _h)| {
                if let Some(Some(steps)) = visited.get(&v.reverse(*pos)) {
                    *steps
                } else {
                    std::usize::MAX
                }
            })
            .unwrap();

        let p = v.reverse(offset.0);

        visited[&p].unwrap()
    }
}

crate::default_tests!(447, 446);
crate::path_tests!([(t1, "test/day12.txt", 31)], [(t2, "test/day12.txt", 29)]);
