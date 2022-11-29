use crate::intcode::IntCodeMachine;
use std::io::Result as IoResult;

use itertools::Itertools;

pub fn series_machine(codes: &[isize], feedback: bool) -> isize {
    let starts = if feedback { 5..=9 } else { 0..=4 };
    starts
        .permutations(5)
        .map({
            |phases| {
                let mut machines = phases
                    .iter()
                    .enumerate()
                    .map(|(i, phase)| (i, IntCodeMachine::new(codes.to_vec(), vec![*phase])))
                    .collect::<Vec<_>>();

                let mut start_input = 0;
                loop {
                    let v = machines
                        .iter_mut()
                        .scan(start_input, |input, (_num, machine)| {
                            machine.feed_input(*input);
                            machine.run();
                            *input = machine.next_output().unwrap();
                            Some(*input)
                        })
                        .last()
                        .unwrap();
                    if machines[0].1.halted() {
                        break v;
                    } else {
                        start_input = v;
                    }
                }
            }
        })
        .max()
        .unwrap()
}

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let r = series_machine(&codes, false);
    println!("Day 7 Part 1 {}", r);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day7.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let r = series_machine(&codes, true);
    println!("Day 7 Part 1 {}", r);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            series_machine(
                &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                false
            ),
            43210
        );
        assert_eq!(
            series_machine(
                &[
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                false
            ),
            54321
        );
        assert_eq!(
            series_machine(
                &[
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                false
            ),
            65210
        );

        assert_eq!(
            series_machine(
                &[
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                true
            ),
            139629729
        );
        assert_eq!(
            series_machine(
                &[
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                true
            ),
            18216
        );
    }
}
