use itertools::Itertools;
use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

pub fn p1() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = //"123456789012"
        std::fs::read_to_string("input/day8.txt")?
        .trim().chars().map(|c| c.to_digit(10).unwrap()).chunks(wide).into_iter().map(
        |c| c.collect::<Vec<_>>()
    ).chunks(tall).into_iter().map(
        |c| c.collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let m = v
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|r| r.iter())
                .map(|d| if *d == 0 { 1 } else { 0 })
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let l = &v[m.0];
    let counts = (
        l.iter()
            .flat_map(|r| r.iter())
            .map(|d| if *d == 1 { 1 } else { 0 })
            .sum::<u32>(),
        l.iter()
            .flat_map(|r| r.iter())
            .map(|d| if *d == 2 { 1 } else { 0 })
            .sum::<u32>(),
    );
    println!("{}", counts.0 * counts.1);
    //println!("{:?}", v);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = //"123456789012"
        std::fs::read_to_string("input/day8.txt")?
        .trim().chars().map(|c| c.to_digit(10).unwrap()).chunks(wide).into_iter().map(
        |c| c.collect::<Vec<_>>()
    ).chunks(tall).into_iter().map(
        |c| c.collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let m = v
        .iter()
        .fold(None, |acc: Option<Vec<Vec<u32>>>, l| {
            if let Some(old) = acc {
                Some(
                    old.iter()
                        .zip(l)
                        .map(|(o, n)| {
                            o.iter()
                                .zip(n)
                                .map(|(o, n)| if *o == 2 { *n } else { *o })
                                .collect()
                        })
                        .collect(),
                )
            } else {
                Some(l.to_vec())
            }
        })
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    //let m = m.iter().map(|x| x.iter()).flatten().collect::<Vec<_>>();

    for y in 0..tall {
        for x in 0..wide {
            print!("{}", if m[y * wide + x] == 0 { " " } else { "X" });
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(false);
    }
}
