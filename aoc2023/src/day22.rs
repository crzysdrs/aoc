use crate::Day;
use cgmath::Point3;
use rayon::prelude::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Brick {
    start: Point3<i32>,
    end: Point3<i32>,
}

fn dim_intersect(a: std::ops::RangeInclusive<i32>, b: std::ops::RangeInclusive<i32>) -> bool {
    let max_a = std::cmp::max(a.start(), a.end());
    let min_a = std::cmp::min(a.start(), a.end());

    let max_b = std::cmp::max(b.start(), b.end());
    let min_b = std::cmp::min(b.start(), b.end());

    !((max_a < min_b) || (max_b < min_a))
}

impl Brick {
    fn down(&self) -> Option<Brick> {
        if self.start.z == 1 || self.end.z == 1 {
            return None;
        }
        Some(Brick {
            start: Point3 {
                z: self.start.z - 1,
                ..self.start
            },
            end: Point3 {
                z: self.end.z - 1,
                ..self.end
            },
        })
    }
    fn points(&self) -> HashSet<Point3<i32>> {
        let min_y = std::cmp::min(self.start.y, self.end.y);
        let max_y = std::cmp::max(self.start.y, self.end.y);

        let min_x = std::cmp::min(self.start.x, self.end.x);
        let max_x = std::cmp::max(self.start.x, self.end.x);

        let min_z = std::cmp::min(self.start.z, self.end.z);
        let max_z = std::cmp::max(self.start.z, self.end.z);

        let mut v = HashSet::new();

        // println!(
        //     "{} {} {} {} {} {}",
        //     min_x, max_x, min_y, max_y, min_z, max_z
        // );
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    v.insert(Point3::new(x, y, z));
                }
            }
        }
        //println!("{:?} {:?}: {:?}", self.start, self.end, v);
        v
    }

    fn xy_intersect(&self, b: &Brick) -> bool {
        dim_intersect(self.start.x..=self.end.x, b.start.x..=b.end.x)
            && dim_intersect(self.start.y..=self.end.y, b.start.y..=b.end.y)
    }
    fn intersect(&self, b: &Brick) -> bool {
        dim_intersect(self.start.x..=self.end.x, b.start.x..=b.end.x)
            && dim_intersect(self.start.y..=self.end.y, b.start.y..=b.end.y)
            && dim_intersect(self.start.z..=self.end.z, b.start.z..=b.end.z)

        // if dim_intersect(self.start.x..=self.end.x, b.start.x..=b.end.x)
        //     || dim_intersect(self.start.y..=self.end.y, b.start.x..=b.end.x)
        //     || dim_intersect(self.start.z..=self.end.z, b.start.z..=b.end.z)
        // {
        //     self.points_intersect(b)
        // } else {
        //     false
        // }
    }
    fn points_intersect(&self, b: &Brick) -> bool {
        self.points().intersection(&b.points()).count() > 0
    }
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 22;
    type Input1 = Vec<Brick>;
    type Input2 = Vec<Brick>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|l| {
                let (left, right) = l.split_once('~').unwrap();
                let pt = |p: &str| {
                    let mut p = p.split(',');
                    let x = p.next()?.parse().unwrap();
                    let y = p.next()?.parse().unwrap();
                    let z = p.next()?.parse().unwrap();
                    Some(Point3::new(x, y, z))
                };

                Brick {
                    start: pt(left).unwrap(),
                    end: pt(right).unwrap(),
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        fn settle(bricks: &mut [Brick]) -> bool {
            let mut new_bricks = vec![];
            let mut moved = false;
            for (i, b) in bricks.iter().enumerate() {
                let mut min_z = 1;
                let cur_min_z = std::cmp::min(b.start.z, b.end.z);
                for (j, b2) in bricks.iter().enumerate().filter(|(j, _)| i != *j) {
                    let max_z = std::cmp::max(b2.start.z, b2.end.z);

                    if max_z < cur_min_z && b.xy_intersect(b2) {
                        //println!("{:?} {:?}", max_z, b);
                        min_z = std::cmp::max(min_z, max_z + 1);
                    };
                }
                let offset = if b.start.z == cur_min_z {
                    b.start.z - min_z
                } else {
                    b.end.z - min_z
                };
                //println!("{} {:?}", i, offset);
                if offset > 0 {
                    moved = true;
                }
                new_bricks.push(Brick {
                    start: Point3 {
                        z: b.start.z - offset,
                        ..b.start
                    },
                    end: Point3 {
                        z: b.end.z - offset,
                        ..b.end
                    },
                })
            }
            if moved {
                bricks.clone_from_slice(&new_bricks);
            }
            moved
        }
        let mut bricks = v.clone();
        //println!("{:?}", bricks);
        'bricks: loop {
            let moved = settle(&mut bricks);
            if !moved {
                break 'bricks;
            }
        }
        // all bricks in place
        //println!("{:?}", bricks);

        let mut count = 0;
        for (i, b) in bricks.iter().enumerate() {
            let mut new = bricks.clone();
            new.retain(|b2| *b2 != *b);
            let moved = settle(&mut new);
            println!("{}: {:?}", i, moved);
            count += usize::from(!moved);
        }
        count
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        fn settle(bricks: &mut [Brick]) -> usize {
            let mut new_bricks = vec![];
            let mut moved = 0;
            for (i, b) in bricks.iter().enumerate() {
                let mut min_z = 1;
                let cur_min_z = std::cmp::min(b.start.z, b.end.z);
                for (j, b2) in bricks.iter().enumerate().filter(|(j, _)| i != *j) {
                    let max_z = std::cmp::max(b2.start.z, b2.end.z);

                    if max_z < cur_min_z && b.xy_intersect(b2) {
                        //println!("{:?} {:?}", max_z, b);
                        min_z = std::cmp::max(min_z, max_z + 1);
                    };
                }
                let offset = if b.start.z == cur_min_z {
                    b.start.z - min_z
                } else {
                    b.end.z - min_z
                };
                //println!("{} {:?}", i, offset);
                if offset > 0 {
                    moved += 1;
                }
                new_bricks.push(Brick {
                    start: Point3 {
                        z: b.start.z - offset,
                        ..b.start
                    },
                    end: Point3 {
                        z: b.end.z - offset,
                        ..b.end
                    },
                })
            }
            if moved != 0 {
                bricks.clone_from_slice(&new_bricks);
            }
            moved
        }
        let mut bricks = v.clone();
        //println!("{:?}", bricks);
        'bricks: loop {
            let moved = settle(&mut bricks);
            if moved == 0 {
                break 'bricks;
            }
        }
        bricks
            .par_iter()
            .enumerate()
            .map(|(i, b)| {
                let mut new = bricks.clone();
                new.retain(|b2| *b2 != *b);
                let old = new.clone();
                loop {
                    let moved = settle(&mut new);
                    if moved == 0 {
                        break;
                    }
                }
                let moved = new
                    .iter()
                    .zip(old.iter())
                    .filter(|(b1, b2)| b1 != b2)
                    .count();
                println!("{}: {:?}", i, moved);
                moved
            })
            .sum()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        5
    )],
    [(
        foo_sol2,
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        7
    )]
);
