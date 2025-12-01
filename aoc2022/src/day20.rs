use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

pub struct Solution {}

#[derive(Copy, Clone, Debug)]
struct Entry {
    prev: usize,
    next: usize,
}

struct Indices {
    idx: Vec<Entry>,
}

impl Indices {
    fn remove(&mut self, idx: usize) {
        let cur = self.idx[idx];
        self.idx[cur.prev].next = cur.next;
        self.idx[cur.next].prev = cur.prev;
    }
    fn insert(&mut self, idx: usize, at: usize) {
        let old_next = self.idx[at].next;
        self.idx[at].next = idx;
        self.idx[old_next].prev = idx;

        self.idx[idx].prev = at;
        self.idx[idx].next = old_next;
    }
    fn iter(&self) -> IndexIter<'_> {
        self.iter_at(0)
    }
    fn iter_at(&self, at: usize) -> IndexIter<'_> {
        IndexIter {
            idx: self,
            cur_idx: at % self.idx.len(),
        }
    }
}

struct IndexIter<'a> {
    idx: &'a Indices,
    cur_idx: usize,
}

impl<'a> Iterator for IndexIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let old_index = self.cur_idx;
        self.cur_idx = self.idx.idx[self.cur_idx].next;
        Some(old_index)
    }
}

impl<'a> DoubleEndedIterator for IndexIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.cur_idx = self.idx.idx[self.cur_idx].prev;
        Some(self.cur_idx)
    }
}

fn mix(vals: &[i64], iterations: usize) -> i64 {
    let entries: Vec<_> = (0..vals.len())
        .map(|x| Entry {
            next: (x + 1) % vals.len(),
            prev: (x + vals.len() - 1) % vals.len(),
        })
        .collect();

    let mut indices = Indices { idx: entries };
    for _ in 0..iterations {
        for (idx, v) in vals.iter().enumerate() {
            let u_v = v.unsigned_abs() as usize % (vals.len() - 1);
            if *v == 0 || u_v == 0 {
                /* skip */
            } else {
                let after = indices.iter_at(idx).nth(1).unwrap();
                indices.remove(idx);
                let mut iter = indices.iter_at(after);

                let insert_at = if *v < 0 {
                    iter.nth_back(u_v)
                } else {
                    iter.nth(u_v - 1)
                }
                .unwrap();
                indices.insert(idx, insert_at);
            }
        }
    }

    indices
        .iter()
        .map(|v| vals[v])
        .skip_while(|v| *v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum::<i64>()
}

impl Day for Solution {
    const DAY: u32 = 20;
    type Input1 = Vec<i32>;
    type Input2 = Vec<i32>;
    type Sol1 = i32;
    type Sol2 = i64;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines().map(|x| x.parse().unwrap()).collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(vals: &Self::Input1) -> Self::Sol1 {
        mix(&vals.iter().map(|v| i64::from(*v)).collect::<Vec<_>>(), 1) as i32
    }
    fn p2(vals: &Self::Input2) -> Self::Sol2 {
        let vals: Vec<_> = vals.iter().map(|v| i64::from(*v) * 811589153).collect();
        mix(&vals, 10)
    }
}

crate::default_tests!(3346, 4265712588168);
crate::path_tests!(
    [(t1, "test/day20.txt", 3)],
    [(t2, "test/day20.txt", 1623178306)]
);
