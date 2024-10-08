use std::io::Result as IoResult;

use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, pt: &Point) -> Point {
        Point {
            x: pt.x + self.x,
            y: pt.y + self.y,
        }
    }
    fn mul(&self, d: i32) -> Point {
        Point {
            x: self.x * d,
            y: self.y * d,
        }
    }
    fn dist(&self, pt2: &Point) -> u32 {
        u32::try_from((self.x - pt2.x).abs() + (self.y - pt2.y).abs()).unwrap()
    }
}
#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Wire {
    dir: Dir,
    mag: i32,
}

impl TryFrom<&str> for Wire {
    type Error = ();
    fn try_from(s: &str) -> Result<Wire, ()> {
        if s.len() < 2 {
            Err(())
        } else {
            let dir = match s.chars().next().unwrap() {
                'R' => Some(Dir::Right),
                'D' => Some(Dir::Down),
                'U' => Some(Dir::Up),
                'L' => Some(Dir::Left),
                _ => None,
            };

            let mag = s.chars().skip(1).collect::<String>().parse::<i32>();

            match (dir, mag) {
                (Some(d), Ok(m)) => Ok(Wire { dir: d, mag: m }),
                _ => Err(()),
            }
        }
    }
}
pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day3.txt")?;
    let mut hm = HashMap::new();
    wires(&s)
        .iter()
        .enumerate()
        .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
    println!("Day 3 P1: {}", find_dist(&hm));
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day3.txt")?;
    let mut hm = HashMap::new();
    wires(&s)
        .iter()
        .enumerate()
        .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
    println!("Day 3 P2: {}", find_potential(&hm));
    Ok(())
}

fn wires(s: &str) -> Vec<Vec<Wire>> {
    s.lines()
        .map(|l| {
            l.split(',')
                .map(|x| Wire::try_from(x).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Space {
    Wire(usize),
}
fn draw_wire(w: &[Wire], space: Space, hm: &mut HashMap<Point, HashMap<Space, u32>>) {
    w.iter()
        .scan((Point { x: 0, y: 0 }, 0), |pt, Wire { dir, mag }| {
            let dir_pt = match dir {
                Dir::Up => Point { x: 0, y: 1 },
                Dir::Down => Point { x: 0, y: -1 },
                Dir::Left => Point { x: -1, y: 0 },
                Dir::Right => Point { x: 1, y: 0 },
            };
            let (mut iter_pt, start_c) = *pt;
            let target = iter_pt.add(&dir_pt.mul(*mag));
            let mut c = start_c;
            while iter_pt != target {
                c += 1;
                iter_pt = iter_pt.add(&dir_pt);
                let e = hm.entry(iter_pt).or_default();
                e.entry(space).or_insert(c);
            }
            *pt = (target, c);
            Some(*pt)
        })
        .for_each(drop);
}

fn find_potential(hm: &HashMap<Point, HashMap<Space, u32>>) -> i32 {
    let mut overlaps = hm
        .iter()
        .filter(|(_k, v)| v.len() >= 2)
        .map(|(_k, v)| {
            let mut entries: Vec<u32> = v.iter().map(|(_k, v)| *v).collect::<Vec<_>>();
            entries.sort();
            entries.iter().take(2).sum()
        })
        .collect::<Vec<u32>>();
    overlaps.sort();
    *overlaps.iter().take(1).next().unwrap() as i32
}

fn find_dist(hm: &HashMap<Point, HashMap<Space, u32>>) -> u32 {
    let mut overlaps = hm
        .iter()
        .filter(|(_k, v)| v.len() >= 2)
        .map(|(k, _v)| Point { x: 0, y: 0 }.dist(k))
        .collect::<Vec<u32>>();
    overlaps.sort();
    *overlaps.iter().take(1).next().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn overlaps() {
        let mut hm = HashMap::new();
        let s = concat!("R8,U5,L5,D3\n", "U7,R6,D4,L4");
        wires(s)
            .iter()
            .enumerate()
            .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
        assert_eq!(find_dist(&hm), 6);

        let mut hm = HashMap::new();
        let s = concat!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        );
        wires(s)
            .iter()
            .enumerate()
            .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
        assert_eq!(find_dist(&hm), 159);

        let mut hm = HashMap::new();
        let s = concat!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        );
        wires(s)
            .iter()
            .enumerate()
            .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
        assert_eq!(find_dist(&hm), 135);

        let mut hm = HashMap::new();
        let s = concat!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        );
        wires(s)
            .iter()
            .enumerate()
            .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
        assert_eq!(find_potential(&hm), 610);

        let mut hm = HashMap::new();
        let s = concat!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        );
        wires(s)
            .iter()
            .enumerate()
            .for_each(|(i, w)| draw_wire(w, Space::Wire(i), &mut hm));
        assert_eq!(find_potential(&hm), 410);
    }
}
