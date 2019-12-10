use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Index, IndexMut};
use std::convert::TryFrom;
enum Space {
    Asteroid,
    Empty,
}

#[derive(PartialEq, Eq,Clone,Debug)]
enum Seen {
    Start,
    Asteroid,
    Blocked,
}

#[derive(PartialEq, Eq,Debug, Copy,Clone)]
struct Point {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Grid {
    grid : Vec<Vec<Option<Seen>>>,    
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            grid: vec![vec![None; size]; size],
        }
    }

    fn iter(&self) -> impl Iterator<Item=&Vec<Option<Seen>>> {
        self.grid.iter()
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Index<Point> for Grid {
    type Output = Option<Seen>;
    fn index(&self, pt: Point) -> &Self::Output {
        &self.grid[usize::try_from(pt.y).unwrap()][usize::try_from(pt.x).unwrap()]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, pt: Point) -> &mut Self::Output {
        &mut self.grid[usize::try_from(pt.y).unwrap()][usize::try_from(pt.x).unwrap()]
    }
}

impl Point {
    fn sub(&self, pt: &Point) -> Result<Point, ()> {
        let (x, overflow) = self.x.overflowing_sub(pt.x);
        let (y, overflow2) = self.y.overflowing_sub(pt.y);
        if overflow || overflow2 {
            Err(())
        } else {
            Ok(Point {
                x, y
            })
        }
    }

    fn add(&self, pt: &Point) -> Result<Point, ()> {
        let (x, overflow) = self.x.overflowing_add(pt.x);
        let (y, overflow2) = self.y.overflowing_add(pt.y);
        if overflow || overflow2 {
            Err(())
        } else {
            Ok(Point {
                x, y
            })
        }
    }
}

fn find_best_asteroid(s: &str) -> ((usize, usize), usize) {
    let space =s
        .lines()
        .map(|l| {
             l.chars()
                .map(|x| match x{
                    '.' => Space::Empty,
                    '#' => Space::Asteroid,
                    _ => panic!("Bad Entry"),
                })
                .collect::<Vec<_>>()
        })        
        .collect::<Vec<_>>();

    let size = space.len();
    let asteroids = space.iter().flatten().enumerate().flat_map(|(i,x)| if let Space::Asteroid = x {
        Some(i)
    } else {
        None
    }).map(|i| Point {y:  i32::try_from(i / size).unwrap(), x: i32::try_from(i % size).unwrap() })
        .collect::<Vec<_>>();

    
    let max = asteroids.iter().map(|start| {
        let mut grid = Grid::new(size);
        for a2 in asteroids.iter() {
            match a2 {
                a if start == a2 => {
                    grid[*a] = Some(Seen::Start);
                },
                target => {
                    match grid[*target] {
                        Some(Seen::Blocked) => {},
                        Some(Seen::Start) | Some(Seen::Asteroid) => panic!("Dupe Asteroid?"),
                        None => {
                            fn update_grid(grid: &mut Grid, start: &Point, target : &Point) -> Result<(), ()> {
                                //println!("Pt {:?} {:?}", start, target);
                                let incr = &target.sub(start)?;
                                let incr_gcd = gcd(incr.x, incr.y).abs();
                                let incr = Point {
                                    x: incr.x / incr_gcd,
                                    y: incr.y / incr_gcd,
                                };
                                //println!("GCD: {}, Incr {:?}", incr_gcd, incr);
                                let mut pt = target.add(&incr)?;
                                let size = i32::try_from(grid.len()).unwrap();
                                loop {
                                    //println!("Pt {:?}", pt);
                                    if pt.x >= 0 && pt.x < size && pt.y >= 0 && pt.y < size {
                                        grid[pt] = Some(Seen::Blocked);
                                        pt = pt.add(&incr)?;
                                    } else {
                                        break;
                                    }
                                }
                                Ok(())
                            }
                            grid[*target] = Some(Seen::Asteroid);
                            let _ = update_grid(&mut grid, &start, &target);
                        }
                    }
                }
            }
        }        
        //println!("start {:?} {:?}", start, grid);
        println!("{}", grid.iter().flatten().flatten()
                 .flat_map(|x| if *x == Seen::Asteroid { Some(0) } else {None})
                 .count());
        (
            start.clone(),
            grid.iter().flatten().flatten()
                .flat_map(|x| if *x == Seen::Asteroid { Some(0) } else {None})
                .count())
            
    }).max_by_key(|(_, k)| *k).unwrap();
    
    (
        (usize::try_from(max.0.x).unwrap(),
         usize::try_from(max.0.y).unwrap()),
            max.1)
}

pub fn p1() -> IoResult<()> {
    println!("Part 2 {:?} ", find_best_asteroid(&std::fs::read_to_string("input/day10.txt")?));
    
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
        assert_eq!(
            find_best_asteroid(&
                               concat!(".#..#\n",                                  
                                       ".....\n",
                                       "#####\n",
                                       "....#\n",
                                       "...##\n")
            ), ((3,4), 8));

        assert_eq!(
            find_best_asteroid(&                           
                               "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ), ((5,8), 33));

        assert_eq!(
            find_best_asteroid(&                                                          
                               "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            ), ((1,2), 35));

        assert_eq!(
            find_best_asteroid(&
                               ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
              
            ), ((6,3), 41));

        assert_eq!(
            find_best_asteroid(&
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"           
            ), ((11,13), 210));
        
    }
}
