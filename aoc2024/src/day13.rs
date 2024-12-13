use crate::Day;
use cgmath::{Point2, Vector2};
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, Copy)]
pub struct Arcade {
    button_a: Vector2<usize>,
    button_b: Vector2<usize>,
    prize: Point2<usize>,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 13;
    type Input1 = Vec<Arcade>;
    type Input2 = Vec<Arcade>;
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
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
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
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        v.iter_mut().for_each(|a| {
            let offset = 10000000000000;
            a.prize.x += offset;
            a.prize.y += offset;
        });

        v.iter()
            .map(|arcade| {
                let lcm = arcade.button_a.y;
                let eq1 = [
                    lcm * arcade.button_a.x,
                    lcm * arcade.button_b.x,
                    lcm * arcade.prize.x,
                ];
                let lcm = arcade.button_a.x;
                let eq2 = [
                    lcm * arcade.button_a.y,
                    lcm * arcade.button_b.y,
                    lcm * arcade.prize.y,
                ];

                assert_eq!(eq1[0] - eq2[0], 0);

                let choice = if (eq1[2] >= eq2[2]) && (eq1[1] >= eq2[1]) {
                    let b = (eq1[2] - eq2[2]) / (eq1[1] - eq2[1]);
                    Some(b)
                } else if (eq1[2] <= eq2[2]) && (eq1[1] <= eq2[1]) {
                    let b = (eq2[2] - eq1[2]) / (eq2[1] - eq1[1]);
                    Some(b)
                } else {
                    None
                };

                if let Some(b) = choice {
                    // arcade.prize.x  = a * arcade.button_a.x + b * arcade_button_b.x;
                    if arcade.prize.x > b * arcade.button_b.x {
                        let a = (arcade.prize.x - b * arcade.button_b.x) / arcade.button_a.x;

                        if arcade.prize.x == a * arcade.button_a.x + b * arcade.button_b.x
                            && arcade.prize.y == a * arcade.button_a.y + b * arcade.button_b.y
                        {
                            return a * 3 + b;
                        }
                    }
                }
                0
            })
            .sum()
    }
}

crate::default_tests!(31552, 95273925552482);
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
    [(
        foo_sol2,
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
        875318608908
    )]
);
