use itertools::Itertools;
use std::io::Result as IoResult;

pub fn p1() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = std::fs::read_to_string("input/day8.txt")?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chunks(wide * tall)
        .into_iter()
        .map(|c| c.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let m = v
        .iter()
        .map(|l| l.iter().map(|d| u32::from(*d == 0)).sum::<u32>())
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let l = &v[m.0];
    let counts = (
        l.iter().map(|d| u32::from(*d == 1)).sum::<u32>(),
        l.iter().map(|d| u32::from(*d == 2)).sum::<u32>(),
    );
    println!("Part 1: {}", counts.0 * counts.1);
    Ok(())
}

fn compute_image(wide: usize, tall: usize, s: &str) -> Vec<u32> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chunks(wide * tall)
        .into_iter()
        .fold(None, |acc: Option<Vec<_>>, l| {
            if let Some(old) = acc {
                Some(
                    old.iter()
                        .zip(l)
                        .map(|(o, n)| if *o == 2 { n } else { *o })
                        .collect(),
                )
            } else {
                Some(l.collect())
            }
        })
        .unwrap()
}

fn draw_image(wide: usize, tall: usize, v: &[u32]) {
    for y in 0..tall {
        for x in 0..wide {
            print!(
                "{}",
                match v[y * wide + x] {
                    0 => "▉",
                    1 => " ",
                    2 => " ",
                    _ => panic!("Invalid Pixel Color"),
                }
            )
        }
        println!();
    }
}
pub fn p2() -> IoResult<()> {
    let wide = 25;
    let tall = 6;
    let v = compute_image(wide, tall, &std::fs::read_to_string("input/day8.txt")?);

    println!("Part 2:");
    draw_image(wide, tall, &v);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(compute_image(2, 2, "0222112222120000"), &[0, 1, 1, 0]);
    }
}
