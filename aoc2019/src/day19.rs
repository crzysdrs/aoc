use crate::intcode::IntCodeMachine;
use std::io::Result as IoResult;
use cgmath::{Point2, Vector2};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::{HashSet, HashMap};

use std::convert::TryFrom;
use itertools::Itertools;
#[derive(Debug, FromPrimitive, ToPrimitive, PartialEq, Eq, Copy, Clone)]
enum DroneState {
    Stationary = 0,
    Pulled = 1
}

#[allow(unused)]
fn draw(grid: &HashMap<Point2<isize>, DroneState>)
{
    let min_x = grid.keys().map(|p| p.x).min().unwrap();
    let min_y = grid.keys().map(|p| p.y).min().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();

    println!("Image Width: {}x{}", max_y - min_y, max_x - min_x);
    
    for y in (min_y..=max_y) {
        print!("{} ", y);
        for x in min_x..=max_x {
            let p = grid.get(&Point2::new(x, y));
            print!(
                "{}",
                match p {
                    None => " ",
                    Some(DroneState::Stationary) => ".",
                    Some(DroneState::Pulled) => "X",
                }
            )
        }
        println!();
    }
}

#[allow(unused)]
fn draw_range(grid: &HashMap<Point2<isize>, DroneState>, x_range : std::ops::Range<isize>,  y_range: std::ops::Range<isize>)
{
    
    for y in y_range {
        print!("{} ", y);
        for x in x_range.clone() {
            let p = grid.get(&Point2::new(x as isize, y as isize));
            print!(
                "{}",
                match p {
                    None => " ",
                    Some(DroneState::Stationary) => ".",
                    Some(DroneState::Pulled) => "X",
                }
            )
        }
        println!();
    }
}


pub fn p1() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day19.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    

    let mut grid = HashMap::new();
    for x in 0..50{
        for y in 0..50 {
            let mut machine = IntCodeMachine::new(codes.clone(), vec![]);
            machine.feed_input(x);
            machine.feed_input(y);
            machine.run();
            //println!("{} {}", x,y);
            grid.insert(Point2 {x, y}, DroneState::from_isize(machine.next_output().expect("Output")).expect("Valid DroneState"));
        }
    }

    draw(&grid);
    
    println!("Part 1 {}", grid.values().filter(|x| **x == DroneState::Pulled).count());
    // let grid = machine.output().iter()
    //     .map(|x| char::from(*x as u8))
    //     .collect::<String>()
    //     .split("\n")
    //     .enumerate()
    //     .flat_map(
    //         move |(y, l)|
    //         l.chars().enumerate().map(move |(x, c)| (Point2::new(x as i32, y as i32), c))
    //     )
    //     .collect::<HashMap<_,char>>();

    Ok(())
}

pub fn p2() -> IoResult<()> {
    let codes = std::fs::read_to_string("input/day19.txt")?
        .trim()
        .split(",")
        .map(|x| x.parse::<isize>().expect("Valid usize"))
        .collect::<Vec<_>>();
    
    let mut ranges = vec![];
    let mut last_y = 0;
    let desired = 100;
    //let mut old_y = 1028;
    let mut old_y = 0;
    let mut grid = HashMap::new();
    for x in 5.. {
    //for x in (1035 - desired - 10).. {
        let mut beam_state = DroneState::Stationary;
        let mut y = std::cmp::min(0, old_y - 5);
        let x = x as isize;
        while beam_state == DroneState::Stationary {
            let mut machine = IntCodeMachine::new(codes.clone(), vec![x, y]);
            machine.run();
            beam_state = DroneState::from_isize(machine.next_output().expect("Output")).expect("Valid DroneState");
            grid.insert(Point2 {x, y}, beam_state);
            y += 1;
        }
        let top = (x, y - 1);
        old_y = y;

        while beam_state == DroneState::Pulled {
            let mut machine = IntCodeMachine::new(codes.clone(), vec![x, y]);
            machine.run();
            beam_state = DroneState::from_isize(machine.next_output().expect("Output")).expect("Valid DroneState");
            grid.insert(Point2 {x, y}, beam_state);
            y += 1;
        }
        let bottom = (x, y - 1);

        ranges.push((x, top.1..bottom.1));
        if ranges.len() >= desired {
            let (cur_x, source) = ranges.iter().rev().take(1).next().unwrap();
            let (old_x, target) = ranges.iter().rev().skip(desired - 1).take(1).next().unwrap();
            
            if target.contains(&source.start) && usize::try_from(target.end - source.start).unwrap() >= desired  {
                //println!("Found {:?} {:?}", target, ranges.last().unwrap());
                println!("Result {}", old_x * 10000 + source.start);
                //println!("Source {:?} Target {:?}", source, target);
                //draw_range(&grid, (*old_x)..(cur_x + 1), source.start..target.end);
                break;
            }
        }
    }

    //draw(&grid);

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
