use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufRead, BufReader, Read};

use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    inputs : Vec<(u64, ReactionId, ReactionNameId)>,
    output: (u64, ReactionId, ReactionNameId),    
}

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
struct ReactionId(usize);

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
struct ReactionNameId(usize);

fn how_much_ore(reactions: &Vec<Reaction>, extra: &mut Vec<u64>, ore_id: ReactionId, request: (u64, ReactionId)) -> u64 {
   
    let mut worklist = vec![request];

    let mut ore = 0;
  
    while let Some((c, o)) = worklist.pop() {
        //println!("{:?} {:?} {:?}", c, o, worklist);
        if o == ore_id {
            ore += c;
        } else {
            let r = &reactions[o.0];
            let avail = &mut extra[o.0];
            let required = if c > *avail {
                let r = c - *avail;
                *avail = 0;
                r
            } else {
                *avail -= c;
                0
            };
            
            let total = (required as f64 / r.output.0 as f64).ceil() as u64;
            //println!("{} {} {}", total, r.output.0, required);
            let new_extra = total * r.output.0 - required;
            
            if total > 0 {
                worklist.extend(r.inputs.iter().map(|(new_c, i, _)| {                    
                    (new_c * total, *i)
                }));
            }            
            if new_extra > 0 {
                extra[o.0] += new_extra;
            }
        }
    }
    ore
}
pub fn p1() -> IoResult<()> {
    return Ok(())
    // let s = std::fs::read_to_string("input/day14.txt")?;
    // let reactions =
    //     s.trim()
    //     .lines()
    //     .map(|s| {
    //         let reaction = s.split(" => ").collect::<Vec<_>>();
    //         let inputs : Vec<_> = reaction[0].split(", ")
    //             .map(|s| {
    //                 let mut items = s.split(' ');
    //                 (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
    //             }
    //             ).collect();
    //         let outputs : Vec<_> = reaction[1].split(", ")
    //             .map(|s| {
    //                  let mut items = s.split(' ');
    //                 (items.next().unwrap().parse::<u64>().unwrap(), items.next().unwrap().to_string())
    //             }
    //             ).collect();
    //         Reaction {
    //             inputs,
    //             output: outputs[0].clone()
    //         }
    //     })
    //     .collect::<Vec<_>>();
 
    // let mut extra :HashMap<String, u64> = HashMap::new();
    // println!("Part 1 {}", how_much_ore(&reactions, &mut extra, (1, "FUEL".to_string())));
    // Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day14.txt")?;

    let mut ids : HashMap<String, usize> = HashMap::new();
    let mut id = 0;

    let mut lookup = |name: &str|
    if let Some(old_id) = ids.get(name) {
        ReactionNameId(*old_id)
    } else {
        ids.insert(name.to_string(), id);
        let new = ReactionNameId(id);
        id += 1;
        new
    };
    
    let mut reactions =
        s.trim()
        .lines()
        .map(|s| {
            let reaction = s.split(" => ").collect::<Vec<_>>();
            let inputs : Vec<_> = reaction[0].split(", ")
                .map(|s| {
                    let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), ReactionId(0), lookup(items.next().unwrap()))
                }
                ).collect();
            let outputs : Vec<_> = reaction[1].split(", ")
                .map(|s| {
                     let mut items = s.split(' ');
                    (items.next().unwrap().parse::<u64>().unwrap(), ReactionId(0), lookup(items.next().unwrap()))
                }
                ).collect();
            Reaction {
                inputs,
                output: outputs[0].clone()
            }
        })
        .collect::<Vec<_>>();

    
    let ore_id = lookup("ORE");
    let fuel_id = lookup("FUEL");

    reactions.push(Reaction {
        inputs: vec![(1, ReactionId(0), ore_id)],
        output: (1, ReactionId(0), ore_id),
    });

    //println!("{:?}", ids);
    
    let ids : HashMap<ReactionNameId, ReactionId> = reactions.iter().enumerate().map(|(i, r)| (r.output.2, ReactionId(i))).collect();

    //println!("Reactions: {:?}", reactions);
    
    //println!("{:?}", ids);
    for r in &mut reactions {
        for i in &mut r.inputs {
            //println!("{:?}", i.2);
            i.1 = *ids.get(&i.2).unwrap();
        }
        r.output.1 = *ids.get(&r.output.2).unwrap();
    }

    let mut lo = std::u64::MIN;
    let mut hi = std::u64::MAX;
    while hi > lo {
        let mut extra :Vec<u64> = vec![0; reactions.len()];
        let mid = (hi - lo) / 2 + lo;
        let got = how_much_ore(&reactions, &mut extra, *ids.get(&ore_id).unwrap(), (mid, *ids.get(&fuel_id).unwrap()));
        use std::cmp::Ordering;
        match got.cmp(&1000000000000) {
            Ordering::Less => {
                lo = mid + 1;
            }
            Ordering::Greater => {
                hi = mid -1;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert!(true);
    }
}
