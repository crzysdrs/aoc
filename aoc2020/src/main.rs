use clap::Clap;
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
    fn input() -> PathBuf {
        PathBuf::from("input")
            .join(format!("day{}.txt", Self::DAY))
    }
    fn p1() -> IoResult<()> {
        unimplemented!("Missing implementation of Day {} Part 1", Self::DAY)
    }
    fn p2() -> IoResult<()> {
        unimplemented!("Missing implementation of Day {} Part 2", Self::DAY)
    }
}

mod day1;


macro_rules! run_test {
    ($name:expr, $part:expr, $([$val:pat, $test:ident]),* ) => {
        match ($name, $part) {
            $(
                ($val, 1) => {
                    $test::Solution::p1()
                }
                ($val, 2) => {
                    $test::Solution::p2()
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

    run_test!(
        opts.test, opts.part,
        [1, day1]
    )
}
