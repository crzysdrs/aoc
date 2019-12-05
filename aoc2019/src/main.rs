mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> std::io::Result<()> {
    if false {
        day1::p1()?;
        day1::p2()?;

        day2::p1()?;
        day2::p2()?;

        day3::p1()?;
        day3::p2()?;

        day4::p1()?;
        day4::p2()?;
    }
    day5::p1()?;
    day5::p2()?;

    Ok(())
}
