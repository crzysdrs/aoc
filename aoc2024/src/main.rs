use clap::Parser;
use std::collections::HashMap;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

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
    type Input1;
    type Input2;
    type Sol1: std::fmt::Display + PartialEq + std::fmt::Debug;
    type Sol2: std::fmt::Display + PartialEq + std::fmt::Debug;
    fn input1() -> PathBuf {
        let input = PathBuf::from("input").join(format!("day{:02}.txt", Self::DAY));
        if !input.exists() {
            panic!("Input does not exist: {}", input.display());
        }
        input
    }
    fn input2() -> PathBuf {
        Self::input1()
    }
    fn run_p1() -> IoResult<()> {
        Self::run_p1_path(&Self::input1())?;
        Ok(())
    }
    fn run_p1_path(path: &Path) -> IoResult<Self::Sol1> {
        let buf = std::fs::read_to_string(path)?;
        let v = Self::process_input1(&buf);
        println!("Solution Day {} Part 1", Self::DAY);
        let sol = Self::p1(&v);
        println!("{}", sol);
        Ok(sol)
    }
    fn run_p2() -> IoResult<()> {
        Self::run_p2_path(&Self::input2())?;
        Ok(())
    }
    fn run_p2_path(path: &Path) -> IoResult<Self::Sol2> {
        let buf = std::fs::read_to_string(path)?;
        let v = Self::process_input2(&buf);
        println!("Solution Day {} Part 2", Self::DAY);
        let sol = Self::p2(&v);
        println!("{}", sol);
        Ok(sol)
    }
    fn process_input1(r: &str) -> Self::Input1;
    fn process_input2(r: &str) -> Self::Input2;

    fn p1(_input: &Self::Input1) -> Self::Sol1 {
        unimplemented!("Missing implementation of Day {} Part 1", Self::DAY)
    }
    fn p2(_input: &Self::Input2) -> Self::Sol2 {
        unimplemented!("Missing implementation of Day {} Part 2", Self::DAY)
    }
}

type IoFn = fn() -> IoResult<()>;
macro_rules! tests {
    ($($name:ident),*) => {
        $(
            mod $name;
        )*
        fn tests() -> HashMap<u32, (IoFn, IoFn)> {
            [
                $(
                    (
                        $name::Solution::DAY,
                        ($name::Solution::run_p1 as IoFn,
                         $name::Solution::run_p2 as IoFn)
                    ),
                )*
            ].into_iter().collect()
        }
    }
}

#[macro_export]
macro_rules! default_tests {
    ($sol1:expr, $sol2:expr) => {
        #[cfg(test)]
        mod default_tests {
            use super::*;
            #[test]
            fn part1() {
                assert_eq!(Solution::run_p1_path(&Solution::input1()).unwrap(), $sol1);
            }
            #[test]
            fn part2() {
                assert_eq!(Solution::run_p2_path(&Solution::input2()).unwrap(), $sol2);
            }
        }
    };
}

#[macro_export]
macro_rules! string_tests {
    (
        [$(($name1:ident, $input1:expr, $sol1:expr)),*],
        [$(($name2:ident, $input2:expr, $sol2:expr)),*]
    ) => {
        #[cfg(test)]
        mod string_tests {
            use super::*;
            $(
                #[test]
                fn $name1() {
                    let v = Solution::process_input1($input1);
                    assert_eq!(Solution::p1(&v), $sol1);
                }
            )*
            $(
                #[test]
                fn $name2() {
                    let v = Solution::process_input2($input2);
                    assert_eq!(Solution::p2(&v), $sol2);
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! path_tests {
    (
        [$(($name1:ident, $input1:expr, $sol1:expr)),*],
        [$(($name2:ident, $input2:expr, $sol2:expr)),*]
    ) => {
        #[cfg(test)]
        mod path_tests {
            use super::*;
            $(
                #[test]
                fn $name1() {
                    let buf = std::fs::read_to_string($input1).expect("valid input file");
                    let v = Solution::process_input1(&buf);
                    assert_eq!(Solution::p1(&v), $sol1);
                }
            )*
            $(
                #[test]
                fn $name2() {
                    let buf = std::fs::read_to_string($input2).expect("valid input file");
                    let v = Solution::process_input2(&buf);
                    assert_eq!(Solution::p2(&v), $sol2);
                }
            )*
        }
    };
}

tests! {
    template
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let sols = tests();
    if let Some(sol) = sols.get(&opts.test) {
        match opts.part {
            1 => (sol.0)()?,
            2 => (sol.1)()?,
            p => {
                let err = format!("Unknown Test (Day {} Part {})", opts.test, p);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
            }
        }
    } else {
        let err = format!("Unknown Test (Day {} Part {})", opts.test, opts.part);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
    }

    Ok(())
}
