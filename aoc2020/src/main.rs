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

trait Day {
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
}

mod day1;
mod day2;
mod template;

macro_rules! run_test {
    ($name:expr, $part:expr, $([$val:pat, $test:ident]),* ) => {
        match ($name, $part) {
            $(
                ($val, 1) => {
                    $test::Solution::run_p1()
                }
                ($val, 2) => {
                    $test::Solution::run_p2()
                }
            )*,
            _ => {
                let err = format!("Unknown Test (Day {} Part {})", $name, $part);
                Err(std::io::Error::new(std::io::ErrorKind::Other, err))
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    run_test!(opts.test, opts.part, [1, day1], [2, day2])
}
