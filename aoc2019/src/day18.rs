use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::io::Result as IoResult;

#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Copy, Clone)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
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
    #[allow(dead_code)]
    fn rotate(&self, left: bool) -> Dir {
        let dirs = &[Dir::North, Dir::West, Dir::South, Dir::East];
        let cur = dirs.iter().position(|x| *x == *self).unwrap();
        let next = if left { cur + 1 } else { dirs.len() + cur - 1 } % dirs.len();
        dirs[next].clone()
    }
}

fn point_dir(p: &Point2<i32>, d: &Dir) -> Point2<i32> {
    let mut p = p.clone();
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

#[allow(dead_code)]
fn draw_dist(grid: &HashMap<Point2<i32>, u32>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y));
            print!("{}", if p.is_some() { "X" } else { " " },)
        }
        println!();
    }
}

#[allow(unused)]
fn dijkstra<T, C>(
    start: &Point2<i32>,
    grid: &HashMap<Point2<i32>, T>,
    traversable: C,
) -> HashMap<Point2<i32>, u32>
where
    C: Fn(T) -> bool,
    T: Copy,
{
    let mut q = VecDeque::new();
    let mut lookup: HashMap<Point2<_>, usize> = grid
        .iter()
        .enumerate()
        .map(|(i, (p, _))| (*p, i))
        .collect::<HashMap<_, _>>();
    let mut dist = vec![std::u32::MAX; grid.len()];

    q.push_front((*lookup.get(start).unwrap(), *start));
    dist[*lookup.get(start).unwrap()] = 0;
    let mut idx = 0;
    while !q.is_empty() {
        let (q_idx, _) = q
            .iter()
            .enumerate()
            .min_by_key(|(_i, (idx, p))| dist[*idx])
            .unwrap();
        if let Some((dist_idx, pt)) = q.remove(q_idx) {
            //            println!("{:?} {:?}", dist_idx, pt);
            let min_dist = dist[dist_idx];
            let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
            for d in dirs.iter() {
                let search = point_dir(&pt, &d);
                grid.get(&search).map(|x| {
                    if !traversable(*x) {
                        /* do nothing */
                    } else if let Some(idx) = lookup.get(&search) {
                        let next = &mut dist[*idx];
                        if *next > min_dist.saturating_add(1) {
                            q.push_back((*idx, search));
                        }
                        *next = std::cmp::min(*next, min_dist.saturating_add(1));
                    }
                });
            }
        };
    }

    lookup
        .iter()
        .map(|(p, x)| (*p, dist[*x]))
        .filter(|(p, x)| *x != std::u32::MAX)
        .collect()
}

