use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

fn cups_score(cups: &[usize]) -> usize {
    let mut start = 1;
    let mut state = 0;
    while cups[start] != 1 {
        state *= 10;
        state += cups[start];
        start = cups[start];
    }
    state
}

#[allow(dead_code)]
fn cups_arr(cups: &[usize]) -> Vec<usize> {
    //println!("{:?}", cups);
    let mut new = vec![];
    let mut start = 1;
    new.push(1);
    while cups[start] != 1 {
        new.push(cups[start]);
        start = cups[start];
    }
    new
}

fn cups_game(iterations: usize, v: &[usize]) -> Vec<usize> {    
    let mut cups = vec![];
    cups.push(0);
    cups.resize(v.len() + 1, 0);
    
    v.windows(2).for_each(|win| {
        cups[win[0]] = win[1];
    });
    cups[*v.last().unwrap()] = v[0];
    
    let max = *v.iter().max().unwrap();
    let mut cur = v[0];

    for _m in  1..=iterations {
        //println!("-- move {} --", m);
        //println!("cur: {}", cur);
        //println!("cups: {:?}", cups_arr(&cups));
        let cup1 = cups[cur];
        let cup2 = cups[cup1];
        let cup3 = cups[cup2];

        cups[cur] = cups[cup3];
        
        //println!("pickup: {} {} {}", cup1, cup2, cup3);
        let mut label = cur - 1;
        if label == 0 {
            label = max;
        }
        while label == cup1 || label == cup2 || label == cup3 {
            label -= 1;
            if label == 0 {
                label = max;
            }
        }
        
        let after_label = cups[label];
        cups[label] = cup1;
        cups[cup3] = after_label;
        
        //println!("destination: {}", label);

        cur = cups[cur]
    }

    //println!("-- final --");
    //println!("cups: {:?}", cups);
    cups    
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 23;
    type Input = usize;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        let mut input : usize = r.lines().next().unwrap().unwrap().parse().unwrap();
        let mut cups = vec![];

        while input > 0 {
            cups.push(input %10);
            input /= 10;
        }
        cups.reverse();
        Ok(cups)
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        cups_score(&cups_game(100, &v))
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        let mill = 1_000_000;
        let ten_mill = 10_000_000;
        let len = v.len() + 1;
        let mut new_v = v.to_vec();
        new_v.extend(len..=mill);
        let cups = cups_game(ten_mill, &new_v);
        let a = cups[1];
        let b = cups[a];
        a * b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "389125467";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(cups_score(&cups_game(10, &v)), 92658374);
        assert_eq!(Solution::p1(&v), 67384529);

        assert_eq!(Solution::p2(&v), 149245887792);
    }
}
