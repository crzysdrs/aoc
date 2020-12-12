use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use cgmath::{Vector2, Point2};

#[derive(PartialEq,Copy,Clone,Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Copy,Clone,Debug)]
pub enum Rotate {
    Left,
    Right,
}

impl Dir {
    fn vec2(&self) -> Vector2<i32> {
        let (x, y) = match self {
            Dir::North => (0, 1),
            Dir::South => (0, -1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0)
        };
        Vector2::new(x, y)
    }
    fn rot(&self, rot: Rotate, angle: i32) -> Dir {
        let angle = match rot {
            Rotate::Left => -angle,
            Rotate::Right => angle,
        };
        let dirs = [Dir::North, Dir::East, Dir::South, Dir::West];
        let start = dirs.iter().position(|x| x == self).unwrap();
        let count = (angle / 90 ) % 4;
        dirs[((start as isize + count as isize + 4) % 4) as usize]
    }
}

struct Ship {
    pos : Point2<i32>,
    face: Dir,    
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos : Point2::new(0, 0),
            face: Dir::East,
        }
    }
    fn run(&mut self, i: &Instr) {
        match i {
            Instr::Forward(count) => {
                self.pos += self.face.vec2() * *count;
            }
            Instr::Turn(dir, count) => {
                self.face = self.face.rot(*dir, *count);
            }
            Instr::Move(dir, count) => {
                self.pos += dir.vec2() * *count;
            }
        }
    }
}


#[derive(Debug)]
struct ShipWaypoint {
    waypoint: Vector2<i32>,
    pos : Point2<i32>,
}

impl ShipWaypoint {
    fn new() -> ShipWaypoint {
        ShipWaypoint {
            pos : Point2::new(0, 0),
            waypoint: Vector2::new(10, 1),
        }
    }
    fn run(&mut self, i: &Instr) {
        match i {
            Instr::Forward(count) => {
                self.pos += self.waypoint * *count;
            }
            Instr::Turn(dir, count) => {
                let dir = match dir {
                    Rotate::Left => *count,
                    Rotate::Right => -*count,
                };

                self.waypoint = match (dir + 360) % 360{
                    0 => self.waypoint,
                    90 => Vector2::new(-self.waypoint.y, self.waypoint.x),
                    180 => Vector2::new(-self.waypoint.x, -self.waypoint.y),
                    270 => Vector2::new(self.waypoint.y, -self.waypoint.x),
                    _ => panic!()
                }
            }
            Instr::Move(dir, count) => {
                self.waypoint += dir.vec2() * *count;
            }
        }
    }
}
pub enum Instr {
    Move(Dir, i32),
    Turn(Rotate, i32),
    Forward(i32)
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 12;
    type Input = Instr;
    type Sol1 = i32;
    type Sol2 = i32;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                let c = l.chars().nth(0).unwrap();                
                let count = l[1..].parse::<i32>().unwrap();
                let instr = match c {
                    'N' | 'S' | 'E' | 'W' => {
                        let dir = match c { 
                            'N' => Dir::North,
                            'S' => Dir::South,
                            'E' => Dir::East,
                            'W' => Dir::West,
                            _ => panic!()
                        };
                        Instr::Move(dir, count)
                    }
                    'L' | 'R'  => {
                        let rot  = match c {
                            'L' => Rotate::Left,
                            'R' => Rotate::Right,
                            _ => panic!()
                        };
                        assert_eq!(count % 90, 0);
                        Instr::Turn(rot, count)
                    }
                    'F' => {
                        Instr::Forward(count)
                    }
                    _ => {
                        panic!()
                    }
                };
                Ok(instr)
            }).collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let last = v.iter().scan(Ship::new(), |ship, instr| {
            ship.run(&instr);
            Some(ship.pos)
        }).last().unwrap();

        last.x.abs() + last.y.abs()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let last = v.iter().scan(ShipWaypoint::new(), |ship, instr| {
            ship.run(&instr);
            Some(ship.pos)
        }).last().unwrap();

        last.x.abs() + last.y.abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Dir::North.rot(Rotate::Right, 90), Dir::East);
        assert_eq!(Dir::North.rot(Rotate::Left, 90), Dir::West);

        let s = "F10
N3
F7
R90
F11";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 25);
        assert_eq!(Solution::p2(&v), 286);
    }
}
