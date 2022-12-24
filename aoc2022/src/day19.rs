use crate::Day;
use rayon::prelude::*;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
pub struct Robot {
    typ: RobotType,
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
pub struct Blueprint {
    num: usize,
    robots: [Robot; 4],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Inventory {
    minutes: u32,
    ore: u32,
    obsidian: u32,
    clay: u32,
    geode: u32,
    robot_clay: u32,
    robot_ore: u32,
    robot_obs: u32,
    robot_geode: u32,
}

impl Inventory {
    fn generate_time(&mut self, min: u32) {
        self.ore += self.robot_ore * min;
        self.clay += self.robot_clay * min;
        self.obsidian += self.robot_obs * min;
        self.geode += self.robot_geode * min;
        //println!("{:?} {:?}", self.minutes, min);
        self.minutes -= min;
    }
    fn when_can_build(&self, bp: &Blueprint, typ: RobotType) -> Option<u32> {
        bp.robots
            .iter()
            .find(|x| x.typ == typ)
            .map(|r| {
                let compute = |have: u32, gen: u32, need: u32| {
                    if need == 0 {
                        Some(0)
                    } else if gen > 0 {
                        let rem = need.saturating_sub(have) % gen;
                        Some((need.saturating_sub(have) / gen) + if rem > 0 { 1 } else { 0 })
                    } else {
                        None
                    }
                };
                let ore = compute(self.ore, self.robot_ore, r.ore);
                let clay = compute(self.clay, self.robot_clay, r.clay);
                let obs = compute(self.obsidian, self.robot_obs, r.obsidian);

                //println!("{:?}", (ore, clay, obs, geode));
                if let (Some(ore), Some(clay), Some(obs)) = (ore, clay, obs) {
                    Some(std::cmp::max(ore, std::cmp::max(clay, obs)))
                } else {
                    None
                }
            })
            .flatten()
            .filter(|t| *t < self.minutes)
    }
    fn do_build(&mut self, bp: &Blueprint, typ: RobotType) {
        let robot = bp.robots.iter().find(|x| x.typ == typ).unwrap();
        //println!("{:?}", typ);
        self.ore -= robot.ore;
        self.obsidian -= robot.obsidian;
        self.clay -= robot.clay;
        match typ {
            RobotType::Ore => self.robot_ore += 1,
            RobotType::Clay => self.robot_clay += 1,
            RobotType::Obsidian => self.robot_obs += 1,
            RobotType::Geode => self.robot_geode += 1,
        }
    }
}

fn run<'a>(
    bps: impl ParallelIterator<Item = &'a Blueprint>,
    minutes: u32,
) -> impl ParallelIterator<Item = (&'a Blueprint, u32)> {
    bps.map(move |bp| {
        let mut best = None;
        let invent = Inventory {
            minutes,
            ore: 0,
            obsidian: 0,
            clay: 0,
            geode: 0,
            robot_clay: 0,
            robot_ore: 1,
            robot_obs: 0,
            robot_geode: 0,
        };
        let mut worklist = vec![invent];
        println!("{:?}", bp.num);

        let mut max_ore = 0;
        let mut max_clay = 0;
        let mut max_obs = 0;
        bp.robots.iter().for_each(|r| {
            max_ore = std::cmp::max(max_ore, r.ore);
            max_clay = std::cmp::max(max_clay, r.clay);
            max_obs = std::cmp::max(max_obs, r.obsidian);
        });
        while let Some(mut item) = worklist.pop() {
            if item.minutes == 0 {
                let geodes = item.geode;
                if let Some(b) = &mut best {
                    if *b < geodes {
                        *b = geodes;
                    }
                } else {
                    best = Some(geodes);
                }
                continue;
            }
            let all_robots = [
                RobotType::Ore,
                RobotType::Clay,
                RobotType::Obsidian,
                RobotType::Geode,
            ];

            let mut one_solution = false;
            all_robots
                .iter()
                .map(|r| item.when_can_build(bp, *r).map(|t| (r, t)))
                .flatten()
                .filter(|(robot, _)| match robot {
                    RobotType::Clay => max_clay >= item.robot_clay,
                    RobotType::Ore => max_ore >= item.robot_ore,
                    RobotType::Obsidian => max_obs >= item.robot_obs,
                    RobotType::Geode => true,
                })
                .for_each(|(r, t)| {
                    let mut item = item.clone();
                    one_solution = true;
                    item.generate_time(t + 1);
                    item.do_build(bp, *r);
                    worklist.push(item);
                });

            if !one_solution {
                item.generate_time(item.minutes);
                worklist.push(item);
            }
        }

        (bp, best.unwrap())
    })
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 19;
    type Input1 = Vec<Blueprint>;
    type Input2 = Vec<Blueprint>;
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input1(s: &str) -> Self::Input1 {
        let re = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

        s.lines()
            .map(|l| {
                let captures = re.captures(l).unwrap();
                Blueprint {
                    num: captures[1].parse().unwrap(),
                    robots: [
                        Robot {
                            typ: RobotType::Ore,
                            ore: captures[2].parse().unwrap(),
                            obsidian: 0,
                            clay: 0,
                        },
                        Robot {
                            typ: RobotType::Clay,
                            ore: captures[3].parse().unwrap(),
                            clay: 0,
                            obsidian: 0,
                        },
                        Robot {
                            typ: RobotType::Obsidian,
                            ore: captures[4].parse().unwrap(),
                            clay: captures[5].parse().unwrap(),
                            obsidian: 0,
                        },
                        Robot {
                            typ: RobotType::Geode,
                            ore: captures[6].parse().unwrap(),
                            clay: 0,
                            obsidian: captures[7].parse().unwrap(),
                        },
                    ],
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        run(v.par_iter(), 24)
            .map(|(bp, geodes)| bp.num as u32 * geodes)
            .sum::<u32>()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        run(v.par_iter().take(3), 32)
            .inspect(|(b, g)| println!("{:?}", (b.num, g)))
            .map(|(_b, g)| g)
            .product::<u32>()
    }
}

crate::default_tests!(1719, 19530);
crate::path_tests!(
    [(t1, "test/day19.txt", 33)],
    [(t2, "test/day19.txt", 62 * 56)]
);
