use crate::Day;
use cgmath::{Point2, Vector2};
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 17;
    type Input1 = HashMap<Point2<i32>, usize>;
    type Input2 = HashMap<Point2<i32>, usize>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    let cell = c.to_digit(10).unwrap() as usize;
                    (Point2::new(x as i32, -(y as i32)), cell)
                })
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(heat: &Self::Input1) -> Self::Sol1 {
        // 1121 Wrong
        // 1108 Too high
        // 1080 Too High
        #[derive(Clone, Debug)]
        struct Crucible {
            pos: Point2<i32>,
            dir: Vector2<i32>,
            cost: usize,
        }
        const UP: Vector2<i32> = Vector2::new(0, 1);
        const DOWN: Vector2<i32> = Vector2::new(0, -1);
        const LEFT: Vector2<i32> = Vector2::new(-1, 0);
        const RIGHT: Vector2<i32> = Vector2::new(1, 0);

        let max_x = heat.keys().max_by_key(|p| p.x).unwrap().x;
        let min_y = heat.keys().min_by_key(|p| p.y).unwrap().y;
        let goal = (max_x, min_y).into();

        assert!(heat.get(&goal).is_some());
        let mut crucibles = vec![
            Crucible {
                pos: (0, 0).into(),
                dir: UP,
                cost: 0,
            },
            Crucible {
                pos: (0, 0).into(),
                dir: LEFT,
                cost: 0,
            },
        ];
        let mut cost: HashMap<(Vector2<i32>, Point2<i32>), usize> = HashMap::default();
        let mh_dist = |p1: Point2<i32>, p2: Point2<i32>| (p1.x - p2.x).abs() + (p1.y - p2.y).abs();

        let mut best = None;

        while let Some(c) = crucibles.pop() {
            //println!("{:?}", c);
            let dirs = match c.dir {
                UP | DOWN => [LEFT, RIGHT],
                LEFT | RIGHT => [UP, DOWN],
                _ => panic!(),
            };
            use std::collections::hash_map::Entry;
            match cost.entry((c.dir, c.pos)) {
                Entry::Occupied(mut o) => {
                    let v = o.get_mut();
                    if *v <= c.cost {
                        continue;
                    }
                    *v = c.cost;
                }
                Entry::Vacant(v) => {
                    v.insert(c.cost);
                }
            }
            //println!("{} {:?} {:?}", mh_dist(c.pos, goal), goal, c.pos);
            if c.pos == goal {
                crucibles.retain(|old_c| {
                    old_c.cost + usize::try_from(mh_dist(old_c.pos, goal)).unwrap() < c.cost
                });
                println!("New Val {:?}", c.cost);
                best = Some(match best {
                    Some(v) => std::cmp::min(v, c.cost),
                    None => c.cost,
                });
                continue;
            }
            if best
                .as_ref()
                .map(|g| *g <= c.cost + usize::try_from(mh_dist(c.pos, goal)).unwrap())
                .unwrap_or(false)
            {
                continue;
            }
            for d in dirs {
                let mut c = c.clone();
                c.dir = d;
                for _ in 0..3 {
                    c.pos += c.dir;
                    if let Some(h) = heat.get(&c.pos) {
                        c.cost += *h;

                        let dist = usize::try_from(mh_dist(goal, c.pos)).unwrap();
                        let insert = crucibles.partition_point(|p| {
                            let p_dist = usize::try_from(mh_dist(goal, p.pos)).unwrap();
                            //(p_dist, p_dist + p.cost) >= (dist, dist + c.cost)
                            (p_dist, p_dist + p.cost) >= (dist, dist + c.cost)
                        });
                        crucibles.insert(insert, c.clone());
                    }
                }
            }
        }
        best.unwrap()
    }
    fn p2(heat: &Self::Input2) -> Self::Sol2 {
        //1224 Too high
        #[derive(Clone, Debug)]
        struct Crucible {
            pos: Point2<i32>,
            dir: Vector2<i32>,
            cost: usize,
        }
        const UP: Vector2<i32> = Vector2::new(0, 1);
        const DOWN: Vector2<i32> = Vector2::new(0, -1);
        const LEFT: Vector2<i32> = Vector2::new(-1, 0);
        const RIGHT: Vector2<i32> = Vector2::new(1, 0);

        let max_x = heat.keys().max_by_key(|p| p.x).unwrap().x;
        let min_y = heat.keys().min_by_key(|p| p.y).unwrap().y;
        let goal = (max_x, min_y).into();

        assert!(heat.get(&goal).is_some());
        let mut crucibles = vec![
            Crucible {
                pos: (0, 0).into(),
                dir: UP,
                cost: 0,
            },
            Crucible {
                pos: (0, 0).into(),
                dir: LEFT,
                cost: 0,
            },
        ];
        let mut cost: HashMap<(Vector2<i32>, Point2<i32>), usize> = HashMap::default();
        let mh_dist = |p1: Point2<i32>, p2: Point2<i32>| (p1.x - p2.x).abs() + (p1.y - p2.y).abs();

        let mut best = None;

        while let Some(c) = crucibles.pop() {
            //println!("{:?}", c);
            let dirs = match c.dir {
                UP | DOWN => [LEFT, RIGHT],
                LEFT | RIGHT => [UP, DOWN],
                _ => panic!(),
            };
            use std::collections::hash_map::Entry;
            match cost.entry((c.dir, c.pos)) {
                Entry::Occupied(mut o) => {
                    let v = o.get_mut();
                    if *v <= c.cost {
                        continue;
                    }
                    *v = c.cost;
                }
                Entry::Vacant(v) => {
                    v.insert(c.cost);
                }
            }
            //println!("{} {:?} {:?}", mh_dist(c.pos, goal), goal, c.pos);
            if c.pos == goal {
                crucibles.retain(|old_c| {
                    old_c.cost + usize::try_from(mh_dist(old_c.pos, goal)).unwrap() < c.cost
                });
                println!("New Val {:?}", c.cost);
                best = Some(match best {
                    Some(v) => std::cmp::min(v, c.cost),
                    None => c.cost,
                });
                continue;
            };
            if best
                .as_ref()
                .map(|g| *g <= c.cost + usize::try_from(mh_dist(c.pos, goal)).unwrap())
                .unwrap_or(false)
            {
                continue;
            }
            for d in dirs {
                let mut c = c.clone();
                c.dir = d;
                for i in 1..=10 {
                    c.pos += c.dir;
                    if let Some(h) = heat.get(&c.pos) {
                        c.cost += *h;
                        if i < 4 {
                            continue;
                        }
                        let dist = usize::try_from(mh_dist(goal, c.pos)).unwrap();
                        let insert = crucibles.partition_point(|p| {
                            let p_dist = usize::try_from(mh_dist(goal, p.pos)).unwrap();
                            //(p_dist, p_dist + p.cost) >= (dist, dist + c.cost)
                            (p_dist, p_dist + p.cost) >= (dist, dist + c.cost)
                        });
                        crucibles.insert(insert, c.clone());
                    }
                }
            }
        }
        best.unwrap()
    }
}

//Too slow to leave enabled in the tests
//crate::default_tests!(1076, 1219);
crate::string_tests!(
    [(
        foo_sol1,
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        102
    )],
    [
        (
            foo_sol2,
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
            94
        ),
        (
            foo2_sol2,
            "111111111111
999999999991
999999999991
999999999991
999999999991",
            71
        )
    ]
);
