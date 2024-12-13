use crate::Day;
use cgmath::{Point2, Vector2};
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub struct Arcade {
    button_a: Vector2<i32>,
    button_b: Vector2<i32>,
    prize: Point2<i32>,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 13;
    type Input1 = Vec<Arcade>;
    type Input2 = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();

        let mut arcade = vec![];
        loop {
            let mut entry = lines.by_ref().take_while(|s| !s.is_empty());
            let button = Regex::new(r"Button (?:A|B): X\+([0-9]+), Y\+([0-9]+)").unwrap();
            let prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

            let Some(l1) = entry.next() else {
                break;
            };
            let l2 = entry.next().unwrap();
            let l3 = entry.next().unwrap();

            let _ = entry.next();
            let m1 = button.captures(l1).unwrap();
            let m2 = button.captures(l2).unwrap();
            let m3 = prize.captures(l3).unwrap();

            let a = Arcade {
                button_a: Vector2::new(m1[1].parse().unwrap(), m1[2].parse().unwrap()),
                button_b: Vector2::new(m2[1].parse().unwrap(), m2[2].parse().unwrap()),
                prize: Point2::new(m3[1].parse().unwrap(), m3[2].parse().unwrap()),
            };
            arcade.push(a);
        }

        arcade
    }
    fn process_input2(_s: &str) -> Self::Input2 {
        unimplemented!()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .map(|arcade| {
                let mut prize_choice = None;
                'prize: for a in 0..100 {
                    for b in 0..100 {
                        if arcade.prize.x == a * arcade.button_a.x + b * arcade.button_b.x
                            && arcade.prize.y == a * arcade.button_a.y + b * arcade.button_b.y
                        {
                            prize_choice = Some((a, b));
                            break 'prize;
                        }
                    }
                }

                if let Some((a, b)) = prize_choice {
                    a as usize * 3 + b as usize
                } else {
                    0
                }
            })
            .sum()
    }
    fn p2(_v: &Self::Input2) -> Self::Sol2 {
        unimplemented!()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        480
    )],
    [(foo_sol2, "hi2", 1)]
);
