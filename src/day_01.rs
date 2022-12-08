use std::fs::read_to_string;

pub fn _day_01_bak() {
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
    let part_1 = calories_by_elf.iter().max().unwrap().to_owned();

    // Part 2's answer.
    calories_by_elf.sort_by(|a, b| b.cmp(a));
    let part_2 = calories_by_elf[0] + calories_by_elf[1] + calories_by_elf[2];

    println!("{}, {}", part_1, part_2);
}

// This version uses Collection and Iterator
// so it takes fewer variables than the previous
// one. Consequently, mre concise and readable.
pub fn day_01() {
    let mut collection = include_str!("day_01_test.txt")
        .split("\n\n")
        .map(|x| x.lines().flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect::<Vec<u32>>();

    // Part 1's answer.
    let part_1 = collection.iter().max().unwrap().to_owned();

    // Part 2's answer.
    collection.sort_by(|a, b| b.cmp(a));
    let part_2 = collection.iter().take(3).sum::<u32>();

    println!("{}, {}", part_1, part_2);
}
