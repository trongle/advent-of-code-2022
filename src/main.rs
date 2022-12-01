use std::error::Error;

use day_01::day01;

mod day_01;

fn main() -> Result<(), Box<dyn Error>> {
    let (day_01_part_01, day_01_part_02) = day01()?;
    
    println!("{}, {}", day_01_part_01, day_01_part_02);

    Ok(())
}
