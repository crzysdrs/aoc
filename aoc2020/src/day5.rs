use crate::Day;
use std::collections::*;
use std::io::Result as IoResult;
use std::ops::Range;

pub enum Dir {
    Front,
    Left,
    Right,
    Back,
}
pub struct Solution {}

struct Seat {
    row: Range<usize>,
    col: Range<usize>,
}

impl Seat {
    fn apply(&mut self, d: &Dir) {
        match d {
            Dir::Front => {
                self.row = self.row.start..(self.row.start + self.row.clone().count() / 2)
            }
            Dir::Back => self.row = (self.row.start + self.row.clone().count() / 2)..self.row.end,
            Dir::Left => self.col = self.col.start..(self.col.start + self.col.clone().len() / 2),
            Dir::Right => self.col = (self.col.start + self.col.clone().len() / 2)..self.col.end,
        }
    }
    fn id(&self) -> usize {
        //println!("{:?} {:?}", self.row, self.col);
        self.row.start * 8 + self.col.start
    }
}

fn build_seats<'a>(seats: &'a [Vec<Dir>]) -> impl Iterator<Item = Seat> + 'a {
    seats.iter().map(|bsp| {
        bsp.iter().fold(
            Seat {
                row: 0..128,
                col: 0..8,
            },
            |mut state, x| {
                state.apply(&x);
                state
            },
        )
    })
}
impl Day for Solution {
    const DAY: u32 = 5;
    type Input = Vec<Dir>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                Ok(l?
                    .chars()
                    .map(|x| match x {
                        'F' => Dir::Front,
                        'L' => Dir::Left,
                        'R' => Dir::Right,
                        'B' => Dir::Back,
                        _ => unreachable!(),
                    })
                    .collect())
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        build_seats(v).map(|s| s.id()).max().unwrap()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let seats = build_seats(v).map(|s| s.id()).collect::<HashSet<_>>();
        
        for r in 0..128 {
            for c in 0..8 {
                let seat = r * 8 + c;
                if !seats.contains(&seat)
                    && seats.contains(&(seat + 1))
                    && seats.contains(&(seat - 1))
                {
                    return seat;
                }
            }
        }
        panic!("Couldn't find seat");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let seats = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";
        let seats = Solution::process_input(std::io::BufReader::new(seats.as_bytes())).unwrap();
        let seats = build_seats(&seats).map(|s| s.id()).collect::<Vec<_>>();
        assert_eq!(&seats, &[567, 119, 820]);
    }
}
