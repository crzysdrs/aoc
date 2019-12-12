use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};
use std::cmp::Ordering;
use std::fmt;
use std::convert::TryFrom;

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

impl Moon {
    fn tick(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }

    fn potential_energy(&self) -> u32 {
        u32::try_from(self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()).unwrap()
    }
    fn kinetic_energy(&self) -> u32 {
        u32::try_from(self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs()).unwrap()
    }

    fn state_x(&self) -> (i32, i32) {
        (self.pos.0, self.vel.0)
    }
    fn state_y(&self) -> (i32, i32) {
        (self.pos.1, self.vel.1)
    }
    fn state_z(&self) -> (i32, i32){
        (self.pos.2, self.vel.2)
    }

}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
               self.pos.0,
               self.pos.1,
               self.pos.2,
               self.vel.0,
               self.vel.1,
               self.vel.2
        )
    }
}

fn sim_moons(moons: &mut [Moon]) {
    let gravity = moons.iter().map(|m1| {
        moons.iter().flat_map(|m2| if m1 == m2 {
            None
        } else {
            Some(m2)
        }).map(|m2| {
            (
                m1.pos.0.cmp(&m2.pos.0),                    
                m1.pos.1.cmp(&m2.pos.1),
                m1.pos.2.cmp(&m2.pos.2),
            )
        }).collect::<Vec<_>>()
    })
        .collect::<Vec<_>>(); 
    
    fn apply_grav(vel : &mut i32, order: Ordering) {
        match order {
            Ordering::Less => {
                *vel += 1;
            },
            Ordering::Greater => {
                *vel -= 1;
            },
            _ => {},
        }
    }
    gravity.iter().enumerate().for_each(|(i, grav)| {
        for g in grav {
            apply_grav(&mut moons[i].vel.0, g.0);
            apply_grav(&mut moons[i].vel.1, g.1);
            apply_grav(&mut moons[i].vel.2, g.2);
        }
    });
    
    moons.iter_mut().for_each(|m| {
        m.tick();
    });
}

fn read_input(s: &str) -> Vec<Moon> {
    s.trim().lines().map(
        |s| {
            let v : Vec<_> = s.chars().skip(1).take(s.len() - 2).collect::<String>().split(", ").flat_map(|s| s.split('=').skip(1)).map(|s| s.parse::<i32>().unwrap()).collect();
            Moon {vel: (0,0,0), pos: (v[0], v[1], v[2])}
        }
    ).collect()
}
pub fn p1() -> IoResult<()> {
    let mut moons = read_input(&std::fs::read_to_string("input/day12.txt")?);
    for time in  1..=1000 {
        sim_moons(&mut moons);        
    }

    println!("Part 1: {}", moons.iter().map(|m| m.kinetic_energy() * m.potential_energy()).sum::<u32>());
    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a : usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
pub fn p2() -> IoResult<()> {
    let mut moons = read_input(&std::fs::read_to_string("input/day12.txt")?);

    let mut moon_state = (0..3).map(|_| std::collections::HashMap::new()).collect::<Vec<_>>();

    let mut repeat : Vec<Option<(usize, usize)>> = vec![None;3];
    
    for time in  0.. {
        if repeat.iter().flat_map(|x| x).count() == 3 {
            break;
        }
        moon_state.iter_mut().enumerate().for_each(
            |(i, state)| {
                if let None = repeat[i] {
                    let search = moons.iter().map(|m| match i {
                        0 => m.state_x(),
                        1 => m.state_y(),
                        2 => m.state_z(),
                        _ => panic!("Invalid state"),
                    }).collect::<Vec<_>>();
                    if let Some(t) = state.get(&search) {
                        //println!("Repeat {} {} {}", i, time, t);
                        repeat[i] = Some((time, *t));
                    }
                    state.entry(search).or_insert(time);
                }
            }
        );
        sim_moons(&mut moons);        
    }

    println!("Part 2: {}", repeat.iter().map(|x| x.unwrap().0).fold(1, |a, b| lcm(a,b)));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(false);
    }
}
