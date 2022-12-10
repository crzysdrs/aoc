use crate::Day;
use itertools::Itertools;
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

#[derive(Clone, Debug)]
struct Cpu {
    cycle: usize,
    x: i32,
}

fn run_cpu(cmds: &[Cmd]) -> impl Iterator<Item = Cpu> + itertools::PeekingNext + '_ {
    let mut cpu = Cpu { cycle: 0, x: 1 };

    std::iter::once(cpu.clone())
        .chain(cmds.iter().map(move |cmd| {
            if let Cmd::Addx(v) = cmd {
                cpu.x += v;
            }
            cpu.cycle += cmd.cycle_count();

            cpu.clone()
        }))
        .peekable()
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
        let mut cpu_state = run_cpu(v);

        (0..=220)
            .skip(20)
            .step_by(40)
            .map_while(|x| {
                cpu_state
                    .by_ref()
                    .peeking_take_while(|cpu| cpu.cycle < x)
                    .last()
                    .map(|cpu| cpu.x * x as i32)
            })
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut cpu_state = run_cpu(v);

        let mut pixels = vec!['.'; 40 * 6];

        let mut last = None;
        pixels.iter_mut().enumerate().for_each(|(crt_pos, p)| {
            if let Some(cpu) = cpu_state
                .by_ref()
                .peeking_take_while(|cpu| cpu.cycle <= crt_pos)
                .last()
            {
                last = Some(cpu);
            }

            if let Some(cpu) = &last {
                if ((crt_pos % 40) as i32 - cpu.x).abs() <= 1 {
                    *p = '#';
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
