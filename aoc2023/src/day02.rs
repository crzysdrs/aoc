use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}
pub struct Draw {
    marbles: Vec<Marble>,
}
pub struct Marble {
    count: usize,
    color: Color,
}
pub struct Game {
    id: usize,
    draw: Vec<Draw>,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 2;
    type Input1 = Vec<Game>;
    type Input2 = Vec<Game>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|x| {
                let (game, commands) = x.split_once(':').unwrap();
                let game = game.strip_prefix("Game ").unwrap().parse().unwrap();
                Game {
                    id: game,
                    draw: commands
                        .split(';')
                        .map(|m| Draw {
                            marbles: m
                                .split(',')
                                .map(|m| {
                                    let (num, color) = m.trim().split_once(' ').unwrap();
                                    let num = num.parse().unwrap();
                                    let color = match color {
                                        "red" => Color::Red,
                                        "green" => Color::Green,
                                        "blue" => Color::Blue,
                                        _ => panic!(),
                                    };
                                    Marble { count: num, color }
                                })
                                .collect(),
                        })
                        .collect(),
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.iter()
            .filter(|g| {
                g.draw.iter().all(|d| {
                    d.marbles.iter().all(|m| {
                        let count = match m.color {
                            Color::Red => 12,
                            Color::Green => 13,
                            Color::Blue => 14,
                        };
                        m.count <= count
                    })
                })
            })
            .map(|g| g.id)
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        v.iter()
            .map(|game| {
                let count: (usize, usize, usize) = game
                    .draw
                    .iter()
                    .flat_map(|d| d.marbles.iter())
                    .map(|m| match m.color {
                        Color::Red => (m.count, 0, 0),
                        Color::Green => (0, m.count, 0),
                        Color::Blue => (0, 0, m.count),
                    })
                    .fold((0, 0, 0), |state, v| {
                        (
                            std::cmp::max(state.0, v.0),
                            std::cmp::max(state.1, v.1),
                            std::cmp::max(state.2, v.2),
                        )
                    });

                count.0 * count.1 * count.2
            })
            .sum()
    }
}

crate::default_tests!(2156, 66909);
crate::string_tests!(
    [(
        sol1,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        8
    )],
    [(
        sol2,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        2286
    )]
);
