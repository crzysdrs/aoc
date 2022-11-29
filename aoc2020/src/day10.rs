use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 10;
    type Input = u32;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                Ok(l.parse::<u32>().unwrap())
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut v = v.to_vec();
        v.push(0);
        let max = *v.iter().max().unwrap();
        v.push(max + 3);
        v.sort();

        let jolts = v.windows(2).map(|items| items[1] - items[0]).fold(
            HashMap::new(),
            |mut state, diff| {
                state.entry(diff).and_modify(|x| *x += 1).or_insert(1);
                state
            },
        );

        jolts.get(&1).cloned().unwrap_or(0) * jolts.get(&3).cloned().unwrap_or(0)
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut v = v.to_vec();
        v.push(0);
        let max = *v.iter().max().unwrap();
        v.push(max + 3);
        v.sort();

        let nodes = v.clone().into_iter().collect::<HashSet<_>>();
        let adj = v
            .iter()
            .map(|v| {
                (
                    *v,
                    (v + 1..v + 4)
                        .filter(|new_v| nodes.contains(new_v))
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, e)| !e.is_empty())
            .collect::<HashMap<_, _>>();

        let mut memo = HashMap::new();

        count_paths(&mut memo, &adj, 0)
    }
}

fn count_paths(memo: &mut HashMap<u32, usize>, adj: &HashMap<u32, Vec<u32>>, node: u32) -> usize {
    if let Some(count) = memo.get(&node) {
        *count
    } else if let Some(edges) = adj.get(&node) {
        let count = edges.iter().map(|e| count_paths(memo, adj, *e)).sum();
        memo.insert(node, count);
        count
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 22 * 10);
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&v), 19208);
    }
}
