use std::convert::TryFrom;
use std::io::Result as IoResult;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Space {
    Asteroid,
    Empty,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Seen {
    Start,
    Asteroid,
    Blocked,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Option<Seen>>>,
}

impl Grid {
    fn new(height: usize, width: usize) -> Grid {
        //println!("Height {} Width: {}", height, width);
        Grid {
            grid: vec![vec![None; width]; height],
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<Option<Seen>>> {
        self.grid.iter()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
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
            Ok(Point { x, y })
        }
    }

    fn add(&self, pt: &Point) -> Result<Point, ()> {
        let (x, overflow) = self.x.overflowing_add(pt.x);
        let (y, overflow2) = self.y.overflowing_add(pt.y);
        if overflow || overflow2 {
            Err(())
        } else {
            Ok(Point { x, y })
        }
    }
}

fn visible_asteroids(
    height: usize,
    width: usize,
    start: Point,
    asteroids: &[Point],
) -> (Point, Vec<Point>) {
    let start = &start;
    let mut grid = Grid::new(height, width);
    for a2 in asteroids.iter() {
        match a2 {
            a if start == a2 => {
                grid[*a] = Some(Seen::Start);
            }
            target => {
                match grid[*target] {
                    Some(Seen::Blocked) => {}
                    Some(Seen::Start) | Some(Seen::Asteroid) => panic!("Dupe Asteroid?"),
                    None => {
                        fn update_grid(
                            grid: &mut Grid,
                            start: &Point,
                            target: &Point,
                        ) -> Result<(), ()> {
                            //println!("Pt {:?} {:?}", start, target);
                            let incr = &target.sub(start)?;
                            let incr_gcd = gcd(incr.x, incr.y).abs();
                            let incr = Point {
                                x: incr.x / incr_gcd,
                                y: incr.y / incr_gcd,
                            };
                            //println!("GCD: {}, Incr {:?}", incr_gcd, incr);
                            let mut pt = target.add(&incr)?;
                            loop {
                                //println!("Pt {:?}", pt);
                                if pt.x >= 0
                                    && pt.x < i32::try_from(grid.width()).unwrap()
                                    && pt.y >= 0
                                    && pt.y < i32::try_from(grid.height()).unwrap()
                                {
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
    // println!("{}", grid.iter().flatten().flatten()
    //          .flat_map(|x| if *x == Seen::Asteroid { Some(0) } else {None})
    //          .count());
    (
        start.clone(),
        grid.iter()
            .flatten()
            .enumerate()
            .flat_map(|(i, v)| v.as_ref().map(|v| (i, v)))
            .flat_map(|(i, x)| {
                if *x == Seen::Asteroid {
                    Some(Point {
                        y: i32::try_from(i / grid.width()).unwrap(),
                        x: i32::try_from(i % grid.width()).unwrap(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
    )
}

fn destroy_asteroids(start: Point, s: &str) -> Vec<Point> {
    let mut space = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    '.' => Space::Empty,
                    '#' => Space::Asteroid,
                    _ => panic!("Bad Entry"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ordered = vec![];
    loop {
        let asteroids = all_asteroids(&space);
        //println!("Asteroids {:?}", asteroids);
        let (_, vis) = visible_asteroids(space.len(), space[0].len(), start, &asteroids);
        let mut targets = vis
            .iter()
            .map(|orig| (orig, orig.sub(&start).unwrap()))
            .map(|(orig, off)| {
                (
                    orig,
                    //((off.y as f64).atan2(off.x as f64) + (5.0 * std::f64::consts::PI / 2.0)) % std::f64::consts::PI * 2.0
                    ((off.y as f64).atan2(off.x as f64)),
                )
            })
            .collect::<Vec<_>>();
        targets.reverse();
        //println!("Targets len {}", targets.len());
        if targets.len() == 0 {
            break;
        }
        targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        //println!("{:?}", targets);
        //println!("{}", targets.len());
        let targets = targets
            .iter()
            .skip_while(|(_, rad)| *rad < -std::f64::consts::PI / 2.0)
            .chain(
                targets
                    .iter()
                    .take_while(|(_, rad)| *rad < -std::f64::consts::PI / 2.0),
            );

        for t in targets.clone() {
            //println!("{:?}", t);
            assert_eq!(Space::Asteroid, space[t.0.y as usize][t.0.x as usize]);
            space[t.0.y as usize][t.0.x as usize] = Space::Empty;
        }
        ordered.extend(targets.map(|(p, _)| **p));
    }
    ordered
}

fn all_asteroids(space: &[Vec<Space>]) -> Vec<Point> {
    let width = space[0].len();
    space
        .to_vec()
        .iter()
        .flatten()
        .enumerate()
        .flat_map(|(i, x)| {
            if let Space::Asteroid = x {
                Some(i)
            } else {
                None
            }
        })
        .map(|i| Point {
            y: i32::try_from(i / width).unwrap(),
            x: i32::try_from(i % width).unwrap(),
        })
        .collect::<Vec<_>>()
}
fn find_best_asteroid(s: &str) -> (Point, usize) {
    let space = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    '.' => Space::Empty,
                    '#' => Space::Asteroid,
                    _ => panic!("Bad Entry"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let size = space.len();
    let asteroids = all_asteroids(&space);

    let max = asteroids
        .iter()
        .map(|start| {
            let (p, visible) = visible_asteroids(size, space[0].len(), *start, &asteroids);
            (p, visible.len())
        })
        .max_by_key(|(_, k)| *k)
        .unwrap();

    max
}

pub fn p1() -> IoResult<()> {
    println!(
        "Part 1 {:?} ",
        find_best_asteroid(&std::fs::read_to_string("input/day10.txt")?)
    );

    Ok(())
}

pub fn p2() -> IoResult<()> {
    let input = std::fs::read_to_string("input/day10.txt")?;
    let (pt, _count) = find_best_asteroid(&input);
    let order = destroy_asteroids(pt, &input);

    println!(
        "Part 2 {:?} ",
        order.iter().skip(199).take(1).next().unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            find_best_asteroid(&concat!(
                ".#..#\n", ".....\n", "#####\n", "....#\n", "...##\n"
            )),
            (Point { x: 3, y: 4 }, 8)
        );

        assert_eq!(
            find_best_asteroid(
                &"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ),
            (Point { x: 5, y: 8 }, 33)
        );

        assert_eq!(
            find_best_asteroid(
                &"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            ),
            (Point { x: 1, y: 2 }, 35)
        );

        assert_eq!(
            find_best_asteroid(
                &".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            ),
            (Point { x: 6, y: 3 }, 41)
        );

        let input = ".#..##.###...#######
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
###.##.####.##.#..##";
        assert_eq!(find_best_asteroid(&input,), (Point { x: 11, y: 13 }, 210));

        //         let input =
        // ".#....#####...#..
        // ##...##.#####..##
        // ##...#...#.#####.
        // ..#.....#...###..
        // ..#.#.....#....##";

        let (pt, _count) = find_best_asteroid(&input);
        let order = destroy_asteroids(pt, &input);

        let order = order.iter().map(|pt| (pt.x, pt.y)).collect::<Vec<_>>();
        assert_eq!(order[0], (11, 12));
        assert_eq!(order[1], (12, 1));
        assert_eq!(order[2], (12, 2));
        assert_eq!(order[9], (12, 8));
        assert_eq!(order[19], (16, 0));
        assert_eq!(order[49], (16, 9));
        assert_eq!(order[99], (10, 16));
        assert_eq!(order[198], (9, 6));
        assert_eq!(order[199], (8, 2));
        assert_eq!(order[200], (10, 9));
        assert_eq!(order[298], (11, 1));
    }
}