fn min_dist(grid: &HashMap<Point2<i32>, char>) -> u32 {
    fn keys(grid: &HashMap<Point2<i32>, char>) -> impl Iterator<Item = (&Point2<i32>, &char)> {
        grid.iter().filter(|(_p, x)| ('a'..='z').contains(*x))
    }
    fn doors(grid: &HashMap<Point2<i32>, char>) -> impl Iterator<Item = (&Point2<i32>, &char)> {
        grid.iter().filter(|(_p, x)| ('A'..='Z').contains(*x))
    }

    fn starts(grid: &HashMap<Point2<i32>, char>) -> Vec<Point2<i32>> {
        grid.iter()
            .filter(|(_p, x)| **x == '@')
            .map(|(p, _x)| *p)
            .collect()
    }

    fn traversable(x: char, keys: &HashSet<char>) -> bool {
        match x {
            '#' => false,
            'a'..='z' | '.' | '@' => true,
            _ => keys.contains(&x.to_ascii_lowercase()),
        }
    }
    #[allow(dead_code)]
    fn all_traversable(x: char, _keys: &HashSet<char>) -> bool {
        match x {
            '#' => false,
            'a'..='z' | '.' | '@' => true,
            _ => true,
        }
    }

    let key_count = keys(&grid).count();
    let key_pos = keys(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>();
    let _door_pos = doors(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>();
    let key_lookup: HashMap<char, Point2<_>> = keys(&grid).map(|(p, v)| (*v, *p)).collect();

    println!(
        "Count {} All Keys {:?}",
        key_count,
        keys(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>()
    );

    let mut worklist = vec![(starts(&grid), Vec::<char>::new(), HashSet::new(), 0)];

    let mut min_dist = std::u32::MAX;
    let mut seen: HashMap<(Vec<Point2<_>>, Vec<char>), u32> = HashMap::new();

    let mut reachable: HashMap<Vec<char>, Vec<char>> = HashMap::new();
    let mut dist: HashMap<(Vec<Point2<_>>, Vec<char>), HashMap<char, (usize, u32)>> =
        HashMap::new();

    while let Some((starts, path, keys_collected, total_steps)) = worklist.pop() {
        if keys_collected.len() == key_count {
            println!(
                "Path {:?} Dist {} Remain {}",
                path,
                total_steps,
                worklist.len()
            );
            min_dist = std::cmp::min(min_dist, total_steps);
            continue;
        }
        let mut sorted_keys: Vec<_> = keys_collected.iter().cloned().collect();
        sorted_keys.sort();

        let reach_keys = match reachable.get(&sorted_keys) {
            None => {
                let mut reach_keys = Vec::new();
                for s in starts.iter() {
                    let dijk = dijkstra(&s, &grid, |c| traversable(c, &keys_collected));
                    reach_keys.extend(
                        key_pos
                            .iter()
                            .filter(|(p, c)| grid.get(p).unwrap() == c)
                            .map(|(p, c)| (dijk.get(p), *p, *c))
                            .filter(|(d, _, _)| d.is_some())
                            .map(|(_d, _p, c)| c),
                    );
                }
                reachable.insert(sorted_keys.clone(), reach_keys);
                reachable.get(&sorted_keys).unwrap()
            }
            Some(x) => x,
        };
        let dists = match dist.get(&(starts.clone(), sorted_keys.clone())) {
            None => {
                let mut key_dists = HashMap::new();
                for (i, s) in starts.iter().enumerate() {
                    let dijk = dijkstra(&s, &grid, |c| traversable(c, &keys_collected));
                    key_dists.extend(
                        key_pos
                            .iter()
                            .filter(|(p, c)| grid.get(p).unwrap() == c)
                            .map(|(p, c)| (*c, dijk.get(p)))
                            .flat_map(|(c, d)| d.map(|d| (c, (i, *d)))),
                    );
                }
                //println!("Start: {:?} Dist {:?}", s, key_dists);
                dist.insert((starts.clone(), sorted_keys.clone()), key_dists);
                dist.get(&(starts.clone(), sorted_keys.clone())).unwrap()
            }
            Some(x) => x,
        };
        let mut reach_keys = reach_keys
            .iter()
            .map(|x| {
                let (i, d) = dists.get(x).unwrap();
                (i, d, key_lookup.get(x).unwrap(), x)
            })
            .filter(|(_, _, _, c)| !keys_collected.contains(c))
            .collect::<Vec<_>>();
        reach_keys.sort_by_key(|(_, d, _, _)| *d);
        reach_keys.reverse();

        //println!("Reach Keys {:?}", reach_keys);

        for (i, d, p, c) in reach_keys {
            let mut new_keys = keys_collected.clone();
            new_keys.insert(*c);
            let mut path = path.clone();
            path.extend(&[*c]);
            let mut starts = starts.clone();
            starts[*i] = *p;
            let new_steps = total_steps + d;
            {
                let mut new_keys = new_keys.iter().cloned().collect::<Vec<_>>();
                new_keys.sort();
                let lookup = (starts.clone(), new_keys);
                if new_steps >= min_dist
                    || *seen.get(&lookup).unwrap_or(&std::u32::MAX) <= new_steps
                {
                    continue;
                }
                seen.insert(lookup, new_steps);
            }
            worklist.push((starts, path, new_keys, new_steps))
        }
    }
    min_dist
}

pub fn p1() -> IoResult<()> {
    let _s = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
        .to_string();

    let _s = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
        .to_string();

    let _s = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
        .to_string();
    let _s = "#########
#b.A.@.a#
#########"
        .to_string();

    let _s = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
        .to_string();
    let s = std::fs::read_to_string("input/day18.txt")?;
    let grid = s
        .split('\n')
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    println!("{:?}", min_dist(&grid));
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let _s = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############"
        .to_string();

    let _s = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############"
        .to_string();

    let _s = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"
        .to_string();

    let s = std::fs::read_to_string("input/day18_2.txt")?;
    let grid = s
        .split('\n')
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<_, char>>();

    println!("{:?}", min_dist(&grid));
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
