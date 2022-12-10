use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub enum Cmd {
    Nop,
    Addx(i32),
}

impl Cmd {
    fn cycle_count(&self) -> usize {
        match self {
            Self::Nop => 1,
            Self::Addx(_i32) => 2,
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input1 = Vec<Cmd>;
    type Input2 = Vec<Cmd>;
    type Sol1 = i32;
    type Sol2 = String;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                if x == "noop" {
                    Cmd::Nop
                } else {
                    let (addx, val) = x.split_once(' ').unwrap();
                    assert_eq!(addx, "addx");
                    Cmd::Addx(val.parse().unwrap())
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        #[derive(Clone, Debug)]
        struct Cpu {
            cycle: usize,
            x: i32,
        }
        let mut cpu = Cpu { cycle: 0, x: 1 };
        let v: Vec<_> = v
            .iter()
            .scan(cpu, |cpu, cmd| {
                if let Cmd::Addx(v) = cmd {
                    cpu.x += v;
                }
                cpu.cycle += cmd.cycle_count();
                Some(cpu.clone())
            })
            .collect();

        let mut total = v.last().unwrap();
        let mut pts = vec![];
        for x in (0..total.cycle).skip(20).step_by(40) {
            let found = v.binary_search_by_key(&x, |cpu| cpu.cycle);
            match found {
                Ok(pos) | Err(pos) => {
                    let pos = pos.saturating_sub(1);
                    println!("{:?}", &v[pos]);
                    pts.push(v[pos].x * x as i32)
                }
            }
        }

        pts.iter().sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        #[derive(Clone, Debug)]
        struct Cpu {
            cycle: usize,
            x: i32,
        }
        let mut cpu = Cpu { cycle: 0, x: 1 };
        let mut v: Vec<_> = std::iter::once(cpu.clone())
            .chain(v.iter().scan(cpu, |cpu, cmd| {
                match cmd {
                    Cmd::Addx(v) => cpu.x += v,
                    _ => {}
                }
                cpu.cycle += cmd.cycle_count();
                Some(cpu.clone())
            }))
            .collect();

        let mut pixels = vec!['.'; 40 * 6];

        pixels.iter_mut().enumerate().for_each(|(crt_pos, p)| {
            let found = v.binary_search_by_key(&(crt_pos + 1), |cpu| cpu.cycle);
            match found {
                Ok(pos) | Err(pos) => {
                    let pos = pos.saturating_sub(1);
                    if ((crt_pos % 40) as i32 - v[pos].x).abs() <= 1 {
                        *p = '#';
                    }
                }
            }
        });

        let screen = pixels
            .chunks_exact(40)
            .map(|c| {
                let mut s = c.iter().collect::<String>();
                s.push('\n');
                s
            })
            .collect::<String>();

        screen
    }
}

crate::default_tests!(
    12540,
    "####.####..##..####.####.#....#..#.####.\n\
     #....#....#..#....#.#....#....#..#.#....\n\
     ###..###..#......#..###..#....####.###..\n\
     #....#....#.....#...#....#....#..#.#....\n\
     #....#....#..#.#....#....#....#..#.#....\n\
     #....####..##..####.####.####.#..#.####.\n"
);

crate::path_tests!(
    [(t1, "test/day10.txt", 13140)],
    [(
        t2,
        "test/day10.txt",
        "##..##..##..##..##..##..##..##..##..##..\n\
         ###...###...###...###...###...###...###.\n\
         ####....####....####....####....####....\n\
         #####.....#####.....#####.....#####.....\n\
         ######......######......######......####\n\
         #######.......#######.......#######.....\n"
    )]
);
