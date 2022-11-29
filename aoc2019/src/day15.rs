use crate::intcode::IntCodeMachine;
use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::io::Result as IoResult;

use std::collections::{HashMap, VecDeque};

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Dir {
    fn from_vec(v: Vector2<i32>) -> Dir {
        match v {
            Vector2 { x: 0, y: 1 } => Dir::North,
            Vector2 { x: 0, y: -1 } => Dir::South,
            Vector2 { x: 1, y: 0 } => Dir::East,
            Vector2 { x: -1, y: 0 } => Dir::West,
            _ => panic!("Bad Direction"),
        }
    }
}

fn point_dir(p: &Point2<i32>, d: &Dir) -> Point2<i32> {
    let mut p = *p;
    match d {
        Dir::North => {
            p.y += 1;
        }
        Dir::South => {
            p.y -= 1;
        }
        Dir::East => {
            p.x += 1;
        }
        Dir::West => {
            p.x -= 1;
        }
    }
    p
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Status {
    HitWall = 0,
    Moved = 1,
    AtOxygen = 2,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

#[allow(unused)]
fn around<'a, T>(
    pt: &'a Point2<i32>,
    grid: &'a HashMap<Point2<i32>, T>,
) -> impl Iterator<Item = (Point2<i32>, &'a T)> + 'a {
    let dirs = vec![Dir::North, Dir::South, Dir::East, Dir::West];
    let pt = *pt;
    dirs.into_iter()
        .map(move |d| point_dir(&pt, &d))
        .flat_map(move |p| grid.get(&p).map(|t| (p, t)))
}

#[allow(unused)]
fn draw(grid: &HashMap<Point2<i32>, Tile>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y));
            print!(
                "{}",
                match p {
                    Some(Tile::Empty) => ".",
                    Some(Tile::Wall) => "▉",
                    Some(Tile::Oxygen) => "O",
                    //Some(Tile::Unknown) => "?",
                    None => " ",
                }
            )
        }
        println!();
    }
}

#[allow(unused)]
fn dijkstra(start: &Point2<i32>, grid: &HashMap<Point2<i32>, Tile>) -> HashMap<Point2<i32>, u32> {
    let mut dist = HashMap::<Point2<i32>, u32>::new();
    dist.insert(*start, 0);
    let mut q: VecDeque<_> = grid.keys().collect();

    while !q.is_empty() {
        let min = q
            .iter()
            .enumerate()
            .min_by_key(|(_i, k)| *dist.entry(***k).or_insert(std::u32::MAX))
            .map(|(k, v)| (k, **v))
            .unwrap();
        let min_dist = *dist.entry(min.1).or_insert(std::u32::MAX);
        let _v = q.remove(min.0);
        let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
        for d in dirs.iter() {
            if grid.get(&point_dir(&min.1, d)).is_some() {
                let next = dist
                    .entry(point_dir(&min.1, d))
                    .or_insert_with(|| min_dist.saturating_add(1));
                *next = std::cmp::min(*next, min_dist.saturating_add(1));
            }
        }
    }
    dist
}

fn bfs<T>(m: IntCodeMachine, start: Point2<i32>, mut visit: T) -> HashMap<Point2<i32>, Tile>
where
    T: FnMut(u32, &Point2<i32>, &IntCodeMachine, Status),
{
    let mut grid = HashMap::new();
    grid.insert(start, Tile::Empty);
    let mut worklist = VecDeque::from(vec![(0, m, start, start)]);
    while let Some((depth, mut m, mut pos, v)) = worklist.pop_front() {
        //println!("{:?} {:?}", pos, v);
        m.run();
        let moved = if pos != v {
            let dir = Dir::from_vec(v - pos);
            m.feed_input(dir.to_isize().unwrap());
            m.run();
            let read_status = m.next_output().unwrap();
            //println!("Read Status {:?}", read_status);
            let status = Status::from_isize(read_status).unwrap();
            //println!("Status {:?}", status);
            let next_pos = point_dir(&pos, &dir);
            let moved = match status {
                Status::HitWall => {
                    grid.insert(next_pos, Tile::Wall);
                    false
                }
                Status::AtOxygen => {
                    pos = next_pos;
                    grid.insert(pos, Tile::Oxygen);
                    true
                }
                Status::Moved => {
                    pos = next_pos;
                    let entry = grid.entry(pos).or_insert(Tile::Empty);
                    *entry = Tile::Empty;
                    true
                }
            };
            visit(depth, &pos, &m, status);
            moved
        } else {
            true
        };

        if moved {
            let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
            for d in dirs.iter() {
                let new_pt = point_dir(&pos, d);
                if grid.get(&new_pt).is_none() {
                    worklist.push_back((depth + 1, m.clone(), pos, new_pt));
                }
            }
        }
    }
    grid
}
pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day15.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let m = IntCodeMachine::new(codes, vec![]);

    let mut min_d = std::u32::MAX;
    let grid = bfs(m, Point2::new(0, 0), |d, _l, _m, s| {
        if let Status::AtOxygen = s {
            min_d = std::cmp::min(d, min_d);
        }
    });

    draw(&grid);
    //    let dists = dijkstra(&Point2::new(0,0), &grid);
    //let oxygen = grid.iter().find(|(k, v)| **v == Tile::Oxygen).unwrap();
    println!(
        "Part 1: {}",
        //dists.get(oxygen.0).unwrap()
        min_d
    );
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day15.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let m = IntCodeMachine::new(codes, vec![]);

    let mut fill_ox = None;
    let mut max_d = 0;
    let _grid = bfs(m, Point2::new(0, 0), |_d, l, m, s| {
        if let Status::AtOxygen = s {
            fill_ox = Some((*l, m.clone()));
        }
    });

    let (l, m) = fill_ox.unwrap();
    bfs(m, l, |d, _l, _m, s| match s {
        Status::AtOxygen | Status::Moved => max_d = std::cmp::max(max_d, d),
        _ => {}
    });
    println!(
        "Part 2: {}",
        //dists.get(oxygen.0).unwrap()
        max_d
    );
    Ok(())
}
