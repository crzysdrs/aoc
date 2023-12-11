use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    maps: HashMap<(String, String), Map>,
}

#[derive(Debug, PartialEq)]
pub struct Mapping {
    source: Range<usize>,
    dest: Range<usize>,
}

#[derive(Debug)]
pub struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn any_range(&self, source: usize) -> Option<&Mapping> {
        self.mappings.iter().find(|x| x.source.contains(&source))
    }
    fn find_dest(&self, source: usize) -> usize {
        self.mappings
            .iter()
            .find(|x| x.source.contains(&source))
            .map(|x| x.dest.start + source - x.source.start)
            .unwrap_or(source)
    }
    fn find_dest_ranges(&self, source: Range<usize>) -> Vec<Range<usize>> {
        let mut pois: Vec<_> = self
            .mappings
            .iter()
            .flat_map(|x| {
                vec![
                    x.source.start.saturating_sub(1),
                    x.source.start,
                    x.source.end - 1,
                    x.source.end,
                ]
            })
            .collect();
        pois.push(source.start);
        pois.push(source.end - 1);
        pois.sort();
        pois.dedup();

        let start = pois.partition_point(|v| source.start > *v);
        let end = pois.partition_point(|v| (source.end - 1) >= *v);
        // println!("{:?}", source);
        // println!("{:?}", pois);
        // println!("{:?}", (start, end));
        pois[start..end]
            .windows(2)
            .flat_map(|x| {
                if self.any_range(x[0]) == self.any_range(x[1]) {
                    let l = self.find_dest(x[0]);
                    let r = self.find_dest(x[1]);
                    Some(l..r + 1)
                } else {
                    None
                }
            })
            .collect()
    }
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
        let first = lines.by_ref().next().unwrap();
        let _ = lines.by_ref().next();
        let mut maps = HashMap::default();

        let seeds = first.strip_prefix("seeds: ").unwrap();
        let seeds = seeds
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        while let Some(l) = lines.by_ref().next() {
            let (xtox, _) = l.split_once(' ').unwrap();
            let (src, dst) = xtox.split_once("-to-").unwrap();
            let mut ranges = vec![];
            while let Some(l) = lines.by_ref().next() {
                if l.is_empty() {
                    break;
                }
                let v: Vec<_> = l
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                ranges.push(Mapping {
                    dest: v[0]..v[0] + v[2],
                    source: v[1]..v[1] + v[2],
                });
            }
            ranges.sort_by_key(|v| v.source.start);
            maps.insert((src.to_string(), dst.to_string()), Map { mappings: ranges });
        }

        Input { seeds, maps }
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let search = [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];
        let mut current = v.seeds.clone();

        for s in search.windows(2) {
            let &[source, dest] = s else {
                panic!();
            };
            let mappings = v.maps.get(&(source.to_string(), dest.to_string())).unwrap();
            current = current.iter().map(|v| mappings.find_dest(*v)).collect();
        }

        let min = current.iter().min().unwrap();
        *min
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let search = [
            "seed",
            "soil",
            "fertilizer",
            "water",
            "light",
            "temperature",
            "humidity",
            "location",
        ];
        let mut current: Vec<_> = v.seeds.chunks(2).map(|v| v[0]..v[0] + v[1]).collect();

        for s in search.windows(2) {
            let &[source, dest] = s else {
                panic!();
            };
            let mappings = v.maps.get(&(source.to_string(), dest.to_string())).unwrap();
            current = current
                .iter()
                .flat_map(|v| mappings.find_dest_ranges(v.clone()))
                .collect();
        }

        let min = current.iter().map(|x| x.start).min().unwrap();
        min
    }
}

crate::default_tests!(600279879, 20191102);
crate::string_tests!(
    [(
        foo_sol1,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        35
    )],
    [(
        foo_sol2,
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        46
    )]
);
