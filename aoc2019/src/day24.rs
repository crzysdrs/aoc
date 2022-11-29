use cgmath::Point2;
use std::collections::HashSet;
use std::io::Result as IoResult;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Bug,
    Empty,
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
    tiles
        .iter()
        .enumerate()
        .map(|(i, t)| match t {
            Tile::Bug => 1 << i,
            Tile::Empty => 0,
        })
        .sum()
}

pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day24.txt")?;
    //     let s = "....#
    // #..#.
    // #..##
    // ..#..
    // #....";
    let mut tiles = s
        .trim()
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '.' => Tile::Empty,
            '#' => Tile::Bug,
            c => panic!("Unhandled char {}", c),
        })
        .collect::<Vec<_>>();

    let width = 5;
    let mut next = vec![Tile::Empty; tiles.len()];

    let mut gen_tiles = HashSet::<Vec<Tile>>::new();
    'done: for g in 0.. {
        println!("Gen {}", g);
        draw(width, &tiles);
        for y in 0..width {
            for x in 0..width {
                let mut count = 0;
                let target_idx = width * y + x;
                for (i, j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let y = y as isize;
                    let x = x as isize;
                    let width = width as isize;
                    if y + i >= 0 && y + i < width && x + j >= 0 && x + j < width {
                        let idx = width * (y + i) + x + j;
                        match tiles[idx as usize] {
                            Tile::Bug => {
                                count += 1;
                            }
                            Tile::Empty => {}
                        }
                    }
                }
                next[target_idx] = match tiles[target_idx] {
                    Tile::Bug if count != 1 => Tile::Empty,
                    Tile::Empty if count == 1 || count == 2 => Tile::Bug,
                    a => a,
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

fn adj(item: Item) -> impl Iterator<Item = Item> {
    let x = item.pt.x;
    let y = item.pt.y;
    let depth = item.depth;
    let mut adjs = vec![];

    for (i, j) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let y_offset = y as isize + i;
        let x_offset = x as isize + j;
        let width = 5;
        if y_offset == 2 && x_offset == 2 {
            match (j, i) {
                (0, 1) => {
                    adjs.extend((0..5).map(|x| Item {
                        depth: depth - 1,
                        pt: Point2 { x, y: 0 },
                    }));
                }
                (1, 0) => {
                    adjs.extend((0..5).map(|x| Item {
                        depth: depth - 1,
                        pt: Point2 { x: 0, y: x },
                    }));
                }
                (-1, 0) => {
                    adjs.extend((0..5).map(|x| Item {
                        depth: depth - 1,
                        pt: Point2 { x: 4, y: x },
                    }));
                }
                (0, -1) => {
                    adjs.extend((0..5).map(|x| Item {
                        depth: depth - 1,
                        pt: Point2 { x, y: 4 },
                    }));
                }
                _ => panic!("Invalid Vector"),
            }
        } else {
            let mut y_on_grid = false;
            let mut x_on_grid = false;
            if y_offset < 0 {
                //top edge
                adjs.push(Item {
                    depth: depth + 1,
                    pt: Point2 { x: 2, y: 1 },
                });
            } else if y_offset >= width {
                //bottom edge
                adjs.push(Item {
                    depth: depth + 1,
                    pt: Point2 { x: 2, y: 3 },
                });
            } else {
                y_on_grid = true;
            }

            if x_offset < 0 {
                //left edge
                adjs.push(Item {
                    depth: depth + 1,
                    pt: Point2 { x: 1, y: 2 },
                });
            } else if x_offset >= width {
                //right edge
                adjs.push(Item {
                    depth: depth + 1,
                    pt: Point2 { x: 3, y: 2 },
                });
            } else {
                x_on_grid = true;
            }

            if x_on_grid && y_on_grid {
                adjs.push(Item {
                    depth,
                    pt: Point2 {
                        x: x_offset as u32,
                        y: y_offset as u32,
                    },
                });
            }
        }
    }
    adjs.into_iter()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Item {
    depth: i32,
    pt: Point2<u32>,
}

fn draw_r(items: &HashSet<Item>) {
    let mut depths = items.iter().map(|i| i.depth).collect::<Vec<_>>();
    depths.sort();
    depths.dedup();

    for d in depths {
        println!("Depth:  {}", d);
        for y in 0..5 {
            for x in 0..5 {
                if y == 2 && x == 2 {
                    print!("?");
                } else if items.contains(&Item {
                    depth: d,
                    pt: Point2 { x, y },
                }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}
pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day24.txt")?;
    //     let s = "....#
    // #..#.
    // #..##
    // ..#..
    // #....".to_string();
    let mut current: HashSet<Item> = HashSet::new();

    let tiles = s
        .trim()
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '.' => Tile::Empty,
            '#' => Tile::Bug,
            c => panic!("Unhandled char {}", c),
        })
        .collect::<Vec<_>>();

    let width = 5;
    for y in 0..width {
        for x in 0..width {
            if y == 2 && x == 2 {
                //question mark
            } else if let Tile::Bug = tiles[y * width + x] {
                current.insert(Item {
                    depth: 0,
                    pt: Point2 {
                        x: x as u32,
                        y: y as u32,
                    },
                });
            }
        }
    }

    draw_r(&current);

    for g in 0..200 {
        println!("Generation {}", g);
        let mut worklist: Vec<Item> = vec![];

        worklist.extend(current.iter());
        let mut seen: HashSet<Item> = HashSet::new();
        let mut next: HashSet<Item> = HashSet::new();
        while let Some(item) = worklist.pop() {
            if seen.contains(&item) {
                continue;
            }
            seen.insert(item);

            if current.contains(&item) {
                worklist.extend(adj(item));
            }
            let neighbors: u32 = adj(item)
                .map(|x| if current.get(&x).is_some() { 1 } else { 0 })
                .sum();
            match current.contains(&item) {
                true if neighbors != 1 => {}
                false if neighbors == 1 || neighbors == 2 => {
                    next.insert(item);
                }
                true => {
                    next.insert(item);
                }
                false => {}
            }
        }
        std::mem::swap(&mut next, &mut current);
        draw_r(&current);
    }

    println!("{}", current.iter().count());

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
