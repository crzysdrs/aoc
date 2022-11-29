use std::io::Result as IoResult;

use itertools::Itertools;
use std::collections::HashMap;

// fn compute_phase(input : String, repeat: usize, count: usize, offset: usize) -> String {
//     let base = [0, 1, 0, -1];
//     let mut v = input;
//     let mut seen = HashMap::new();
//     for p in 0..count {
//         println!("Phase {}", p);
//         v = (1..=v.len()).map(|i| {
//             let pattern_length = base.len().pow(i) - 1;

//             let base = base.iter().flat_map(
//                 |b| itertools::repeat_n(b, i)
//             ).skip(offset).take(v.len() + 1).collect::<Vec<_>>();

//             let base = base.iter().cycle().skip(1);

//             let v : u32 = v.chars().map(|x| x.to_digit(10).unwrap() as i32)
//                 .zip(base)
//                 .map(|(x, b)| x * **b).sum::<i32>().abs() as u32 % 10;

//             std::char::from_digit(v, 10).unwrap()
//         }
//         ).collect::<String>();
//     }
//     v
// }

#[allow(unused)]
fn compute_phase2(input: String, repeat: usize, count: usize, offset: usize) -> String {
    let mut input = input;
    for p in 0..count {
        //let mut sums = HashMap::new();
        println!("Phase {}", p);
        input = (1..=input.len() + offset)
            .skip(offset)
            .map(|i| {
                let v = (0..)
                    .map(|c| {
                        let mut v = (i - 1)..(i - 1 + i);
                        v.start += 2 * i * c - offset;
                        v.end += 2 * i * c - offset;
                        if v.end >= input.len() {
                            v.end = input.len();
                        }
                        v
                    })
                    .take_while(|v| v.start < input.len() && v.end <= input.len())
                    .zip([false, true].iter().cycle())
                    .map(|(s, neg)| {
                        // let mut new = s.start..std::cmp::min((s.start / repeat + 1) * repeat, s.end);

                        // let mut new_sum = 0;
                        // loop {
                        //     let search = (new.start % repeat)..if new.end % repeat == 0 {repeat} else {new.end % repeat};
                        //     //                        println!("New {:?} Search {:?}", new, search);
                        //     sum
                        //     if let Some(s) = sums.get(&search.clone()) {
                        //         new_sum += s;
                        //     } else {
                        //         let s2 = input[new.clone()].chars().map(|c| c.to_digit(10).unwrap() as i32).sum::<i32>();
                        //         new_sum += s2;
                        //         //println!("{}", s2);
                        //         sums.insert(search.clone(), s2);
                        //     }
                        //     if s.end == new.end {
                        //         break;
                        //     }
                        //     new.start = new.end;
                        //     new.end = std::cmp::min(s.end, new.end + repeat);
                        // }
                        let new_sum = input[s]
                            .chars()
                            .map(|c| c.to_digit(10).unwrap() as i32)
                            .sum::<i32>();
                        if *neg {
                            -new_sum
                        } else {
                            new_sum
                        }
                    })
                    .sum::<i32>()
                    .abs()
                    % 10;
                v as u32
            })
            .map(|v| std::char::from_digit(v, 10).unwrap())
            .collect::<String>();
    }
    input
}

fn compute_phase(input: String, repeat: usize, count: usize, offset: usize) -> String {
    let base = [0, 1, 0, -1];
    let mut v = input;
    for _p in 0..count {
        let mut seen = HashMap::new();
        v = (1..=v.len())
            .map(|i| {
                let pattern_length = base.len().pow(i as u32);

                let base = base
                    .iter()
                    .flat_map(|b| itertools::repeat_n(b, i))
                    .skip(offset)
                    .take(v.len() + 1)
                    .collect::<Vec<_>>();

                let base = base.iter().cycle().skip(1);

                let v: u32 = v
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as i32)
                    .zip(base)
                    .chunks(repeat)
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let key = (i * repeat % pattern_length, pattern_length);
                        let got = seen.get(&key);
                        if i == 0 || got.is_none() {
                            let v = c.map(|(x, b)| x * **b).sum::<i32>().abs() as u32 % 10;
                            if i != 0 {
                                seen.insert(key, v);
                            }
                            v
                        } else {
                            *got.unwrap()
                        }
                    })
                    .sum::<u32>()
                    % 10;
                std::char::from_digit(v, 10).unwrap()
            })
            .collect::<String>();
    }
    v
}

pub fn p1() -> IoResult<()> {
    let v = "12345678".to_string();
    //let mut v = std::fs::read_to_string("input/day16.txt")?.trim().to_string();
    //let mut v = "80871224585914546619083218645595".to_string();
    let len = v.len();
    //let v2 = compute_phase2(v.clone(), len, 100);
    let v = compute_phase(v, len, 100, 0);

    println!("{} ", v.chars().take(8).collect::<String>());
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let v = std::fs::read_to_string("input/day16.txt")?
        .trim()
        .to_string();
    //let mut v = "03036732577212944063491565474664".to_string();
    //let mut v = "02935109699940807407585447034323".to_string();
    //let mut v = "03081770884921959731165446850517".to_string();
    let offset = v
        .chars()
        .take(7)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("{}", offset);
    println!("{}", v.len() * 10_000);
    let _len = v.len();

    assert!(v.len() / 2 <= offset);
    let mut s = v
        .repeat(10_000)
        .chars()
        .skip(offset)
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    s.reverse();
    for p in 0..100 {
        println!("Phase {}", p);
        s = s
            .iter()
            .scan(0, |acc, x| {
                *acc += x;
                *acc %= 10;
                Some(*acc)
            })
            .collect::<Vec<_>>();

        //println!("{}", s.len())
    }

    s.reverse();
    println!(
        "Hi, {}",
        s.iter()
            .take(8)
            .map(|x| std::char::from_digit(*x, 10).unwrap())
            .collect::<String>(),
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(true);
    }
}
