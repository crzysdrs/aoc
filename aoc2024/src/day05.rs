use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
pub struct Order {
    l: usize,
    r: usize,
}

#[derive(Debug)]
pub struct Produce {
    pages: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
    order: Vec<Order>,
    produce: Vec<Produce>,
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 5;
    type Input1 = Input;
    type Input2 = Input;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut lines = s.lines();
        let before: Vec<_> = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let mut pipe = s.split('|').map(|s| s.parse::<usize>().unwrap());
                let l = pipe.next().unwrap();
                let r = pipe.next().unwrap();

                Order { l, r }
            })
            .collect();

        let pages: Vec<_> = lines
            .map(|s| Produce {
                pages: s.split(',').map(|v| v.parse::<usize>().unwrap()).collect(),
            })
            .collect();

        Input {
            produce: pages,
            order: before,
        }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        v.produce
            .iter()
            .filter(|p| {
                let h: HashMap<_, _> = p.pages.iter().enumerate().map(|(i, v)| (v, i)).collect();

                v.order.iter().all(|p| match (h.get(&p.l), h.get(&p.r)) {
                    (Some(l), Some(r)) => l < r,
                    _ => true,
                })
            })
            .map(|p| p.pages[p.pages.len() / 2])
            .sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut before: HashMap<usize, Vec<usize>> = HashMap::new();
        for o in &v.order {
            before
                .entry(o.l)
                .and_modify(|v| (*v).push(o.r))
                .or_insert_with(|| vec![o.r]);
        }

        v.produce
            .iter()
            .filter(|p| {
                let h: HashMap<_, _> = p.pages.iter().enumerate().map(|(i, v)| (v, i)).collect();

                !v.order.iter().all(|p| match (h.get(&p.l), h.get(&p.r)) {
                    (Some(l), Some(r)) => l < r,
                    _ => true,
                })
            })
            .map(|p| {
                let mut pages = p.pages.clone();

                for (l_order, values) in before.iter() {
                    let h: HashMap<_, _> = pages.iter().enumerate().map(|(i, v)| (*v, i)).collect();
                    let min = values.iter().flat_map(|v| h.get(&v)).min();

                    if let (Some(l), Some(r)) = (h.get(l_order), min) {
                        if l > r {
                            let old_l = pages.remove(*l);
                            assert_eq!(old_l, *l_order);
                            pages.insert(*r, *l_order)
                        }
                    }
                }
                let h: HashMap<_, _> = pages.iter().enumerate().map(|(i, v)| (*v, i)).collect();

                assert!(v.order.iter().all(|p| match (h.get(&p.l), h.get(&p.r)) {
                    (Some(l), Some(r)) => l < r,
                    _ => true,
                }));

                Produce { pages, ..*p }
            })
            .map(|p| p.pages[p.pages.len() / 2])
            .sum()
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [(
        foo_sol1,
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        143
    )],
    [(
        foo_sol2,
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        123
    )]
);
