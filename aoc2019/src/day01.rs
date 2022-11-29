use std::io::Result as IoResult;

fn fuel_required(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn recursive_fuel_required(mass: u32) -> u32 {
    if mass == 0 {
        0
    } else {
        let new_mass = (mass / 3).saturating_sub(2);
        new_mass + recursive_fuel_required(new_mass)
    }
}

pub fn p1() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day1.txt")?
        .lines()
        .map(|x| fuel_required(x.parse::<u32>().expect("Valid int")))
        .sum::<u32>();
    println!("Day 1 P1: {}", s);
    Ok(())
}

pub fn p2() -> IoResult<()> {
    let s = std::fs::read_to_string("input/day1.txt")?
        .lines()
        .map(|x| recursive_fuel_required(x.parse::<u32>().expect("Valid int")))
        .sum::<u32>();
    println!("Day 1 P2: {}", s);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn masses() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);

        assert_eq!(recursive_fuel_required(14), 2);
        assert_eq!(recursive_fuel_required(1969), 966);
        assert_eq!(recursive_fuel_required(100756), 50346);
    }
}
