use crate::Day;
use cgmath::{Point2, Vector2};
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Clone)]
pub struct Tile {
    id: usize,
    tile: Vec<Vec<bool>>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Side {
    Bottom,
    Top,
    Left,
    Right,
}

impl Side {
    fn sides() -> Vec<Side> {
        vec![Side::Top, Side::Right, Side::Bottom, Side::Left]
    }
    fn offset(&self, p: &Point2<i32>) -> Point2<i32> {
        match self {
            Side::Bottom => p + Vector2::new(0, 1),
            Side::Top => p + Vector2::new(0, -1),
            Side::Right => p + Vector2::new(1, 0),
            Side::Left => p + Vector2::new(-1, 0),
        }
    }
    fn opposite(&self) -> Side {
        match self {
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Top => Side::Bottom,
        }
    }
}

#[allow(unused)]
fn print(grid: &[Vec<bool>]) {
    grid.iter().for_each(|r| {
        println!(
            "{}",
            r.iter()
                .map(|x| if *x { '#' } else { '.' })
                .collect::<String>()
        )
    });
}

impl Tile {
    fn top(&self) -> Vec<bool> {
        self.tile[0].clone()
    }
    fn bottom(&self) -> Vec<bool> {
        self.tile.iter().last().unwrap().clone()
    }
    fn left(&self) -> Vec<bool> {
        self.tile.iter().map(|v| v[0]).collect()
    }
    fn right(&self) -> Vec<bool> {
        self.tile
            .iter()
            .map(|v| *v.iter().last().unwrap())
            .collect()
    }
    fn side(&self, s: Side) -> Vec<bool> {
        match s {
            Side::Top => self.top(),
            Side::Right => self.right(),
            Side::Left => self.left(),
            Side::Bottom => self.bottom(),
        }
    }
    fn borders(&self) -> Vec<(Side, Vec<bool>)> {
        vec![
            (Side::Top, self.top()),
            (Side::Right, self.right()),
            (Side::Bottom, self.bottom()),
            (Side::Left, self.left()),
        ]
    }

