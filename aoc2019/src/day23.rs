use crate::intcode::IntCodeMachine;
use std::io::Result as IoResult;
use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::{HashSet, HashMap};

use std::convert::TryFrom;
use itertools::Itertools;

use std::fmt;

#[derive(Debug)]
struct Packet {
    x: isize,
    y: isize,
    m: usize,
}
pub fn p1() -> IoResult<()> {
     let codes = std::fs::read_to_string("input/day23.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();

    let mut machines = (0..50).map(|m| IntCodeMachine::new(codes.clone(), vec![m])).collect::<Vec<_>>();

    let mut packets :Vec<Packet>= vec![];
    
    'outer: loop {
        let mut input = HashSet::new();        
        for p in packets.drain(..) {
            if p.m == 255 {
                println!("{:?}", p);
                break 'outer
            }
            machines[p.m].feed_input(p.x);
            machines[p.m].feed_input(p.y);
            input.insert(p.m);
        }
        
        let out = machines.iter_mut().enumerate().flat_map(|(i, m)| {
            if !input.contains(&i) {
                m.feed_input(-1);
            }
            m.run();
            let v = m.output();
            //println!("{} {:?}", v.len(), v);
            v.into_iter()
        }).collect::<Vec<_>>();

        packets.extend(
            out.chunks(3).into_iter().map(
                |g| Packet { m: g[0] as usize, x: g[1], y: g[2]}
            )
        )
                          
    }
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
