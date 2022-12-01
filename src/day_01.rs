use std::{error::Error, fs::read_to_string};

pub fn day01() -> Result<(u32, u32), Box<dyn Error>> {
    let input = read_to_string("day_01.txt").unwrap();
    let mut calories_by_elf = vec![];
    let mut total_calories = 0;

    input.split("\n").for_each(|i| {
        if i.is_empty() {
            calories_by_elf.push(total_calories);
            total_calories = 0;
        } else {
            total_calories += i.parse::<u32>().unwrap();
        }
    });

    // Part 1's answer.
    let part_01 = calories_by_elf.iter().max().unwrap().to_owned();

    // Part 2's answer.
    calories_by_elf.sort_by(|a, b| b.cmp(a));
    let part_02 = calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2];

    Ok((part_01, part_02))
}
