use crate::Day;
use cgmath::{Point2, Vector2};

#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, PartialEq)]
pub enum Path {
    Forest,
    Clear,
    Up,
    Left,
    Right,
    Down,
}

const UP: Vector2<i32> = Vector2::new(0, -1);
const DOWN: Vector2<i32> = Vector2::new(0, 1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 23;
    type Input1 = HashMap<Point2<i32>, Path>;
    type Input2 = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars().enumerate().map(move |(x, c)| {
                    let p = Point2::new(x as i32, y as i32);
                    let e = match c {
                        '#' => Path::Forest,
                        '.' => Path::Clear,
                        '^' => Path::Up,
                        '>' => Path::Right,
                        '<' => Path::Left,
                        'v' => Path::Down,
                        _ => panic!(),
                    };
                    (p, e)
                })
            })
            .collect()
    }
    fn process_input2(_s: &str) -> Self::Input2 {
        unimplemented!()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let start = v
            .iter()
            .find(|(k, v)| k.y == 0 && **v == Path::Clear)
            .map(|(k, v)| k)
            .cloned()
            .unwrap();

        let max_y = v.keys().max_by_key(|v| v.y).copied().unwrap().y;

        let exit = v
            .iter()
            .find(|(k, v)| k.y == max_y && **v == Path::Clear)
            .map(|(k, v)| k)
            .cloned()
            .unwrap();

        // struct Key {
        //     pos: Point2<i32>,
        // }

        // struct Value {
        //     dir: Vector2<i32>,
        //     dist: usize,
        // }

        // dists.insert(
        //     Key { pos: *start },
        //     Value {
        //         dir: Vector2::new(0, 0),
        //         dist: 0,
        //     },
        // );

        fn depth_first(
            path: &mut Vec<Point2<i32>>,
            locs: &HashMap<Point2<i32>, Path>,
            pos: Point2<i32>,
            exit: Point2<i32>,
            dist: usize,
        ) -> Option<usize> {
            //println!("{:?}", (dist, pos));
            if exit == pos {
                return Some(dist);
            }
            let dirs = [UP, DOWN, LEFT, RIGHT];
            let cur = locs.get(&pos).unwrap();

            dirs.iter()
                .flat_map(|d| {
                    match (cur, *d) {
                        (Path::Clear, _)
                        | (Path::Left, LEFT)
                        | (Path::Right, RIGHT)
                        | (Path::Down, DOWN)
                        | (Path::Up, UP) => {}
                        _ => return None,
                    }
                    let new_pos = pos + d;
                    match locs.get(&new_pos) {
                        Some(Path::Left | Path::Right | Path::Up | Path::Down | Path::Clear) => {
                            if path.iter().find(|p| **p == new_pos).is_none() {
                                path.push(new_pos);
                                let new = depth_first(path, locs, new_pos, exit, dist + 1)
                                    .map(|dist| (d, dist));
                                path.pop();
                                new
                            } else {
                                None
                            }
                        }
                        Some(Path::Forest) | None => None,
                    }
                })
                .max_by_key(|(_, v)| *v)
                .map(|(_, v)| v)
        }

        depth_first(&mut vec![start], &v, start, exit, 0).unwrap()
    }

    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        unimplemented!()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
        94
    )],
    [
        //(foo_sol2, "hi2", 1)
    ]
);
