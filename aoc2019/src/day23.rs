use crate::intcode::IntCodeMachine;
use std::collections::{HashMap, HashSet};
use std::io::Result as IoResult;

#[derive(Debug)]
struct Packet {
    x: isize,
    y: isize,
    m: usize,
}
pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day23.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let mut machines = (0..50)
        .map(|m| IntCodeMachine::new(codes.clone(), vec![m]))
        .collect::<Vec<_>>();

    let mut packets: Vec<Packet> = vec![];

    'outer: loop {
        let mut input = HashSet::new();
        for p in packets.drain(..) {
            if p.m == 255 {
                println!("{:?}", p);
                break 'outer;
            }
            machines[p.m].feed_input(p.x);
            machines[p.m].feed_input(p.y);
            input.insert(p.m);
        }

        let out = machines
            .iter_mut()
            .enumerate()
            .flat_map(|(i, m)| {
                if !input.contains(&i) {
                    m.feed_input(-1);
                }
                m.run();
                let v = m.output();
                //println!("{} {:?}", v.len(), v);
                v.into_iter()
            })
            .collect::<Vec<_>>();

        packets.extend(out.chunks(3).into_iter().map(|g| Packet {
            m: g[0] as usize,
            x: g[1],
            y: g[2],
        }))
    }
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day23.txt")?
        .trim()
        .split(',')
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let mut machines = (0..50)
        .map(|m| IntCodeMachine::new(codes.clone(), vec![m]))
        .collect::<Vec<_>>();

    let mut packets: Vec<Packet> = vec![];

    let mut nat = None;

    let mut nat_packets = HashMap::new();

    'outer: loop {
        let mut input = HashSet::new();
        for p in packets.drain(..) {
            if p.m == 255 {
                nat = Some(p);
                continue;
            }
            machines[p.m].feed_input(p.x);
            machines[p.m].feed_input(p.y);
            input.insert(p.m);
        }

        if input.is_empty() && machines.iter().all(|m| !m.has_input()) {
            let mut p = nat.take().unwrap();
            p.m = 0;
            *nat_packets.entry(p.y).or_insert(0) += 1;
            if let Some(find) = nat_packets.iter().find(|(_k, v)| **v == 2) {
                println!("{:?}", find);
                break 'outer;
            }
            input.insert(p.m);
            packets.push(p);
        }

        let out = machines
            .iter_mut()
            .enumerate()
            .flat_map(|(i, m)| {
                if !input.contains(&i) {
                    m.feed_input(-1);
                }
                m.run();
                let v = m.output();
                //println!("{} {:?}", v.len(), v);
                v.into_iter()
            })
            .collect::<Vec<_>>();

        packets.extend(out.chunks(3).into_iter().map(|g| Packet {
            m: g[0] as usize,
            x: g[1],
            y: g[2],
        }))
    }
    Ok(())
}
