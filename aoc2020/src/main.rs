use clap::Clap;
use std::io::BufReader;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Clap)]
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
    fn both() -> (Box<dyn Fn() -> IoResult<()>>, Box<dyn Fn() -> IoResult<()>>) {
        (Box::new(Self::run_p1), Box::new(Self::run_p2))
    }
}

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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

    let sols = tests!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11);

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
