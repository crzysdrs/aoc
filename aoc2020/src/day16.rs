use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;
use std::ops::Range;

#[derive(Debug)]
pub struct Ticket {
    fields: HashMap<String, Vec<Range<usize>>>,
}

#[derive(Debug)]
pub struct Input {
    ticket: Ticket,
    yours: Vec<usize>,
    nearby: Vec<Vec<usize>>,
}
pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 16;
    type Input = Input;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let field = regex::Regex::new(r"([^:]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
        let mut lines = r.lines();

        let fields = lines.by_ref().flatten().take_while(|x| !x.is_empty()).fold(
            HashMap::new(),
            |mut m, l| {
                if let Some(cap) = field.captures(&l) {
                    let name = cap.get(1).unwrap().as_str().to_string();
                    let r1 = cap.get(2).unwrap().as_str().parse::<usize>().unwrap()
                        ..cap.get(3).unwrap().as_str().parse::<usize>().unwrap() + 1;
                    let r2 = cap.get(4).unwrap().as_str().parse::<usize>().unwrap()
                        ..cap.get(5).unwrap().as_str().parse::<usize>().unwrap() + 1;

                    m.insert(name, vec![r1, r2]);
                } else {
                    panic!()
                }
                m
            },
        );

        let ticket = Ticket { fields };

        let yours: Vec<usize> = lines
            .by_ref()
            .map_while(Result::ok)
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let nearby: Vec<Vec<usize>> = lines
            .map_while(Result::ok)
            .skip(2)
            .map(|l| l.split(',').map(|x| x.parse::<usize>().unwrap()).collect())
            .collect();

        let input = Input {
            ticket,
            yours,
            nearby,
        };
        Ok(vec![input])
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let input = &v[0];

        input
            .nearby
            .iter()
            .flatten()
            .flat_map(|data| {
                if !input
                    .ticket
                    .fields
                    .iter()
                    .any(|(_, ranges)| ranges.iter().any(|val| val.contains(data)))
                {
                    Some(data)
                } else {
                    None
                }
            })
            .sum()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let input = &v[0];

        let valid = input
            .nearby
            .iter()
            .chain(std::iter::once(&input.yours))
            .flat_map(|ticket| {
                if ticket.iter().all(|data| {
                    input
                        .ticket
                        .fields
                        .iter()
                        .any(|(_, ranges)| ranges.iter().any(|val| val.contains(data)))
                }) {
                    Some(ticket)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut transpose: Vec<Vec<_>> = vec![];
        for i in 0..valid[0].len() {
            transpose.push(valid.iter().map(|ticket| ticket[i]).collect());
        }

        let mut guessed = transpose
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let fields = input
                    .ticket
                    .fields
                    .iter()
                    .map(move |(f, ranges)| {
                        (
                            f,
                            row.iter()
                                .all(|data| ranges.iter().any(|val| val.contains(data))),
                        )
                    })
                    .filter(|(_, all)| *all)
                    .map(|(f, _)| f)
                    .collect::<HashSet<_>>();
                (i, fields)
            })
            .collect::<Vec<_>>();

        let mut real_fields = HashMap::new();
        while !guessed.is_empty() {
            guessed.sort_by_key(|(_, s)| s.len());
            guessed.reverse();
            while let Some((i, fields)) = guessed.pop() {
                assert_eq!(fields.len(), 1);
                let field = fields.iter().next().unwrap().to_string();
                guessed.iter_mut().for_each(|g| {
                    g.1.remove(&field);
                });
                real_fields.insert(field, i);
            }
        }

        real_fields
            .iter()
            .filter(|(f, _)| f.starts_with("departure"))
            .map(|(_, i)| input.yours[*i])
            .product()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 71);
    }
}
