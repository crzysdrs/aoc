use std::io::Result as IoResult;

use std::collections::{HashMap, HashSet};

fn orbits(input_conns: &[(String, String)]) -> usize {
    let mut conns: HashMap<String, HashSet<String>> = HashMap::new();
    input_conns.iter().for_each(|(a, b)| {
        conns
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
    });

    let nodes = conns
        .iter()
        .flat_map(|(k, v)| std::iter::once(k).chain(v.iter()))
        .collect::<HashSet<_>>();

    let values = conns.values().flatten().collect::<HashSet<_>>();
    let roots = nodes.difference(&values);

    let mut worklist = roots.map(|r| (r.to_string(), 0)).collect::<Vec<_>>();
    let mut sum = 0;
    while let Some((w, c)) = worklist.pop() {
        sum += c;
        if let Some(vs) = conns.get(&w) {
            worklist.extend(vs.iter().map(|v| (v.to_string(), c + 1)));
        }
    }
    sum
}

pub fn p1() -> IoResult<()> {
    let conns = text_to_conns(std::fs::read_to_string("input/day6.txt")?);
    let sum = orbits(&conns);
    println!("Day 6 Part 1: {}", sum);
    Ok(())
}

fn shared_path(conns: &[(String, String)]) -> usize {
    let mut up = HashMap::new();
    conns.iter().for_each(|(a, b)| {
        up.entry(b.clone()).or_insert_with(|| a.clone());
    });

    let mut you =
        std::iter::successors(up.get("YOU"), |v| up.get(&(*v).clone())).collect::<Vec<_>>();
    let mut san =
        std::iter::successors(up.get("SAN"), |v| up.get(&(*v).clone())).collect::<Vec<_>>();

    you.reverse();
    san.reverse();

    let shared = you
        .iter()
        .zip(san.iter())
        .take_while(|(a, b)| a == b)
        .count();
    you.len() - shared + san.len() - shared
}

fn text_to_conns(s: String) -> Vec<(String, String)> {
    s.trim()
        .lines()
        .map(|x| {
            let mut s = x.split(')');
            let a = s.next().unwrap().to_string();
            let b = s.next().unwrap().to_string();
            (a, b)
        })
        .collect()
}
pub fn p2() -> IoResult<()> {
    let conns = text_to_conns(std::fs::read_to_string("input/day6.txt")?);
    println!("Day 6 Part 2: {}", shared_path(&conns));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(
            orbits(&text_to_conns(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
                .to_string()
            )),
            42
        );

        assert_eq!(
            shared_path(&text_to_conns(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
                    .to_string()
            )),
            4
        );
    }
}
