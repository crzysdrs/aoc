use crate::Day;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9999;
    type Input = ();
    type Sol1 = u32;
    type Sol2 = u32;

    fn process_input<R>(_r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        unimplemented!()
    }
    fn p1(_v: &[Self::Input]) -> Self::Sol1 {
        unimplemented!()
    }
    fn p2(_v: &[Self::Input]) -> Self::Sol2 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    #[test]
    fn test() {
        //unimplemented!()
    }
}
