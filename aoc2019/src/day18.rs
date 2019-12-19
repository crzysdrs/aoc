use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};
use cgmath::{Point2,Vector2};
use std::collections::{HashMap,HashSet};
use num_traits::{ToPrimitive, FromPrimitive};
use num_derive::{ToPrimitive, FromPrimitive};
use std::collections::VecDeque;

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
            print!(
                "{}",
                p.unwrap_or(&'?')
            )
        }
        println!();
    }
}

fn draw_dist(grid: &HashMap<Point2<i32>, u32>) {
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    for y in (min_y..=max_y) {
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y));
            print!(
                "{}",
                if p.is_some() {"X"} else {" "},
            )
        }
        println!();
    }
}

#[allow(unused)]
fn dijkstra<T, C>(start: &Point2<i32>, grid: &HashMap<Point2<i32>, T>, traversable: C) -> HashMap<Point2<i32>, u32>
where C:  Fn(T) -> bool,
    T: Copy
{
    let mut q: VecDeque<_> = grid.iter().filter(|(p, x)| traversable(**x))
        .enumerate().map(|(i, (p, x))| (i, p, x)).collect();
    let lookup : HashMap<Point2<_>, usize> = q.iter().map(|(i, p, x)| (**p, *i)).collect::<HashMap<_,_>>();
    let mut dist = vec![std::u32::MAX; q.len()];

    dist[*lookup.get(start).unwrap()] = 0;
    while !q.is_empty() {
        let (q_idx, (dist_idx, &pt, _x)) = q
            .iter()
            .enumerate()
            .min_by_key(|(_i, (idx, p, x))| dist[*idx])
            .unwrap();
        let min_dist = dist[*dist_idx];
        let _v = q.remove(q_idx);
        let dirs = [Dir::North, Dir::South, Dir::East, Dir::West];
        for d in dirs.iter() {
            let search = point_dir(&pt, &d);
            if let Some(idx) = lookup.get(&search) {
                let next = &mut dist[*idx];
                *next = std::cmp::min(*next, min_dist.saturating_add(1));
            }
        }
    }

    lookup.iter().map(|(p, x)| (*p, dist[*x])).filter(|(p, x)| *x != std::u32::MAX).collect()
}

