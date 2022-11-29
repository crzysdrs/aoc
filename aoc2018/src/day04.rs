use serde::{Deserialize};
//#[derive(Deserialize, Debug)]

use std::io::BufRead;
use std::fs::File;

use chrono::prelude::*;

#[derive(Debug)]
enum Content {
    WakeUp,
    Sleep,
    Start(u32),
}
#[derive(Debug)]
struct Msg {
    time :DateTime<Utc>,
    content : Content
}
pub fn p1() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day4.txt")?);
    let mut v = Vec::new();
    use regex::Regex;
    let re = Regex::new(r".*Guard #([0-9]+) .*").unwrap();

    for s in f.lines() {
        let s = s.unwrap();
        let s = s.as_ref();
        let (year, mon, day, hour, min, msg) : (i32, u32, u32, u32, u32, &str)
            = serde_scan::scan!("[{}-{}-{} {}:{}] {}" <- s).unwrap();

        let c = match msg {
            "Guard" => {
                let m = re.captures(s).unwrap();
                Content::Start(m[1].parse::<u32>().unwrap())
            },
            "falls" => {Content::Sleep},
            "wakes" => {Content::WakeUp},
            _ => panic!(),
        };

        let d = Utc.ymd(year, mon, day).and_hms_opt(hour, min, 0).unwrap();
        v.push(Msg { time: d, content: c } );
    }
    v.sort_by_key(|x| x.time);

    let mut hm = std::collections::HashMap::new();

    let mut cur_guard = None;
    let mut sleep = None;

    for m in v {
        match m {
            Msg {content: Content::Start(n), .. } => {
                cur_guard = Some(n);
            },
            Msg {content : Content::Sleep, time: t } => {
                sleep = Some(t);
            },
            Msg {content : Content::WakeUp, time: t } => {
                match (cur_guard, sleep) {
                    (Some(g), Some(s)) => {
                        let e = hm.entry(g).or_insert((0, vec![0; 60]));
                        e.0 += (t - s).num_minutes() as u32;
                        for m in s.minute()..s.minute() + (t-s).num_minutes() as u32{
                            e.1[m as usize % 60] += 1;
                        }
                    },
                    _ => {panic!("Incorrect msg ordering")}
                }
                sleep = None;
            },
        }
    }
    let m = hm.iter().max_by_key(|&(g, (t, v))| t).unwrap();
    let max_min = (m.1).1.iter().enumerate().max_by_key(|&(i, m)| m).map(|(i, m)| i).unwrap();
    let g = m.0;

    println!("{:?}", max_min as u32 * *g);
    //part 2
    let m = hm.iter().max_by_key(|&(g, (t, v))| v.iter().max()).unwrap();
    let max_min = (m.1).1.iter().enumerate().max_by_key(|&(i, m)| m).map(|(i, m)| i).unwrap();
    println!("{:?}", *m.0  * max_min as u32);
    Ok(())
}
