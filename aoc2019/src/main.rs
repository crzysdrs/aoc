mod day1;
mod day2;

fn main() -> std::io::Result<()>{
    day1::p1()?;
    day1::p2()?;

    day2::p1()?;
    day2::p2()?;

    Ok(())
}
