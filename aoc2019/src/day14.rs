use std::collections::HashMap;
use std::io::Result as IoResult;

#[derive(Debug)]
struct Reaction {
    inputs: Vec<(u64, String)>,
    output: (u64, String),
}

fn how_much_ore(
    reactions: &[Reaction],
    extra: &mut HashMap<String, u64>,
    request: (u64, String),
) -> u64 {
    let search: HashMap<_, _> = reactions
        .iter()
        .map(|r| (r.output.1.to_string(), r))
        .collect();

    let mut worklist = vec![request];

    let mut ore = 0;

    while let Some((c, o)) = worklist.pop() {
        //println!("{} {} {:?}", c, o, worklist);
        if o == "ORE" {
            ore += c;
        } else {
            let r = search.get(&o).unwrap();
            let avail = extra.entry(o.to_string()).or_insert(0);
            let required = if c > *avail {
                let r = c - *avail;
                *avail = 0;
                r
            } else {
                *avail -= c;
                0
            };

            let total = (required as f64 / r.output.0 as f64).ceil() as u64;
            let new_extra = total * r.output.0 - required;

            if total > 0 {
                worklist.extend(
                    r.inputs
                        .iter()
                        .map(|(new_c, i)| (new_c * total, i.to_string())),
                );
            }
            if new_extra > 0 {
                *extra.entry(o.to_string()).or_insert(0) += new_extra;
            }
        }
    }
    ore
}
pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day14.txt")?;
    let reactions = s
        .trim()
        .lines()
        .map(|s| {
            let reaction = s.split(" => ").collect::<Vec<_>>();
            let inputs: Vec<_> = reaction[0]
                .split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (
                        items.next().unwrap().parse::<u64>().unwrap(),
                        items.next().unwrap().to_string(),
                    )
                })
                .collect();
            let outputs: Vec<_> = reaction[1]
                .split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (
                        items.next().unwrap().parse::<u64>().unwrap(),
                        items.next().unwrap().to_string(),
                    )
                })
                .collect();
            Reaction {
                inputs,
                output: outputs[0].clone(),
            }
        })
        .collect::<Vec<_>>();

    let mut extra: HashMap<String, u64> = HashMap::new();
    println!(
        "Part 1 {}",
        how_much_ore(&reactions, &mut extra, (1, "FUEL".to_string()))
    );
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day14.txt")?;
    let reactions = s
        .trim()
        .lines()
        .map(|s| {
            let reaction = s.split(" => ").collect::<Vec<_>>();
            let inputs: Vec<_> = reaction[0]
                .split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (
                        items.next().unwrap().parse::<u64>().unwrap(),
                        items.next().unwrap().to_string(),
                    )
                })
                .collect();
            let outputs: Vec<_> = reaction[1]
                .split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (
                        items.next().unwrap().parse::<u64>().unwrap(),
                        items.next().unwrap().to_string(),
                    )
                })
                .collect();
            Reaction {
                inputs,
                output: outputs[0].clone(),
            }
        })
        .collect::<Vec<_>>();

    let mut lo = u64::MIN;
    let mut hi = 9999999999;
    while hi > lo {
        let mut extra = HashMap::new();
        let mid = (hi - lo) / 2 + lo;
        let got = how_much_ore(&reactions, &mut extra, (mid, "FUEL".to_string()));
        use std::cmp::Ordering;
        match got.cmp(&1000000000000) {
            Ordering::Less => {
                lo = mid + 1;
            }
            Ordering::Greater => {
                hi = mid - 1;
            }
            Ordering::Equal => {
                lo = mid;
                hi = mid;
            }
        }
    }

    println!("Part 2 {}", lo);
    Ok(())
}