    fn border_id(&self, s: Side) -> usize {
        let x = self.side(s);
        let bin = |mut state: usize, x: &bool| -> usize {
            state <<= 1;
            if *x {
                state |= 1;
            }
            state
        };
        let a = x.iter().fold(0, bin);
        let b = x.iter().rev().fold(0, bin);
        std::cmp::min(a, b)
    }
    fn border_ids(&self) -> Vec<(Side, usize)> {
        self.borders()
            .into_iter()
            .map(|(s, _)| (s, self.border_id(s)))
            .collect()
    }
    fn flipx(&mut self) {
        self.tile.iter_mut().for_each(|r| r.reverse());
    }
    fn flipy(&mut self) {
        self.tile.reverse();
    }
    fn rot_right(&mut self) {
        let mut new = vec![vec![false; self.tile.len()]; self.tile.len()];
        for (x, item) in new.iter_mut().enumerate().take(self.tile.len()) {
            for y in 0..self.tile.len() {
                item[self.tile.len() - 1 - y] = self.tile[y][x];
            }
        }
        self.tile = new;
    }
    fn strip(&mut self) {
        let len = self.tile.len();
        self.tile = self
            .tile
            .iter()
            .skip(1)
            .take(len - 2)
            .map(|r| r.iter().skip(1).take(len - 2).cloned().collect())
            .collect();
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 20;
    type Input = Tile;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let tile_name = Regex::new("Tile ([0-9]+):").unwrap();
        let mut lines = r.lines();

        let mut tiles = vec![];
        loop {
            let mut tile = lines.by_ref().flatten().take_while(|x| !x.is_empty());

            if let Some(name) = tile.next() {
                let m = tile_name.captures(&name).unwrap();
                let num = m.get(1).unwrap().as_str().parse().unwrap();
                let tile = tile
                    .map(|x| x.chars().map(|x| x == '#').collect())
                    .collect();
                tiles.push(Tile { id: num, tile })
            } else {
                break Ok(tiles);
            }
        }
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let borders =
            v.iter()
                .flat_map(|t| t.borders())
                .fold(HashMap::new(), |mut state, (_, b)| {
                    let mut rev = b.clone();
                    rev.reverse();
                    if let Some(v) = state.get_mut(&b) {
                        *v += 1;
                    } else if let Some(v) = state.get_mut(&rev) {
                        *v += 1;
                    } else {
                        state.insert(b, 1);
                    }
                    state
                });

        v.iter()
            .filter(|t| {
                let mut unique_borders = 0;
                for (_, b) in t.borders() {
                    let mut rev = b.clone();
                    rev.reverse();
                    if let Some(v) = borders.get(&b) {
                        if *v == 1 {
                            unique_borders += 1;
                        }
                    } else if let Some(v) = borders.get(&rev) {
                        if *v == 1 {
                            unique_borders += 1;
                        }
                    } else {
                        panic!()
                    }
                }
                unique_borders == 2
            })
            .map(|t| t.id)
            .product()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let border_ids = v
            .iter()
            .flat_map(|t| {
                let id = t.id;
                t.border_ids().into_iter().map(move |(_s, bid)| (bid, id))
            })
            .fold(
                HashMap::<usize, Vec<usize>>::new(),
                |mut state, (bid, id)| {
                    state
                        .entry(bid)
                        .and_modify(|v| v.push(id))
                        .or_insert_with(|| vec![id]);
                    state
                },
            );

        #[derive(Copy, Clone, Debug)]
        enum SideMatch {
            Known(usize),
            Unknown,
        }

        let mut grid: HashMap<Point2<i32>, Tile> = HashMap::new();
        let mut remaining_tiles = v.to_vec();

        let mut count = 0i32;
        let grid_size = loop {
            count += 1;
            if remaining_tiles.len() as i32 == count.pow(2) {
                break count;
            }
        };
        //println!("{:?}", border_ids);
        for y in 0..grid_size {
            for x in 0..grid_size {
                let pos = Point2::new(x, y);
                let need = Side::sides()
                    .into_iter()
                    .map(|s| (s, s.offset(&pos)))
                    .map(|(s, p)| {
                        if p.x < 0 || p.x >= grid_size || p.y < 0 || p.y >= grid_size {
                            (p, s, None)
                        } else if let Some(t) = grid.get(&p) {
                            (p, s, Some(SideMatch::Known(t.border_id(s.opposite()))))
                        } else {
                            (p, s, Some(SideMatch::Unknown))
                        }
                    })
                    .collect::<Vec<_>>();

                //println!("{:?}", need);
                let tile_pos = remaining_tiles
                    .iter()
                    .position(|t| {
                        //println!("{:?}", t.border_ids());
                        let knowns = need
                            .iter()
                            .map(|(_, _, id)| match id {
                                None => true,
                                Some(SideMatch::Known(id)) => {
                                    t.border_ids().iter().any(|(_, bid)| id == bid)
                                }
                                Some(SideMatch::Unknown) => true,
                            })
                            .all(|x| x);
                        let unknowns_need_count =
                            need.iter().filter(|(_, _, id)| id.is_none()).count();
                        let unknowns_count = t
                            .border_ids()
                            .iter()
                            .filter(|(_s, id)| {
                                border_ids.get(id).map(|v| v.len()).unwrap_or(0) == 1
                            })
                            .count();
                        //println!("{} {}", unknowns_need_count, unknowns_count);
                        let unknowns = unknowns_count == unknowns_need_count;
                        //println!("{} {}", knowns, unknowns);
                        knowns && unknowns
                    })
                    .unwrap();

                let (target_side, target_need) = need.iter().fold(
                    (Side::Top, Some(SideMatch::Unknown)),
                    |state, (_, side, id)| match (state.1, id) {
                        (_, Some(SideMatch::Known(id))) => (*side, Some(SideMatch::Known(*id))),
                        (Some(SideMatch::Unknown), _) => (*side, *id),
                        (_s, _) => state,
                    },
                );

                let mut tile = remaining_tiles.remove(tile_pos);

                //println!("Target: {} {:?} {:?}", tile.id, target_side, target_need);
                //println!("# Tiles: {}", grid.len());
                //println!("Need: {:?}", need);
                while match target_need {
                    Some(SideMatch::Known(id)) => tile.border_id(target_side) != id,
                    Some(SideMatch::Unknown) => unreachable!(),
                    None => {
                        border_ids
                            .get(&tile.border_id(target_side))
                            .map(|v| v.len())
                            .unwrap_or(0)
                            != 1
                    }
                } {
                    tile.rot_right();
                }

                //println!("{:?}", tile.border_ids().into_iter().map(|(s, bid)| (s, bid, border_ids.get(&bid))).collect::<Vec<_>>());
                for (p, s, typ) in &need {
                    let (flip_tile, flip_side) = match typ {
                        Some(SideMatch::Known(id)) => {
                            let t2 = grid.get(p).unwrap();
                            let flip_tile = tile.border_id(*s) != *id;
                            let new_side = if flip_tile { s.opposite() } else { *s };
                            (flip_tile, t2.side(s.opposite()) != tile.side(new_side))
                        }
                        Some(SideMatch::Unknown) => (false, false),
                        None => {
                            //println!("{:?}", border_ids.get(&tile.border_id(*s)));
                            (
                                border_ids
                                    .get(&tile.border_id(*s))
                                    .map(|v| v.len())
                                    .unwrap_or(0)
                                    != 1,
                                false,
                            )
                        }
                    };
                    //println!("{:?} {:?} {} {}", s, typ, flip_tile, flip_side);
                    if flip_side {
                        match s {
                            Side::Top | Side::Bottom => tile.flipx(),
                            Side::Left | Side::Right => tile.flipy(),
                        }
                    }
                    if flip_tile {
                        match s {
                            Side::Top | Side::Bottom => tile.flipy(),
                            Side::Left | Side::Right => tile.flipx(),
                        }
                    }
                    //println!("{:?}", tile.border_ids().into_iter().map(|(s, bid)| (s, bid, border_ids.get(&bid))).collect::<Vec<_>>());
                }

                for (p, s, typ) in &need {
                    match typ {
                        Some(SideMatch::Known(id)) => {
                            let t2 = grid.get(p).unwrap();
                            assert_eq!(t2.border_id(s.opposite()), *id);
                            assert_eq!(t2.side(s.opposite()), tile.side(*s));
                        }
                        Some(SideMatch::Unknown) => {}
                        None => {
                            assert_eq!(
                                border_ids
                                    .get(&tile.border_id(*s))
                                    .map(|v| v.len())
                                    .unwrap_or(0),
                                1
                            )
                        }
                    }
                }
                //println!("Placed Tile: {:?} {} {:?}", pos, tile.id, tile.border_ids().into_iter().map(|(s, bid)| (s, bid, border_ids.get(&bid))).collect::<Vec<_>>());
                grid.insert(pos, tile);
            }
        }

        grid.values_mut().for_each(|t| t.strip());

        let mut big_tile = vec![];
        for y in 0..grid_size {
            let mut row = grid.iter().filter(|(p, _)| p.y == y).collect::<Vec<_>>();
            row.sort_by_key(|(p, _)| p.x);
            let tile_size = row[0].1.tile.len();
            for i in 0..tile_size {
                let mut new_row = vec![];
                for r in &row {
                    new_row.extend(r.1.tile[i].clone());
                }
                big_tile.push(new_row);
            }
        }
        let mut big_tile = Tile {
            id: 0,
            tile: big_tile,
        };
        let monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
        let monster = monster
            .split('\n')
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for _ in 0..2 {
            for _ in 0..4 {
                let search = big_tile.clone();
                //print(&big_tile.tile);
                let monster = &monster;
                let found = search
                    .tile
                    .windows(monster.len())
                    .enumerate()
                    .flat_map(|(start, win)| {
                        let mut mons = vec![];
                        for i in 0..(win[0].len() - monster[0].len()) {
                            if (0..win.len())
                                .map(|j| {
                                    win[j][i..].iter().zip(&monster[j]).all(|(sea, mon)| {
                                        if *mon {
                                            *sea
                                        } else {
                                            true
                                        }
                                    })
                                })
                                //.inspect(|x| println!("{:?}", x))
                                .all(|x| x)
                            {
                                mons.push((start, i))
                            }
                        }
                        mons.into_iter()
                    })
                    .collect::<Vec<_>>();

                println!("Monsters found {:?}", found);
                if !found.is_empty() {
                    for (start, i) in found {
                        for (j, m) in monster.iter().enumerate() {
                            big_tile.tile[start + j][i..]
                                .iter_mut()
                                .zip(m.iter())
                                .for_each(|(sea, mon)| {
                                    if *mon {
                                        *sea = false
                                    }
                                })
                        }
                    }
                    return big_tile.tile.iter().flatten().filter(|x| **x).count();
                }
                big_tile.rot_right();
            }
            big_tile.flipx();
        }
        panic!("NO solution")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 20899048083289);

        assert_eq!(Solution::p2(&v), 273);
    }
}
