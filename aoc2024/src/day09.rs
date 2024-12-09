use crate::Day;
use itertools::Itertools;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone)]
pub struct DiskMap {
    id: usize,
    files: usize,
    free_space: usize,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9;
    type Input1 = Vec<DiskMap>;
    type Input2 = Vec<DiskMap>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let digits = s
            .lines()
            .flat_map(|s| s.chars().map(|v| v.to_digit(10).unwrap() as usize))
            .chain(std::iter::repeat(0).take(2))
            .into_iter();

        digits
            .tuples()
            .enumerate()
            .map(|(id, (files, free))| {
                println!("{} {} {}", id, files, free);
                DiskMap {
                    id,
                    files,
                    free_space: free,
                }
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        //println!("{:?}", v);

        let mut map = (*v).clone();
        let mut disk = vec![];

        let mut map_iter = map.iter_mut();

        let mut front = None;
        let mut back: Option<&mut DiskMap> = None;
        let mut free = 0;
        loop {
            println!("{:?} {:?} {:?}", front, back, free);
            if front.is_none() {
                front = map_iter.next();
            }
            if front.is_none() && back.is_none() {
                break;
            } else if front.is_some() {
                /* continue */
            } else if let Some(b) = &mut back {
                println!("Finalize back");
                disk.extend(std::iter::repeat(b.id).take(b.files));
                back = None;
            }

            if let Some(f) = &mut front {
                println!("Place {} {}", f.id, f.files);
                disk.extend(std::iter::repeat(f.id).take(f.files));
                free = f.free_space;
                front = None;
            }

            while free > 0 {
                if back.is_none() && free > 0 {
                    back = map_iter.next_back();
                }
                if back.is_none() {
                    break;
                }
                if let Some(b) = &mut back {
                    let take = std::cmp::min(b.files, free);
                    println!("Fill {} {}/{}", b.id, b.files, free);
                    disk.extend(std::iter::repeat(b.id).take(take));
                    free -= take;
                    b.files -= take;
                    if b.files == 0 {
                        back = None;
                    }
                }
            }
        }
        //println!("{:?} {:?} {:?}", free, front, back);
        //println!("{:?}", disk);
        disk.iter().enumerate().map(|(i, b)| i * b).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let map = (*v).clone();
        let mut disk = vec![];

        #[derive(Debug)]
        struct DiskPlace {
            moved: bool,
            offset: usize,
            block: DiskMap,
        }

        let mut offset = 0;
        let mut placed: Vec<_> = map
            .into_iter()
            .map(|f| {
                let d = DiskPlace {
                    moved: false,
                    offset,
                    block: f.clone(),
                };
                offset += f.files;
                offset += f.free_space;
                d
            })
            .collect();

        let mut pos = placed.len() - 1;
        loop {
            if placed[pos].moved {
                pos -= 1;
                if pos == 0 {
                    break;
                }
                continue;
            }
            placed[pos].moved = true;
            if let Some(i) = placed
                .iter()
                .take(pos)
                .position(|p| p.block.free_space >= placed[pos].block.files)
            {
                placed[pos - 1].block.free_space =
                    placed[pos].block.files + placed[pos].block.free_space;
                let old_free = std::mem::replace(&mut placed[i].block.free_space, 0);

                let new_place = DiskPlace {
                    moved: true,
                    offset: placed[i].offset + placed[i].block.files,
                    block: DiskMap {
                        free_space: old_free - placed[pos].block.files,
                        ..placed[pos].block
                    },
                };

                let _ = placed.remove(pos);
                placed.insert(i + 1, new_place);
            }
        }

        for p in &placed {
            disk.extend(std::iter::repeat(None).take(p.offset - disk.len()));
            assert!(p.offset == disk.len());
            disk.extend(std::iter::repeat(Some(p.block.id)).take(p.block.files));
        }
        disk.iter()
            .enumerate()
            .flat_map(|(i, b)| {
                let v = (*b)?;
                Some((i, v))
            })
            .map(|(i, b)| i * b)
            .sum()
    }
}

crate::default_tests!(6211348208140, 6239783302560);
crate::string_tests!(
    [(foo_sol1, "2333133121414131402", 1928)],
    [(foo_sol2, "2333133121414131402", 2858)]
);
