use crate::Day;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    //const DAY: u32 = 1;
    fn p1() -> IoResult<()> {
        let mut v = std::fs::read_to_string(Self::input())?
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        unimplemented!();
        println!("{:?}", v1 * v2);
        Ok(())
    }
    fn p2() -> IoResult<()> {
        let mut v = std::fs::read_to_string(Self::input())?
            .lines()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        unimplemented!();

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        unimplemented!()
    }
}
