use std::io::BufRead;
use std::fs::File;

pub fn p1() -> std::io::Result<()> {
    use regex::Regex;
    let mut f = std::fs::read_to_string("input/day5.txt")?;
    //let mut f = "dabAcCaCBAcCcaDA".to_string();
    let mut v = Vec::new();
    for (c, c2) in (b'a'..=b'z').zip(b'A'..=b'Z').map(|(x,y)| (char::from(x), char::from(y))) {
        v.push(format!("{}{}", c, c2));
        v.push(format!("{}{}", c2, c));
    }
    let s = v.join("|");
    println!("{}", s);
    let re = Regex::new(&s).unwrap();

    let mut prev_len = 0;
    while prev_len != f.len() {
        prev_len = f.len();
        f = re.replace_all(&f, "").to_string();
        println!("{}", f.len());
    }
    println!("{}", f.len() - 1); //Stupid newline char
    Ok(())
}

pub fn p2() -> std::io::Result<()> {
use regex::Regex;
    let f = std::fs::read_to_string("input/day5.txt")?;
    //let mut f = "dabAcCaCBAcCcaDA".to_string();
    let mut v = Vec::new();
    for (c, c2) in (b'a'..=b'z').zip(b'A'..=b'Z').map(|(x,y)| (char::from(x), char::from(y))) {
        v.push(format!("{}{}", c, c2));
        v.push(format!("{}{}", c2, c));
    }
    let s = v.join("|");
    println!("{}", s);
    let re = Regex::new(&s).unwrap();

    let m = (b'a'..=b'z').zip(b'A'..=b'Z').map(|(x,y)| (char::from(x), char::from(y)))
        .map(
        |(c, c2)| {
            let mut f = f.clone();
            let s_re = format!("{}|{}", c, c2);
            let strip = Regex::new(&s_re).unwrap();
            f = strip.replace_all(&f, "").to_string();
            let mut prev_len = 0;
            while prev_len != f.len() {
                prev_len = f.len();
                f = re.replace_all(&f, "").to_string();
            }
            f.len()
        }
    ).min().unwrap();

    println!("{:?}", m - 1); //Stupid newline char
    Ok(())
}
