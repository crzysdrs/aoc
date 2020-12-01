use std::io::Result as IoResult;
use crate::Day;

pub struct Solution {}

impl Day for Solution {
    const DAY : u32 = 1;
     fn p1() -> IoResult<()> {
        unimplemented!("Missing implementation of Day {} Part 1", Self::DAY)
    }
    fn p2() -> IoResult<()> {
        unimplemented!("Missing implementation of Day {} Part 2", Self::DAY)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert!(true);
    }
}
