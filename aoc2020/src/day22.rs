use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

#[derive(Clone)]
pub struct Input {
    player1: Vec<usize>,
    player2: Vec<usize>,
}

use std::cmp::Ordering;

fn recursive_combat(game: u32, mut input: Input) -> (Input, Ordering) {
    let mut state: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    //let mut round = 1;
    //println!(" === Game {} === ", game);
    let winner = loop {
        //println!(" -- Round {} (Game {}) -- ", round, game);
        //println!("Player 1's deck: {:?}", input.player1);
        //println!("Player 2's deck: {:?}", input.player2);

        if input.player1.is_empty() {
            break (input, Ordering::Less);
        } else if input.player2.is_empty()
            || state.contains(&(input.player1.clone(), input.player2.clone()))
        {
            break (input, Ordering::Greater);
        }

        state.insert((input.player1.clone(), input.player2.clone()));

        let p1 = input.player1.remove(0);
        let p2 = input.player2.remove(0);
        //println!("Player 1 plays: {}", p1);
        //println!("Player 2 plays: {}", p2);

        let order = if input.player1.len() >= p1 && input.player2.len() >= p2 {
            //println!("Playing a sub-game to determine the winnner... ");

            //println!("...anyway, back to game {}.", game);
            recursive_combat(
                game + 1,
                Input {
                    player1: input.player1[..p1].to_vec(),
                    player2: input.player2[..p2].to_vec(),
                },
            )
            .1
        } else {
            p1.cmp(&p2)
        };

        match order {
            Ordering::Equal => {
                unreachable!()
            }
            Ordering::Greater => {
                input.player1.push(p1);
                input.player1.push(p2);
                //println!("Player 1 wins round {} of game {}!", round, game);
            }
            Ordering::Less => {
                input.player2.push(p2);
                input.player2.push(p1);
                //println!("Player 2 wins round {} of game {}!", round, game);
            }
        }
        //round += 1;
    };

    match winner {
        (_, Ordering::Greater) => {
            //println!("The winner of game {} is player 1!", game);
        }
        (_, Ordering::Less) => {
            //println!("The winner of game {} is player 2!", game);
        }
        _ => unreachable!(),
    }
    winner
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 22;
    type Input = Input;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let mut lines = r.lines();

        let player1 = lines
            .by_ref()
            .flatten()
            .skip(1)
            .take_while(|l| !l.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        let player2 = lines
            .by_ref()
            .flatten()
            .skip(1)
            .take_while(|l| !l.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(vec![Input { player1, player2 }])
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut input = v[0].clone();
        loop {
            if input.player1.is_empty() || input.player2.is_empty() {
                break;
            }

            let p1 = input.player1.remove(0);
            let p2 = input.player2.remove(0);
            match p1.cmp(&p2) {
                Ordering::Equal => {
                    input.player1.push(p1);
                    input.player2.push(p2);
                }
                Ordering::Greater => {
                    input.player1.push(p1);
                    input.player1.push(p2);
                }
                Ordering::Less => {
                    input.player2.push(p2);
                    input.player2.push(p1);
                }
            }
        }

        let p1_score = input
            .player1
            .iter()
            .rev()
            .zip(1..)
            .map(|(i, j)| i * j)
            .sum();
        let p2_score = input
            .player2
            .iter()
            .rev()
            .zip(1..)
            .map(|(i, j)| i * j)
            .sum();

        std::cmp::max(p1_score, p2_score)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let (input, _) = recursive_combat(1, v[0].clone());

        let p1_score = input
            .player1
            .iter()
            .rev()
            .zip(1..)
            .map(|(i, j)| i * j)
            .sum();
        let p2_score = input
            .player2
            .iter()
            .rev()
            .zip(1..)
            .map(|(i, j)| i * j)
            .sum();

        std::cmp::max(p1_score, p2_score)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 306);

        assert_eq!(Solution::p2(&v), 291);
    }
}
