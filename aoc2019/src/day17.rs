use crate::intcode::IntCodeMachine;
use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::{HashMap, HashSet};
use std::io::Result as IoResult;

use itertools::Itertools;

#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Copy, Clone)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Movement {
    Stop,
    Forward(usize),
    Right,
    Left,
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Movement::Stop => "S".to_string(),
                Movement::Forward(n) => n.to_string(),
                Movement::Right => "R".to_string(),
                Movement::Left => "L".to_string(),
            }
        )
    }
}

impl Dir {
    #[allow(unused)]
    fn from_vec(v: Vector2<i32>) -> Dir {
        match v {
            Vector2 { x: 0, y: 1 } => Dir::North,
            Vector2 { x: 0, y: -1 } => Dir::South,
            Vector2 { x: 1, y: 0 } => Dir::East,
            Vector2 { x: -1, y: 0 } => Dir::West,
            _ => panic!("Bad Direction"),
        }
    }
    fn rotate(&self, left: bool) -> Dir {
        let dirs = &[Dir::North, Dir::West, Dir::South, Dir::East];
        let cur = dirs.iter().position(|x| *x == *self).unwrap();
        let next = if left { cur + 1 } else { dirs.len() + cur - 1 } % dirs.len();
        dirs[next]
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

#[allow(unused)]
fn draw(grid: &HashMap<Point2<i32>, char>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in (min_y..=max_y) {
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y));
            print!("{}", p.unwrap_or(&'?'))
        }
        println!();
    }
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day17.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let mut machine = IntCodeMachine::new(codes, vec![]);

    machine.run();

    let grid = machine
        .output()
        .iter()
        .map(|x| char::from(*x as u8))
        .collect::<String>()
        .split('\n')
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    draw(&grid);
    let scaffolds = grid
        .iter()
        .filter(|(_p, c)| **c == '#')
        .map(|(p, _c)| p)
        .filter(|p| {
            let ps = [
                Point2::new(p.x + 1, p.y),
                Point2::new(p.x - 1, p.y),
                Point2::new(p.x, p.y + 1),
                Point2::new(p.x, p.y - 1),
            ];
            ps.iter().map(|p| grid.get(p)).all(|x| x == Some(&'#'))
        })
        .map(|p| (p.x * p.y) as u32)
        .sum::<u32>();

    println!("part 1: {}", scaffolds);
    Ok(())
}

fn repeated_substring<T>(s: &[T]) -> Option<&[T]>
where
    T: PartialEq,
{
    let mut subs: HashMap<_, _> = HashMap::new();
    let mut res_length = 0;
    let mut index = 0;
    let n = s.len();
    for i in 1..s.len() {
        for j in i + 1..n {
            if s[i - 1] == s[j - 1] && *subs.get(&(i - 1, j - 1)).unwrap_or(&0) < j - i {
                *subs.entry((i, j)).or_insert(0) = *subs.get(&(i - 1, j - 1)).unwrap_or(&0) + 1;

                if *subs.get(&(i, j)).unwrap_or(&0) > res_length {
                    res_length = *subs.get(&(i, j)).unwrap_or(&0);
                    index = std::cmp::max(i, index);
                }
            }
        }
    }

    if res_length > 0 {
        Some(&s[index - res_length..index])
    } else {
        None
    }
}

pub fn p2() -> IoResult<()> {
    let mut codes = std::fs::read_to_string("input/day17.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let mut machine = IntCodeMachine::new(codes.clone(), vec![]);

    machine.run();

    let grid = machine
        .output()
        .iter()
        .map(|x| char::from(*x as u8))
        .collect::<String>()
        .split('\n')
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    let scaffolds = grid
        .iter()
        .filter(|(_p, c)| **c == '#')
        .map(|(p, _c)| p)
        .filter(|p| {
            let ps = [
                Point2::new(p.x + 1, p.y),
                Point2::new(p.x - 1, p.y),
                Point2::new(p.x, p.y + 1),
                Point2::new(p.x, p.y - 1),
            ];
            ps.iter().map(|p| grid.get(p)).all(|x| x == Some(&'#'))
        })
        .collect::<HashSet<_>>();

    let start: Point2<_> = *grid
        .iter()
        .find(|(_x, y)| **y == '^')
        .map(|(x, _y)| x)
        .unwrap();
    let mut current = start;
    let mut dir = Dir::South;
    let movements = (0..)
        .map(|_| {
            let next = point_dir(&current, &dir);
            if scaffolds.contains(&current) {
                current = next;
                Movement::Forward(1)
            } else if let Some('#') = grid.get(&next) {
                current = next;
                Movement::Forward(1)
            } else if let Some('#') = grid.get(&point_dir(&current, &dir.rotate(true))) {
                dir = dir.rotate(true);
                Movement::Right
            } else if let Some('#') = grid.get(&point_dir(&current, &dir.rotate(false))) {
                dir = dir.rotate(false);
                Movement::Left
            } else {
                Movement::Stop
            }
        })
        .take_while(|x| *x != Movement::Stop)
        .coalesce(|x, y| match (x, y) {
            (Movement::Forward(n), Movement::Forward(m)) => Ok(Movement::Forward(m + n)),
            (x, y) => Err((x, y)),
        })
        .collect::<Vec<_>>();

    println!("{:?}", movements);

    let substring = repeated_substring(&movements).unwrap();
    let mut i = 0;
    println!("Unmodified Main:");
    for i in &movements {
        print!("{},", i);
    }
    println!();
    let methods = [
        &movements[0..8],
        &movements[8..][..6],
        &movements[movements.len() - 8..],
    ];
    println!("Methods:");
    for m in &methods {
        for i in *m {
            print!("{},", i);
        }
        println!();
    }
    println!("Main ish:");
    while i < movements.len() {
        let mut seen = false;
        for (method, name) in methods.iter().zip(&['A', 'B', 'C']) {
            if movements[i..].starts_with(method) {
                print!("{},", name);
                i += method.len();
                seen = true;
                break;
            }
        }
        if !seen {
            print!("{},", movements[i]);
            i += 1;
        }
    }

    println!();
    println!("A: ");
    for i in substring {
        print!("{},", i);
    }
    codes[0] = 2;

    println!();
    let routines = concat!(
        "A,B,A,B,C,B,C,A,B,C\n", //main
        "R,4,R,10,R,8,R,4\n",    //A
        "R,10,R,6,R,4\n",        //B
        "R,4,L,12,R,6,L,12\n",   //C
        "n\n",                   //continuous feed
    )
    .chars()
    .map(|x| x as isize)
    .collect();
    let mut machine = IntCodeMachine::new(codes, routines);

    loop {
        machine.run();
        //println!("{:?}", machine.output().iter().collect::<Vec<_>>());
        print!(
            "{}",
            machine
                .output()
                .iter()
                .map(|x| char::from(*x as u8))
                .collect::<String>()
        );
        if machine.halted() {
            break;
        }
    }
    Ok(())
}
