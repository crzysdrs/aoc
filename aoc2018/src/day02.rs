use std::fs::File;
use std::io::BufRead;

pub fn p1() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day2.txt")?);
    let r: (u32, u32) = f
        .lines()
        .map(|x| {
            let mut v = vec![0u8; 26];
            x.unwrap().chars().for_each(|c| {
                let n = c as usize - 'a' as usize;
                v[n] += 1
            });
            v
        })
        .map(|v| (u32::from(v.contains(&2)), u32::from(v.contains(&3))))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("{}", r.0 * r.1);
    Ok(())
}

pub fn p2() -> std::io::Result<()> {
    let f = std::io::BufReader::new(File::open("input/day2.txt")?);
    let mut hm = std::collections::HashMap::<String, String>::new();
    let r = f.lines().try_for_each(|s| {
        let s = s.unwrap();
        for i in 0..s.len() {
            let mut new_s = s.clone();
            new_s.replace_range(i..i + 1, ".");
            if hm.contains_key(&new_s) {
                return Err(new_s);
            }
            hm.entry(new_s).or_insert_with(|| s.clone());
        }
        Ok(())
    });

    println!("{}", r.unwrap_err());
    Ok(())
}
