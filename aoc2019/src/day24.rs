use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashSet;

#[derive(Copy,Clone,Hash,Eq,PartialEq)]
enum Tile {
    Bug,
    Empty
}

fn draw(width: usize, tiles: &[Tile]) {
    for y in 0..width {
        for x in 0..width {
            let c = match tiles[y * width + x] {
                Tile::Bug => '#',
                Tile::Empty => '.',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn bio_rating(tiles: &[Tile]) -> u32 {
    tiles.iter().enumerate().map(
        |(i, t)| {
            match t {
                Tile::Bug => 1 << i,
                Tile::Empty => 0,
            }
        }
    ).sum()
}

pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day24.txt")?;
//     let s = "....#
// #..#.
// #..##
// ..#..
// #....";
    let mut tiles = s.trim().chars().filter(|x| *x != '\n')
        .map(|x|
             match x {
                 '.' => Tile::Empty,
                 '#' => Tile::Bug,
                 c => panic!("Unhandled char {}", c),
             }
        ).collect::<Vec<_>>();

    let width = 5;
    let mut next = vec![Tile::Empty; tiles.len()];

    let mut gen_tiles = HashSet::<Vec<Tile>>::new();
    'done : for g in 0.. {
        println!("Gen {}", g);
        draw(width, &tiles);
        for y in 0..width {
            for x in 0..width {
                let mut count = 0;
                let target_idx = width * y + x;
                for (i,j) in &[(0,1), (1,0), (0,-1), (-1, 0)] {
                    let y = y as isize;
                    let x = x as isize;
                    let width = width as isize;
                    if y + i >= 0 && y + i < width
                        && x + j >=0 && x + j < width {                            
                            let idx = width * (y + i) + x + j;
                            match tiles[idx as usize] {
                                Tile::Bug => {count += 1;}
                                Tile::Empty => {}
                            }
                        }
                }
                next[target_idx] = match tiles[target_idx] {
                    Tile::Bug if count != 1 => Tile::Empty,
                    Tile::Empty if count == 1 || count == 2 => Tile::Bug,
                    a => a
                };
            }
        }
        if let Some(old) = gen_tiles.get(&next) {
            println!("Bio {}", bio_rating(&old));
            break 'done;
        }
        gen_tiles.insert(next.clone());
        std::mem::swap(&mut next, &mut tiles);
    }
    draw(width, &tiles);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    unimplemented!("Part 2")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(false);
    }
}
