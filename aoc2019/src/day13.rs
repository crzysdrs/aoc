use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use crate::intcode::IntCodeMachine;

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
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![]);

    m.run();

    let grid = m
        .output()
        .chunks(3)
        .into_iter()
        .map(|c| {
            let c = c.collect::<Vec<_>>();
            ((c[0], c[1]), c[2])
        })
        .collect::<HashMap<_, _>>();

    assert!(m.halted());
    println!("Part 1: {}", grid.values().filter(|x| ***x == 2).count());
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
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    codes[0] = 2;
    let mut m = IntCodeMachine::new(codes, vec![]);

    let mut last_ball: Option<Vec<_>> = None;
    let mut grid = HashMap::new();
    let mut final_score = None;
    let score = loop {
        assert!(!m.halted());
        m.run();
        grid.extend(m.output().chunks(3).into_iter().map(|c| {
            let c = c.collect::<Vec<_>>();
            (vec![*c[0], *c[1]], *c[2])
        }));
        while let Some(x) = m.next_output() {}

        let blocks = grid.values().filter(|x| **x == 2).count();
        if let Some(score) = grid.get(&vec![-1, 0]) {
            /* remove score so drawing function doesn't get messed up */
            final_score = Some(*score);
            grid.remove(&vec![-1, 0]);
        }
        //draw(&grid);
        if blocks == 0 {
            break final_score.unwrap();
        }

        let ball_pos = grid
            .iter()
            .find(|(_k, v)| **v == Tile::Ball as isize)
            .map(|(k, _)| k)
            .unwrap();
        let paddle_pos = grid
            .iter()
            .find(|(_k, v)| **v == Tile::Paddle as isize)
            .map(|(k, _)| k)
            .unwrap();

        let joy = match paddle_pos[0].cmp(&ball_pos[0]) {
            Ordering::Equal => JoyStick::Neutral,
            Ordering::Less => JoyStick::Right,
            Ordering::Greater => JoyStick::Left,
        };
        last_ball = Some(ball_pos.to_vec());
        m.feed_input(joy as isize);
    };

    println!("Part 2: {}", score);
    Ok(())
}

fn draw(grid: &HashMap<Vec<isize>, isize>) {
    let min_x = grid.iter().min_by_key(|(x, _y)| x[0]).unwrap().0[0];
    let min_y = grid.iter().min_by_key(|(x, _y)| x[1]).unwrap().0[1];
    let max_x = grid.iter().max_by_key(|(x, _y)| x[0]).unwrap().0[0];
    let max_y = grid.iter().max_by_key(|(x, _y)| x[1]).unwrap().0[1];

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let p = grid.get(&vec![x, y]).unwrap_or(&0);
            print!(
                "{}",
                match *p {
                    0 => " ",
                    1 => "▉",
                    2 => "B",
                    3 => "_",
                    4 => "o",
                    _ => panic!("WHAT"),
                }
            )
        }
        println!();
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(false);
    }
}
