use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Result as IoResult;

use crate::intcode::IntCodeMachine;
use cgmath::Point2;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Eq, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day13.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![]);

    m.run();

    let grid = m
        .output()
        .chunks(3)
        .map(|c| (Point2::new(c[0], c[1]), Tile::from_isize(c[2]).unwrap()))
        .collect::<HashMap<_, _>>();

    assert!(m.halted());
    println!(
        "Part 1: {}",
        grid.values().filter(|x| **x == Tile::Block).count()
    );
    Ok(())
}

#[derive(Debug)]
enum JoyStick {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

pub fn p2() -> IoResult<()> {
    let mut codes = std::fs::read_to_string("input/day13.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    codes[0] = 2;
    let mut m = IntCodeMachine::new(codes, vec![]);

    let mut grid = HashMap::new();
    let mut final_score = None;
    let score_point = Point2::new(-1, 0);

    loop {
        assert!(!m.halted());
        m.run();
        grid.extend(m.output().chunks(3).flat_map(|c| {
            let p = Point2::new(c[0], c[1]);
            if p == score_point {
                final_score = Some(c[2]);
                None
            } else {
                Some((p, Tile::from_isize(c[2]).unwrap()))
            }
        }));

        let blocks = grid.values().filter(|x| **x == Tile::Block).count();
        //draw(&grid);
        if blocks == 0 {
            break;
        }

        let ball_pos = grid
            .iter()
            .find(|(_k, v)| **v == Tile::Ball)
            .map(|(k, _)| k)
            .unwrap();
        let paddle_pos = grid
            .iter()
            .find(|(_k, v)| **v == Tile::Paddle)
            .map(|(k, _)| k)
            .unwrap();

        let joy = match paddle_pos.x.cmp(&ball_pos.x) {
            Ordering::Equal => JoyStick::Neutral,
            Ordering::Less => JoyStick::Right,
            Ordering::Greater => JoyStick::Left,
        };
        m.feed_input(joy as isize);
    }

    println!("Part 2: {:?}", final_score);
    Ok(())
}

#[allow(unused)]
fn draw(grid: &HashMap<Point2<isize>, Tile>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y)).unwrap_or(&Tile::Empty);
            print!(
                "{}",
                match *p {
                    Tile::Empty => " ",
                    Tile::Wall => "▉",
                    Tile::Block => "B",
                    Tile::Paddle => "_",
                    Tile::Ball => "o",
                }
            )
        }
        println!();
    }
}
