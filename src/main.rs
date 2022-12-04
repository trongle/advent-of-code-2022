mod day_01;
mod day_02;
mod day_03;

use crate::day_01::day_01;
use crate::day_02::day_02;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (day_01_part_01, day_01_part_02) = day_01()?;

    println!("{}, {}", day_01_part_01, day_01_part_02);

    let (day_02_part_01, day_02_part_02) = day_02();

    println!("{}, {}", day_02_part_01, day_02_part_02);

    day_03::day_03();

    Ok(())
}