pub fn p1() -> IoResult<()> {
    let s = "#########
#b.A.@.a#
#########".to_string();

    let s = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################".to_string();

    let s = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################".to_string();

    let s = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################".to_string();

    let s = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################".to_string();
    
    let s = std::fs::read_to_string("input/day18.txt")?;
    let grid = s        
        .split("\n")
        .enumerate()
        .flat_map(
            move |(y, l)|
            l.chars().enumerate().map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        )
        .collect::<HashMap<_,char>>();

    fn keys(grid : &HashMap<Point2<i32>, char>) -> impl Iterator<Item=(&Point2<i32>, &char)> {
        grid.iter().filter(|(p, x)| ('a'..='z').contains(*x))
    }
    fn doors(grid : &HashMap<Point2<i32>, char>) -> impl Iterator<Item=(&Point2<i32>, &char)> {
        grid.iter().filter(|(p, x)| ('a'..='z').contains(*x))
    }

    fn start(grid : &HashMap<Point2<i32>, char>) -> Point2<i32> {
        grid.iter().find(|(p, x)| **x == '@').map(|(p, x)| *p).unwrap()
    }
                        
    fn traversable(x : char, keys: &[char]) -> bool {
        match x {
            '#' => false,
            'a'..='z' |'.'| '@' =>  true,
            _ => false,
            //_ =>  keys.contains(&x.to_ascii_lowercase())
        }        
    }

    let key_count = keys(&grid).count();
    let key_pos = keys(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>();
    
    println!("Count {} All Keys {:?}", key_count, keys(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>());
             
    let mut worklist = vec![(grid, HashSet::new(), 0)];

    let mut min_dist = std::u32::MAX;
    let mut seen : HashMap<(Point2<_>, Vec<char>), u32> = HashMap::new();
    
    while let Some((grid, keys_collected, total_steps)) = worklist.pop() {
   
        let s = start(&grid);
        let dist = dijkstra(&s, &grid, |c| traversable(c, &[]));

        //println!("Start: {:?} Dist {:?}", s, dist.len());
        let mut reach_keys = key_pos.iter().filter(|(p, c)| grid.get(p).unwrap() == c)
            .map(|(p, c)| (dist.get(p), *p, *c)).filter(|(d, _, _)| d.is_some()).collect::<Vec<_>>();
        reach_keys.sort_by_key(|(d, _, _)| *d.unwrap());
        reach_keys.reverse();
        
        if keys_collected.len() == key_count - 1  && reach_keys.len() > 0 {
            let (d, _, _) = reach_keys[0];
            //println!("Found Key {}", c);
            min_dist = std::cmp::min(min_dist, total_steps + d.unwrap());
            println!("Dist {} Remain {}", min_dist, worklist.len());
        } else if reach_keys.len() == 0 {
            println!("Couldn't get all keys {:?}", keys_collected);
            draw(&grid);
            draw_dist(&dist);
        } else {
            for (d, p, c) in reach_keys {
                let mut sorted_keys :Vec<_> = keys_collected.iter().cloned().collect();
                sorted_keys.sort();
                let lookup = (p.clone(), sorted_keys);
                //println!("Lookup {:?}", lookup);
                if total_steps + d.unwrap() >= min_dist || *seen.get(&lookup).unwrap_or(&std::u32::MAX) <= total_steps + d.unwrap() {
                    continue;
                }
                seen.insert(lookup, total_steps + d.unwrap());
                //println!("Seen {}", seen.len());
                //println!("{}", c);
                //draw(&grid);
                let door = grid.iter().find(|(_, x)| **x == c.to_ascii_uppercase()).map(|(p, _)| *p);                
                let mut grid = grid.clone();
                if let Some(d) = door {
                    *grid.entry(d).or_insert('.') = '.';
                }
                *grid.entry(s).or_insert('.') = '.';
                *grid.entry(p).or_insert('@') = '@';
                let mut new_keys = keys_collected.clone();
                new_keys.insert(c);
                worklist.push((grid.clone(), new_keys, total_steps + d.unwrap()))
            }
        }
    }
    println!("{:?}", min_dist);
    Ok(())
}



pub fn p2() -> IoResult<()> {
    let s = "#########
#b.A.@.a#
#########".to_string();

    let s = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################".to_string();

    let s = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################".to_string();

    let s = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################".to_string();

    let s = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################".to_string();
    
    let s = std::fs::read_to_string("input/day18_2.txt")?;
    let grid = s        
        .split("\n")
        .enumerate()
        .flat_map(
            move |(y, l)|
            l.chars().enumerate().map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
        )
        .collect::<HashMap<_,char>>();

    fn keys(grid : &HashMap<Point2<i32>, char>) -> impl Iterator<Item=(&Point2<i32>, &char)> {
        grid.iter().filter(|(p, x)| ('a'..='z').contains(*x))
    }
    fn doors(grid : &HashMap<Point2<i32>, char>) -> impl Iterator<Item=(&Point2<i32>, &char)> {
        grid.iter().filter(|(p, x)| ('a'..='z').contains(*x))
    }

    fn start(grid : &HashMap<Point2<i32>, char>) -> impl Iterator<Item=&Point2<i32>> {
        grid.iter().filter(|(p, x)| **x == '@').map(|(p, x)| p)
    }
                        
    fn traversable(x : char, keys: &[char]) -> bool {
        match x {
            '#' => false,
            'a'..='z' |'.'| '@' =>  true,
            _ => false,
            //_ =>  keys.contains(&x.to_ascii_lowercase())
        }        
    }

    fn all_traversable(x : char, keys: &[char]) -> bool {
        match x {
            '#' => false,
            _ => true,
        }
    }

    let key_count = keys(&grid).count();
    let key_pos = keys(&grid).map(|(p, v)| (*v, *p)).collect::<HashMap<_,_>>();
    
    println!("Count {} All Keys {:?}", key_count, keys(&grid).map(|(p, v)| (*p, *v)).collect::<Vec<_>>());

    let orig_grid = grid.clone();
    let mut worklist = vec![(grid, HashSet::new(), 0)];

    let mut min_dist = std::u32::MAX;
    //let mut seen : HashMap<(Point2<_>, Vec<char>), u32> = HashMap::new();

    let mut keys_available : HashMap<Vec<char>, HashSet<char>> = HashMap::new();
    let mut key_dists : HashMap<Point2<_>, HashMap<char, u32>> = HashMap::new();
                                     
    while let Some((grid, keys_collected, total_steps)) = worklist.pop() {
        if total_steps >= min_dist {
            continue;
        }
        let starts = start(&grid).collect::<Vec<_>>();
        let mut reach_keys : Vec<(u32, Point2<_>, char,  Point2<_>)>= vec![];       
        //println!("Start: {:?} Dist {:?}", s, dist.len());
        let mut sort_keys_collected :Vec<char>= keys_collected.iter().cloned().collect();
        sort_keys_collected.sort();
        if keys_available.get(&sort_keys_collected).is_none() {
            let mut new_keys = HashSet::new();
            for s in &starts {     
                let dist = dijkstra(&s, &grid, |c| traversable(c, &[]));
                let reach_keys_once = key_pos.iter().filter(|(c, p)| grid.get(p).unwrap() == *c).map(|(c, p)| *c).collect::<HashSet<_>>();
                new_keys.extend(reach_keys_once);
            }
            //println!("{} {:?} {:?}", keys_available.len(), sort_keys_collected, new_keys);
            keys_available.insert(sort_keys_collected.clone(), new_keys);
        }
        let available =  keys_available.get(&sort_keys_collected).unwrap();
        for s in &starts {                                    
            if key_dists.get(&s).is_none() {
                let dist = dijkstra(&s, &orig_grid, |c| all_traversable(c, &[]));
                let new_key_dists = key_pos.iter()
                    .filter(|(c, p)| dist.get(p).is_some())
                    .map(|(c, p)| (*c, *dist.get(p).expect("Dist in Result"))).collect::<HashMap<_,_>>();
                //println!("{:?} {:?}", *s, new_key_dists);
                key_dists.insert(**s, new_key_dists);
            }
            let dists = key_dists.get(&s).unwrap();

            reach_keys.extend(available.iter().flat_map(|c| dists.get(c).map(|x| (c, x))).map(|(c, dist)| (*dist, *key_pos.get(c).unwrap(), *c, **s)));
        }


        //println!("{:?} Reach Keys", reach_keys);
        
        if keys_collected.len() == key_count - 1  && reach_keys.len() > 0 {
            let (d, _, _, _) = reach_keys[0];
            //println!("Found Key {}", c);
            min_dist = std::cmp::min(min_dist, total_steps + d);
            println!("Dist {} Remain {}", min_dist, worklist.len());
        } else if reach_keys.len() == 0 {
            println!("Couldn't get all keys {:?}", keys_collected);
            draw(&grid);
            //draw_dist(&dist);
        } else {
            for (d, p, c, s) in reach_keys {
                // let mut sorted_keys :Vec<_> = keys_collected.iter().cloned().collect();
                // sorted_keys.sort();
                // let lookup = (p.clone(), sorted_keys);
                // //println!("Lookup {:?}", lookup);
                // if total_steps + d >= min_dist || *seen.get(&lookup).unwrap_or(&std::u32::MAX) <= total_steps + d {
                //     continue;
                // }
                // seen.insert(lookup, total_steps + d);
                //println!("Seen {}", seen.len());
                //println!("{}", c);
                //draw(&grid);
                let door = grid.iter().find(|(_, x)| **x == c.to_ascii_uppercase()).map(|(p, _)| *p);                
                let mut grid = grid.clone();
                if let Some(d) = door {
                    *grid.entry(d).or_insert('.') = '.';
                }
                *grid.entry(s).or_insert('.') = '.';
                *grid.entry(p).or_insert('@') = '@';
                let mut new_keys = keys_collected.clone();
                new_keys.insert(c);
                worklist.push((grid.clone(), new_keys, total_steps + d))
            }
        }
    }
    println!("{:?}", min_dist);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(false);
    }
}
