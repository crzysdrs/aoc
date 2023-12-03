use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    y: usize,
    x: Range<usize>,
}

impl Pos {
    fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.x.clone().map(|x| (x, self.y))
    }
    fn adj(&self) -> Vec<(usize, usize)> {
        let off = [
            (0, 1),
            (1, 1),
            (1, 0),
            (-1, -1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (1, -1),
        ];
        self.points()
            .flat_map(|p| {
                off.iter().flat_map(move |(o1, o2)| {
                    let x = i32::try_from(p.0).unwrap().checked_add(*o1)?;
                    let y = i32::try_from(p.1).unwrap().checked_add(*o2)?;
                    Some((x.try_into().ok()?, y.try_into().ok()?))
                })
            })
            .collect()
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Number {
    loc: Pos,
    val: usize,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Number(Number),
    Part(char, Pos),
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 3;
    type Input1 = HashSet<Item>;
    type Input2 = HashSet<Item>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut y = 0;
        let mut map = HashSet::new();

        s.lines().for_each(|s| {
            let mut chars = s.char_indices();

            while let Some((i, c)) = chars.by_ref().next() {
                match c {
                    '.' => {}
                    '0'..='9' => {
                        let last = chars
                            .as_str()
                            .char_indices()
                            .take_while(|s| s.1.is_ascii_digit())
                            .last();
                        let extra = last.map(|l| l.0 + 1).unwrap_or(0);
                        let range = i..i + 1 + extra;

                        map.insert(Item::Number(Number {
                            val: s[range.clone()].parse().unwrap(),
                            loc: Pos {
                                x: range.clone(),
                                y,
                            },
                        }));
                        if extra > 0 {
                            chars.by_ref().nth(extra - 1);
                        }
                    }
                    _ => {
                        map.insert(Item::Part(c, Pos { y, x: i..i + 1 }));
                    }
                }
            }

            y += 1;
        });

        map
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut m = HashSet::new();
        for p in v {
            let Item::Part(_c, pos) = p else { continue };
            let adj = pos.adj();

            for p2 in v {
                let Item::Number(n) = p2 else { continue };

                for p in n.loc.points() {
                    if adj.contains(&p) {
                        m.insert(n);
                        break;
                    }
                }
            }
        }
        m.iter().map(|n| n.val).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut m: HashMap<&Item, HashSet<&Number>> = HashMap::new();

        for part in v {
            let Item::Part(_c, pos) = part else { continue };
            let adj = pos.adj();

            for p2 in v {
                let Item::Number(n) = p2 else { continue };

                for p in n.loc.points() {
                    if adj.contains(&p) {
                        m.entry(part).or_default().insert(n);
                        break;
                    }
                }
            }
        }

        m.iter()
            .filter(|(_k, v)| v.len() == 2)
            .map(|(_k, v)| v.iter().map(|n| n.val).product::<usize>())
            .sum()
    }
}

crate::default_tests!(512794, 67779080);
crate::string_tests!(
    [(
        foo_sol1,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        4361
    )],
    [(
        foo_sol2,
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        467835
    )]
);
