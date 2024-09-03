use crate::intcode::IntCodeMachine;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Result as IoResult;

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "north",
                Direction::South => "south",
                Direction::East => "east",
                Direction::West => "west",
            }
        )
    }
}
#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[allow(dead_code)]
    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}
#[derive(Debug)]
enum Cmd {
    Dir(Direction),
    Take(String),
    Drop(String),
    Inv,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cmd::Dir(d) => write!(f, "{}", d),
            Cmd::Take(i) => write!(f, "take {}", i),
            Cmd::Drop(i) => write!(f, "drop {}", i),
            Cmd::Inv => write!(f, "inv"),
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    dirs: Vec<Direction>,
    items: Vec<String>,
}

fn parse_room(room: String) -> Option<Room> {
    let mut dirs = false;
    let mut items = false;
    let mut r = None;
    for l in room.lines() {
        if l.len() > 2 && &l[..2] == "==" {
            r = Some(Room {
                name: l[3..l.len() - 3].to_string(),
                dirs: vec![],
                items: vec![],
            })
        } else if l == "Doors here lead:" {
            dirs = true;
        } else if l == "Items here:" {
            items = true;
        } else if l.starts_with('-') {
            if dirs {
                let d = match &l[2..] {
                    "north" => Direction::North,
                    "south" => Direction::South,
                    "east" => Direction::East,
                    "west" => Direction::West,
                    _ => panic!("Unhandled direction"),
                };
                if let Some(x) = r.as_mut() {
                    x.dirs.push(d);
                }
            } else if items {
                if let Some(x) = r.as_mut() {
                    x.items.push(l[2..].to_string());
                }
            }
        } else if l.is_empty() {
            dirs = false;
            items = false;
        }
    }
    r
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day25.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let unsafe_items = [
        "photons",
        "escape pod",
        "molten lava",
        //"hypercube",
        "infinite loop",
        "giant electromagnet",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<HashSet<String>>();
    let mut m = IntCodeMachine::new(codes, vec![]);
    let mut rooms = vec![];

    let mut dir_stack = vec![];

    m.run();
    let room = parse_room(
        m.output()
            .into_iter()
            .map(|x| x as u8 as char)
            .collect::<String>(),
    )
    .unwrap();

    use petgraph::Graph;
    let mut deps = Graph::new();
    let idx = deps.add_node(room.name.clone());
    let mut nodes: HashMap<String, _> = HashMap::new();
    nodes.insert(room.name.clone(), idx);

    for d in &room.dirs {
        let mut m = m.clone();
        format!("{}\n", Cmd::Dir(*d))
            .chars()
            .for_each(|x| m.feed_input(x as isize));
        dir_stack.push((m, *d, idx));
    }
    rooms.push(room);

    while let Some((mut m, dir, idx)) = dir_stack.pop() {
        m.run();
        let output = m
            .output()
            .into_iter()
            .map(|x| x as u8 as char)
            .collect::<String>();
        //println!("Output: {}", output);
        let new_room = parse_room(output);
        println!("{:?} {:?} {:?}", dir, idx, new_room);
        if let Some(r) = new_room {
            let (new_idx, seen) = if let Some(idx) = nodes.get(&r.name) {
                println!("Seen {}", r.name);
                (*idx, true)
            } else {
                let new_idx = deps.add_node(r.name.clone());
                nodes.insert(r.name.clone(), new_idx);
                rooms.push(r.clone());
                (new_idx, false)
            };
            deps.add_edge(idx, new_idx, dir);
            if !seen {
                for d in &r.dirs {
                    let mut m = m.clone();
                    format!("{}\n", Cmd::Dir(*d))
                        .chars()
                        .for_each(|x| m.feed_input(x as isize));
                    dir_stack.push((m, *d, new_idx));
                }
            }
        }
    }

    let items = rooms
        .iter()
        .flat_map(|r| {
            r.items
                .clone()
                .into_iter()
                .map(move |i| (r.name.clone(), i))
        })
        .filter(|(_, i)| !unsafe_items.contains(i))
        .collect::<Vec<_>>();
    let mut cur_room = "Hull Breach".to_string();
    use petgraph::algo::astar;
    for (target_room, item) in &items {
        println!("Acquire {:?} {:?}", target_room, item);
        let r = astar(
            &deps,
            *nodes.get(&cur_room).unwrap(),
            |finish| finish == *nodes.get(target_room).unwrap(),
            |_e| 1,
            |_| 0,
        );
        //println!("{:?}", rooms);
        println!("{:?}", r);
        for node in r.unwrap().1.windows(2) {
            let edge = deps
                .edge_weight(deps.find_edge(node[0], node[1]).unwrap())
                .unwrap();
            format!("{}\n", Cmd::Dir(*edge))
                .chars()
                .for_each(|x| m.feed_input(x as isize));
        }
        format!("{}\n", Cmd::Take(item.to_string()))
            .chars()
            .for_each(|x| m.feed_input(x as isize));
        m.run();
        //let output = m.output().into_iter().map(|x| x as u8 as char).collect::<String>();
        //println!("Output {}", output);
        cur_room = target_room.clone();
    }

    drop(m.output());
    let target_room = "Security Checkpoint".to_string();
    let r = astar(
        &deps,
        *nodes.get(&cur_room).unwrap(),
        |finish| finish == *nodes.get(&target_room).unwrap(),
        |_e| 1,
        |_| 0,
    );
    for node in r.unwrap().1.windows(2) {
        let edge = deps
            .edge_weight(deps.find_edge(node[0], node[1]).unwrap())
            .unwrap();
        format!("{}\n", Cmd::Dir(*edge))
            .chars()
            .for_each(|x| m.feed_input(x as isize));
    }
    format!("{}\n", Cmd::Inv)
        .chars()
        .for_each(|x| m.feed_input(x as isize));

    let item_count = items.len();
    for i in 1..item_count {
        for combo in items.iter().combinations(i) {
            let mut m = m.clone();
            combo.iter().for_each(|combo_item| {
                format!("{}\n", Cmd::Drop(combo_item.1.to_string()))
                    .chars()
                    .for_each(|x| m.feed_input(x as isize))
            });
            m.run();
            drop(m.output());
            format!("{}\n", Cmd::Dir(Direction::South))
                .chars()
                .for_each(|x| m.feed_input(x as isize));
            m.run();
            let output = m
                .output()
                .into_iter()
                .map(|x| x as u8 as char)
                .collect::<String>();
            if let Some(r) = parse_room(output.clone()) {
                if &r.name != "Security Checkpoint" {
                    println!("{}", output);
                }
            }
        }
    }

    Ok(())
}

pub fn p2() -> IoResult<()> {
    unimplemented!("Part 2")
}
