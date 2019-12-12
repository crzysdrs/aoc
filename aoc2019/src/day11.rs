use crate::intcode::IntCodeMachine;
use std::io::Result as IoResult;

enum Color {
    Black,
    White,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn mv(&self, pos: (i32, i32)) -> (i32, i32) {
        let update = match self {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        (pos.0 + update.0, pos.1 + update.1)
    }
    fn rotate(&mut self, left: bool) {
        let dirs = &[Dir::Up, Dir::Left, Dir::Down, Dir::Right];
        let cur = dirs.iter().position(|x| *x == *self).unwrap();
        let next = if left { cur + 1 } else { dirs.len() + cur - 1 } % dirs.len();
        *self = dirs[next];
    }
}
impl Color {
    fn color_val(&self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
    fn from_color_val(val: isize) -> Color {
        match val {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color val"),
        }
    }
}

pub fn p1() -> IoResult<()> {
    let mut panel = std::collections::HashMap::new();
    let codes = std::fs::read_to_string("input/day11.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![]);

    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    while !m.halted() {
        let p = panel.entry(pos).or_insert(Color::Black);
        m.feed_input(p.color_val());
        m.run();
        *p = Color::from_color_val(m.next_output().unwrap());
        m.run();
        let cmd_dir = m.next_output().unwrap();
        match cmd_dir {
            0 => dir.rotate(true),
            1 => dir.rotate(false),
            _ => panic!("Unexpected Direction"),
        }
        pos = dir.mv(pos);
    }
    println!("Day 11 Part 1 {}", panel.len());
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let mut panel = std::collections::HashMap::new();
    let codes = std::fs::read_to_string("input/day11.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut m = IntCodeMachine::new(codes, vec![]);

    let mut pos = (0, 0);
    let mut dir = Dir::Up;
    panel.entry(pos).or_insert(Color::White);
    while !m.halted() {
        let p = panel.entry(pos).or_insert(Color::Black);
        m.feed_input(p.color_val());
        m.run();
        *p = Color::from_color_val(m.next_output().unwrap());
        m.run();
        let cmd_dir = m.next_output().unwrap();
        match cmd_dir {
            0 => dir.rotate(true),
            1 => dir.rotate(false),
            _ => panic!("Unexpected Direction"),
        }
        pos = dir.mv(pos);
    }
    let tile_pos: Vec<_> = panel.iter().map(|(k, _v)| k).collect();
    let min_x: (i32, i32) = **tile_pos.iter().min_by_key(|(x, _y)| x).unwrap();
    let min_y = **tile_pos.iter().min_by_key(|(_x, y)| y).unwrap();
    let max_x = **tile_pos.iter().max_by_key(|(x, _y)| x).unwrap();
    let max_y = **tile_pos.iter().max_by_key(|(_x, y)| y).unwrap();

    println!("Day 11 Part 2:");
    for y in (min_y.1..=max_y.1).rev() {
        for x in min_x.0..=max_x.0 {
            let p = panel.entry((x, y)).or_insert(Color::Black);
            print!(
                "{}",
                match *p {
                    Color::Black => "▉",
                    Color::White => " ",
                }
            )
        }
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(true);
    }
}
