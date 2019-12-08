use std::io::Result as IoResult;

use crate::intcode::IntCodeMachine;

pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day5.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut machine = IntCodeMachine::new(codes, vec![1]);
    machine.run();
    machine.done("Part 1");
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day5.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    let mut machine = IntCodeMachine::new(codes, vec![5]);
    machine.run();
    machine.done("Part 2");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]).test(),
            (vec![], vec![1])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![5]).test(),
            (vec![], vec![0])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![5]).test(),
            (vec![], vec![1])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]).test(),
            (vec![], vec![0])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8]).test(),
            (vec![], vec![1])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![7]).test(),
            (vec![], vec![0])
        );

        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![8]).test(),
            (vec![], vec![0])
        );
        assert_eq!(
            IntCodeMachine::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![7]).test(),
            (vec![], vec![1])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![0]
            )
            .test(),
            (vec![], vec![0])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                vec![1]
            )
            .test(),
            (vec![], vec![1])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![0]
            )
            .test(),
            (vec![], vec![0])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                vec![1]
            )
            .test(),
            (vec![], vec![1])
        );

        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![7]
            )
            .test(),
            (vec![], vec![999])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![8]
            )
            .test(),
            (vec![], vec![1000])
        );
        assert_eq!(
            IntCodeMachine::new(
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
                ],
                vec![9]
            )
            .test(),
            (vec![], vec![1001])
        );
    }
}
