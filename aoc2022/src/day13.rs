use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, map_res, recognize},
    multi::many1,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    Elem(u32),
    Elems(Vec<Packet>),
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map_res(recognize(many1(one_of("0123456789"))), |s: &str| {
            Ok::<_, ()>(Packet::Elem(s.parse().map_err(|_| ())?))
        }),
        map(
            delimited(tag("["), separated_list0(tag(","), packet), tag("]")),
            Packet::Elems,
        ),
    ))(input)
}

use std::cmp::Ordering;
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Elem(v), Packet::Elem(v2)) => v.cmp(v2),
            (Packet::Elem(v), Packet::Elems(vec)) => ([Packet::Elem(*v)][..]).cmp(&vec[..]),
            (Packet::Elems(vec), Packet::Elem(v)) => (vec[..]).cmp(&[Packet::Elem(*v)][..]),
            (Packet::Elems(vec), Packet::Elems(vec2)) => vec.cmp(vec2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 13;
    type Input1 = Vec<(Packet, Packet)>;
    type Input2 = Vec<(Packet, Packet)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let mut packets = vec![];
        loop {
            let l = lines.by_ref().take(2).collect::<Vec<_>>();
            if l.is_empty() {
                break;
            }
            let p1 = packet(l[0]).unwrap();
            let p2 = packet(l[1]).unwrap();

            lines.by_ref().next();
            packets.push((p1.1, p2.1));
        }
        packets
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .zip(1..)
            .filter(|((p1, p2), _pos)| {
                matches!(p1.partial_cmp(p2), Some(Ordering::Less | Ordering::Equal))
            })
            .map(|(_, pos)| pos)
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let div = (
            Packet::Elems(vec![Packet::Elem(2)]),
            Packet::Elems(vec![Packet::Elem(6)]),
        );

        let mut all_packets: Vec<_> = v
            .iter()
            .cloned()
            .chain(std::iter::once(div.clone()))
            .flat_map(|(p1, p2)| vec![p1, p2].into_iter())
            .collect();

        all_packets.sort();

        let p1 = all_packets.iter().position(|v| *v == div.0).unwrap();
        let p2 = all_packets.iter().position(|v| *v == div.1).unwrap();

        (p1 + 1) * (p2 + 1)
    }
}

crate::default_tests!(4809, 22600);
crate::path_tests!([(t1, "test/day13.txt", 13)], [(t2, "test/day13.txt", 140)]);
