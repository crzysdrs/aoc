use serde::Deserialize;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn mh_dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn convex_hull(pts: &[Point]) -> Vec<Point> {
    let mut hull = pts[0];
    let mut p = Vec::new();
    loop {
        p.push(hull);
        let mut endpoint = pts[0];
        for search in pts[1..].iter() {
            if endpoint == hull
                || orientation(&hull, &endpoint, &search) == Orient::CounterClockwise
            /* include collinear */
            {
                endpoint = *search;
            }
        }
        hull = endpoint;
        if endpoint == p[0] {
            break;
        }
    }
    p
}

fn inside_hull(pt: Point, pts: &[Point]) -> bool {
    pts.iter()
        .zip(pts[1..].iter())
        .all(|(p1, p2)| orientation(&p1, &p2, &pt) == Orient::Clockwise)
}

#[allow(unused)]
fn colinear_points(pts: &[Point]) -> Vec<Point> {
    let mut v = Vec::new();
    for (p1, p2) in pts.iter().zip(pts[1..].iter().chain(pts[0..].iter())) {
        for p in pts {
            if p != p1 && p != p2 && orientation(&p1, &p2, &p) == Orient::CoLinear {
                v.push(*p);
            }
        }
    }

    v.extend(pts);
    v.sort();
    v.dedup();
    v
}

#[derive(PartialEq, Debug)]
enum Orient {
    Clockwise,
    CounterClockwise,
    CoLinear,
}

fn orientation(p: &Point, q: &Point, r: &Point) -> Orient {
    let v = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    match v {
        0 => Orient::CoLinear,
        n if n > 0 => Orient::Clockwise,
        _ => Orient::CounterClockwise,
    }
}

#[derive(Debug)]
struct Dist {
    start: Point,
    next: Point,
}

pub fn p1() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day6.txt")?);

    let mut v = Vec::new();
    for l in f.lines() {
        let l = l.unwrap();
        let l = l.as_ref();
        let p: Point = serde_scan::scan!("{}, {}" <- l).unwrap();
        v.push(p)
    }
    v.sort_by_key(|p| p.x);
    let hull = convex_hull(&v);
    let mut new_hull = hull.clone();
    for (p1, p2) in hull.iter().zip(hull[1..].iter()) {
        new_hull.push(Point { x: p1.x, y: p2.y });
        new_hull.push(Point { x: p2.x, y: p1.y });
    }
    let hull = convex_hull(&new_hull);
    //let mut exclude = colinear_points(&hull);
    println!("Hull {} {:?}", hull.len(), hull);
    let mut exclude = std::collections::HashSet::<Point>::from_iter(hull.iter().cloned());
    let mut map: std::collections::HashMap<Point, Option<Point>> = std::collections::HashMap::new();

    let mut next: Vec<Dist> = v
        .iter()
        .map(|p| {
            //map.insert(*p, Some(*p));
            Dist {
                start: *p,
                next: *p,
            }
        })
        .collect();

    for dist in 1.. {
        let mut next2 = Vec::new();
        for p in next.drain(..) {
            let mut updated = !map.contains_key(&p.next);
            let _entry = map
                .entry(p.next)
                .and_modify(|e| {
                    *e = match *e {
                        None => None,
                        Some(target) => {
                            if p.start == target {
                                *e
                            } else {
                                match p.start.mh_dist(&p.next).cmp(&target.mh_dist(&p.next)) {
                                    Ordering::Equal => None,
                                    Ordering::Greater => *e,
                                    Ordering::Less => {
                                        updated = true;
                                        Some(p.start)
                                    }
                                }
                            }
                        }
                    }
                })
                .or_insert(Some(p.start));

            if updated {
                if !inside_hull(p.next, &hull) {
                    if !exclude.contains(&p.start) {
                        println!("Exclude {:?} {:?}", p.start, p.next);
                        exclude.insert(p.start);
                    }
                }

                for x in &[-1, 1] {
                    let next = Point {
                        x: p.next.x + x,
                        y: p.next.y,
                    };
                    next2.push(Dist {
                        start: p.start,
                        next,
                    })
                }
                for y in &[-1, 1] {
                    let next = Point {
                        x: p.next.x,
                        y: p.next.y + y,
                    };
                    next2.push(Dist {
                        start: p.start,
                        next,
                    })
                }
            }
        }
        println!("Iteration {}, Next: {}", dist, next2.len());
        println!(
            "{}/{}",
            next2.iter().filter(|p| exclude.contains(&p.start)).count(),
            next2.len()
        );
        if next2.iter().all(|p| exclude.contains(&p.start)) {
            break;
        }
        std::mem::swap(&mut next, &mut next2);
    }
    println!("Excluded {} {:?}", exclude.len(), exclude);

    let mut counts = std::collections::HashMap::new();
    map.iter().for_each(|(_k, v)| {
        if let Some(v) = v {
            counts.entry(v).and_modify(|e| *e += 1).or_insert(1);
        }
    });

    println!(
        "{:?}",
        counts
            .iter()
            .filter(|(k, _v)| !exclude.contains(k))
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        counts
            .iter()
            .filter(|(k, _v)| !exclude.contains(k))
            .max_by_key(|&(_k, v)| v)
            .unwrap()
    );

    Ok(())
}
