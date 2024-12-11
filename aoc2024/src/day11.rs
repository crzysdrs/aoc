use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Stone(usize);

impl Stone {
    fn blink(self) -> (Stone, Option<Stone>) {
        if self.0 == 0 {
            return (Stone(1), None);
        } else if self.0.ilog10() % 2 == 1 {
            let len = self.0.ilog10() + 1;
            let l = Stone(self.0 / 10usize.pow(len / 2));
            let r = Stone(self.0 % 10usize.pow(len / 2));
            return (l, Some(r));
        } else {
            return (Stone(self.0 * 2024), None);
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 11;
    type Input1 = Vec<Stone>;
    type Input2 = Vec<Stone>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .flat_map(|s| {
                s.split_whitespace()
                    .map(|s| Stone(s.parse::<usize>().unwrap()))
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut v = (*v).clone();
        let mut new = vec![];
        for _b in 0..25 {
            for s in v.iter().cloned() {
                let (new_s, opt) = s.blink();
                new.push(new_s);
                if let Some(opt) = opt {
                    new.push(opt);
                }
            }
            std::mem::swap(&mut v, &mut new);
            new.clear();
        }
        v.len()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = HashMap::from_iter(v.iter().cloned().map(|s| (s, 1)));

        let mut new: HashMap<Stone, usize> = HashMap::new();

        for _b in 0..75 {
            for (s, count) in v.iter() {
                let (new_s, opt) = (*s).clone().blink();
                new.entry(new_s)
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
                if let Some(opt) = opt {
                    new.entry(opt)
                        .and_modify(|v| *v += *count)
                        .or_insert(*count);
                }
            }
            std::mem::swap(&mut v, &mut new);
            new.clear();
        }
        v.iter().map(|(_s, c)| c).sum()
    }
}

crate::default_tests!(186996, 221683913164898);
//crate::string_tests!([(foo_sol1, "hi1", 0)], [(foo_sol2, "hi2", 1)]);
