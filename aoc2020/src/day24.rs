use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use cgmath::{Point2, Vector2};
use regex::Regex;

pub enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

enum Tile {
    White,
    Black
}

impl Tile {
    fn toggle(&mut self) {
        *self = match self {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        }
    }
}

impl Dir {
    fn offset(&self) -> Vector2<i32> {
        match self {
            Dir::E => Vector2::new(1, 0),
            Dir::SE => Vector2::new(1, -1),
            Dir::SW => Vector2::new(0, -1),
            Dir::W => Vector2::new(-1, 0),
            Dir::NW => Vector2::new(-1, 1),
            Dir::NE => Vector2::new(0, 1),
        }
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 24;
    type Input = Vec<Dir>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let dir = Regex::new("e|se|sw|w|nw|ne").unwrap();
        Ok(r.lines().flatten()
            .map(|l| {
                dir.captures_iter(&l).map(|d| {
                    match d.get(0).unwrap().as_str() {
                        "e" => Dir::E,
                        "se" => Dir::SE,
                        "sw" => Dir::SW,
                        "w" => Dir::W,
                        "nw" => Dir::NW,
                        "ne" => Dir::NE,
                        _ => unreachable!(),
                    }
                }).collect()
            }).collect())
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut grid = HashMap::<Point2<i32>, Tile>::new();
        
        for dirs in v {
            let last = dirs.iter().scan(Point2::new(0, 0), |pos, dir| {
                *pos += dir.offset();
                Some(*pos)
            }).last().unwrap();
            grid.entry(last).and_modify(|t| t.toggle()).or_insert(Tile::Black);
        };

        grid.values().filter(|x| matches!(x, Tile::Black)).count()
            
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
         let mut grid = HashMap::<Point2<i32>, Tile>::new();
        
        for dirs in v {
            let last = dirs.iter().scan(Point2::new(0, 0), |pos, dir| {
                *pos += dir.offset();
                Some(*pos)
            }).last().unwrap();
            grid.entry(last).and_modify(|t| t.toggle()).or_insert(Tile::Black);
        };

        let dirs =  vec![Dir::E,
                         Dir::SE,
                         Dir::SW,
                         Dir::W,
                         Dir::NW,
                         Dir::NE];

        let mut day = HashMap::<Point2<i32>, usize>::new();
        for _ in 0..100 {
            day.clear();
            grid.iter().filter(|(_k, x)| matches!(x, Tile::Black))
                .for_each(|(p, _)| {
                    for d in &dirs {
                        day.entry(p + d.offset()).and_modify(|v| *v += 1).or_insert(1);
                    }
                    day.entry(*p).or_insert(0); /* black tile with no adjacent needs to be represented */
                }            
                );
            day.iter().for_each(|(p, &v)| {
                match grid.entry(*p).or_insert(Tile::White) {
                    t @ Tile::Black if v == 0 || v > 2 => *t = Tile::White,
                    t @ Tile::White if v == 2 => *t = Tile::Black,
                    _ => {},
                }
            })
        }
        grid.values().filter(|x| matches!(x, Tile::Black)).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 10);
        assert_eq!(Solution::p2(&v), 2208);
    }
}
