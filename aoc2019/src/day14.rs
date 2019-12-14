use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    inputs : Vec<(u64, String)>,
    output: (u64, String),
}


fn how_much_ore(reactions: &Vec<Reaction>, extra: &mut HashMap<String, u64>, request: (u64, String)) -> u64 {
   let search : HashMap<_, _>= reactions.iter().map(
        |r| (r.output.1.to_string(), r)
    ).collect();

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
                worklist.extend(r.inputs.iter().map(|(new_c, i)| {                    
                    (new_c * total, i.to_string())
                }));
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
    let reactions =
        s.trim()
        .lines()
        .map(|s| {
            let reaction = s.split(" => ").collect::<Vec<_>>();
            let inputs : Vec<_> = reaction[0].split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
                }
                ).collect();
            let outputs : Vec<_> = reaction[1].split(", ")
                .map(|s| {
                     let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
                }
                ).collect();
            Reaction {
                inputs,
                output: outputs[0].clone()
            }
        })
        .collect::<Vec<_>>();
 
    let mut extra :HashMap<String, u64> = HashMap::new();
    println!("Part 1 {}", how_much_ore(&reactions, &mut extra, (1, "FUEL".to_string())));
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day14.txt")?;
    let reactions =
        s.trim()
        .lines()
        .map(|s| {
            let reaction = s.split(" => ").collect::<Vec<_>>();
            let inputs : Vec<_> = reaction[0].split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
                }
                ).collect();
            let outputs : Vec<_> = reaction[1].split(", ")
                .map(|s| {
                     let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
                }
                ).collect();
            Reaction {
                inputs,
                output: outputs[0].clone()
            }
        })
        .collect::<Vec<_>>();
 
    let mut extra :HashMap<String, u64> = HashMap::new();
    let orig_ore = 1000000000000;
    let mut avail_ore = orig_ore;
    let mut fuel = 0;
    loop {
        let required = how_much_ore(&reactions, &mut extra, (1, "FUEL".to_string()));
        if required <= avail_ore {
            avail_ore -= required;
            fuel += 1;
            if fuel & 0xFFFF == 0 {
                println!("{}", avail_ore);
            }
            if extra.values().all(|x| *x == 0) {
                let spent_ore = orig_ore - avail_ore;
                fuel += fuel * avail_ore / spent_ore;
                break;
            }
        } else {
            break;
        }
    }
    println!("Part 2 {}", fuel);
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
