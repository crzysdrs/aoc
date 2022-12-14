use crate::Day;
use std::io::Result as IoResult;

pub struct Solution {}

fn find_pair(v: &[u32], sum: u32) -> Option<(u32, u32)> {
    if v.len() < 2 {
        return None;
    }
    let mut i = 0;
    let mut j = v.len() - 1;

    while i < j {
        use std::cmp::Ordering;
        match (v[i] + v[j]).cmp(&sum) {
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
            _ => return Some((v[i], v[j])),
        }
    }
    None
}

fn find_triple(v: &[u32], sum: u32) -> Option<(u32, u32, u32)> {
    if v.len() < 3 {
        return None;
    }
    for i in 0..v.len() {
        if v[i] >= sum {
            return None;
        } else if let Some((v2, v3)) = find_pair(&v[i + 1..], sum - v[i]) {
            return Some((v[i], v2, v3));
        }
    }
    None
}
impl Day for Solution {
    const DAY: u32 = 1;
    type Input = u32;
    type Sol1 = u32;
    type Sol2 = u32;
    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|x| -> IoResult<Self::Input> { Ok(x?.parse::<u32>().unwrap()) })
            .collect::<IoResult<Vec<_>>>()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        let mut v = v.to_vec();
        v.sort();

        let (v1, v2) = find_pair(&v, 2020).unwrap();
        v1 * v2
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mut v = v.to_vec();
        v.sort();

        let (v1, v2, v3) = find_triple(&v, 2020).unwrap();

        v1 * v2 * v3
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &[u32] = &[1721, 979, 366, 299, 675, 1456];
    #[test]
    fn test() {
        let mut test = TEST.to_vec();
        test.sort();
        assert_eq!(find_pair(&test, 2020), Some((299, 1721)));
        assert_eq!(find_triple(&test, 2020), Some((366, 675, 979)));
    }
}
