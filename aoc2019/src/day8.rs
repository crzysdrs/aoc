use itertools::Itertools;
use std::io::Result as IoResult;

pub fn p1() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = 
        std::fs::read_to_string("input/day8.txt")?
        .trim().chars().map(|c| c.to_digit(10).unwrap()).chunks(wide * tall).into_iter().map(
            |c| c.collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let m = v
        .iter()
        .map(|l| {
            l.iter()
                .map(|d| if *d == 0 { 1 } else { 0 })
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let l = &v[m.0];
    let counts = (
        l.iter()
            .map(|d| (if *d == 1 { 1 } else { 0 }))
            .sum::<u32>(),
        l.iter()
            .map(|d| if *d == 2 { 1 } else { 0 })
            .sum::<u32>(),
    );
    println!("Part 1: {}", counts.0 * counts.1);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = 
        std::fs::read_to_string("input/day8.txt")?
        .trim().chars().map(|c| c.to_digit(10).unwrap()).chunks(wide * tall).into_iter()        
        .fold(None, |acc: Option<Vec<_>>, l| {
            if let Some(old) = acc {
                Some(
                    old.iter()
                        .zip(l)
                        .map(|(o, n)| {
                            if *o == 2 { n } else { *o }
                        })
                        .collect()
                )
            } else {
                Some(l.collect())
            }
        })
        .unwrap();

    println!("Part 2:");
    for y in 0..tall {
        for x in 0..wide {
            print!("{}", match v[y * wide + x] {
                0 => "▉",
                1 => " ",
                2 => " ",
                _ => panic!("Invalid Pixel Color"),
            })
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
