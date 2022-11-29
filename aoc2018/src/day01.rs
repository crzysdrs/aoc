
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn p1() -> std::io::Result<()> {
    let f = BufReader::new(File::open("input/day1.txt")?);
    let s: i32 = f
        .lines()
        .map(Result::unwrap)
        .map(|x: String| x.parse::<i32>().expect("Valid int"))
        .sum();
    println!("Frequency: {}", s);
    Ok(())
}
pub fn p2() -> std::io::Result<()> {
    let f = BufReader::new(File::open("input/day1.txt")?);
    let v: Vec<i32> = f
        .lines()
        .map(Result::unwrap)
        .map(|x: String| x.parse::<i32>().expect("Valid int"))
        .collect();
    let mut hs = std::collections::HashSet::<i32>::new();
    let freq = v.iter().cycle().try_fold(0, |mut acc, x| {
        if hs.contains(&acc) {
            Err(acc)
        } else {
            hs.insert(acc);
            acc += x;
            Ok(acc)
        }
    });
    println!("second freq: {}", freq.unwrap_err());
    Ok(())
}
