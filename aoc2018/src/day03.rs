use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct ElfBox {
    claim: u32,
    left : u32,
    top : u32,
    width : u32,
    height: u32,
}

impl ElfBox {
    fn collision(&self, other: &ElfBox) -> bool {
        self.left < other.left + other.width
            && self.left + self.width > other.left
            && self.top < other.top + other.height
            && self.top + self.height > other.top
    }
    fn collide_box(&self, other: &ElfBox) -> Option<ElfBox> {
        if self.collision(other) {
            use core::cmp::{min, max};
            let left = max(self.left, other.left);
            let width =  min(self.left + self.width, other.left + other.width) - left;
            let top = max(self.top, other.top);
            let height = min(self.top + self.height, other.top + other.height) - top;
            if width > 0 && height > 0 {
                Some(ElfBox { claim: 0, left, width, top, height })
            } else {
                None
            }

        } else {
            None
        }

    }
}
use std::io::BufRead;
use std::fs::File;
pub fn p1() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day3.txt")?);
    let mut hm = std::collections::HashMap::new();
    let mut v : Vec<ElfBox> = f.lines()
        .map(|s| {
            let s = s.unwrap();
            let s = s.as_ref();
            let b : ElfBox = serde_scan::scan!("#{} @ {},{}: {}x{}" <- s)
                .expect("Valid ElfBox");
            b
        }
        ).collect::<Vec<ElfBox>>();

    for b in v.iter() {
        for x in b.left..b.left + b.width {
            for y in b.top..b.top + b.height {
                *hm.entry((x,y)).or_insert(0) += 1;
            }
        }
    }

    println!("Inches: {}", hm.values().filter(|x| **x > 1).count());
    Ok(())
}

pub fn p2() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day3.txt")?);
    let mut hm = std::collections::HashMap::new();
    let mut v : Vec<ElfBox> = f.lines()
        .map(|s| {
            let s = s.unwrap();
            let s = s.as_ref();
            let b : ElfBox = serde_scan::scan!("#{} @ {},{}: {}x{}" <- s)
                .expect("Valid ElfBox");
            b
        }
        ).collect::<Vec<ElfBox>>();

    let mut seen :Vec<bool> = vec![false;v.len()];
    for b in v.iter() {
        for x in b.left..b.left + b.width {
            for y in b.top..b.top + b.height {
                if let Some(found) = hm.get(&(x,y)) {
                    seen[b.claim as usize - 1] = true;
                    seen[*found as usize - 1] = true;
                } else {
                    *hm.entry((x,y)).or_insert(b.claim);
                }
            }
        }
    }

    println!("Unclaimed: {:?}",
             seen.iter().enumerate()
             .filter(|(i, x)| !**x)
             .map(|(i, x)| i + 1)
             .collect::<Vec<usize>>());
    Ok(())
}
