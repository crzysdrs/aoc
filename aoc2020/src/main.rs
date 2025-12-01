use clap::Parser;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version = "1.0", author = "Mitch Souders <crzysdrs@gmail.com>")]
struct Opts {
    test: u32,
    part: u32,
}

trait Day
where
    Self: 'static,
{
    const DAY: u32;
    type Input;
    type Sol1: std::fmt::Display;
    type Sol2: std::fmt::Display;
    fn input() -> PathBuf {
        PathBuf::from("input").join(format!("day{}.txt", Self::DAY))
    }
    fn run_p1() -> IoResult<()> {
        let mut buf = BufReader::new(std::fs::File::open(Self::input())?);
        let v = Self::process_input(&mut buf)?;
        println!("Solution Day {} Part 1", Self::DAY);
        println!("{}", Self::p1(&v));
        Ok(())
    }
    fn run_p2() -> IoResult<()> {
        let mut buf = BufReader::new(std::fs::File::open(Self::input())?);
        let v = Self::process_input(&mut buf)?;
        println!("Solution Day {} Part 2", Self::DAY);
        println!("{}", Self::p2(&v));
        Ok(())
    }
    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead;
    fn p1(_input: &[Self::Input]) -> Self::Sol1 {
        unimplemented!("Missing implementation of Day {} Part 1", Self::DAY)
    }
    fn p2(_input: &[Self::Input]) -> Self::Sol2 {
        unimplemented!("Missing implementation of Day {} Part 2", Self::DAY)
    }
    #[allow(clippy::type_complexity)]
    fn both() -> (Box<dyn Fn() -> IoResult<()>>, Box<dyn Fn() -> IoResult<()>>) {
        (Box::new(Self::run_p1), Box::new(Self::run_p2))
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod template;

macro_rules! tests {
    ($($name:ident),*) => {
        [
            $(
                ($name::Solution::DAY, $name::Solution::both()),
            )*
        ]
    }
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let sols = tests!(
        template, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
        day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24,
        day25
    );

    if let Some(sol) = sols.iter().find(|s| s.0 == opts.test) {
        match opts.part {
            1 => (sol.1 .0)()?,
            2 => (sol.1 .1)()?,
            p => {
                let err = format!("Unknown Test (Day {} Part {})", sol.0, p);
                Err(std::io::Error::new(std::io::ErrorKind::Other, err))?
            }
        }
    } else {
        let err = format!("Unknown Test (Day {} Part {})", opts.test, opts.part);
        Err(std::io::Error::new(std::io::ErrorKind::Other, err))?
    }

    Ok(())
}
