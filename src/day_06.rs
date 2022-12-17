use std::collections::HashSet;
pub fn day_06() {
    let text = include_str!("day_06.txt");

    println!("{}, {}", find_nocpbtfmd(text, 4), find_nocpbtfmd(text, 14));
}

fn find_nocpbtfmd(buffer: &str, length: usize) -> usize {
    let buffer = buffer.to_string();
    let mut result = 0;

    for i in 0..buffer.len() {
        let mut set = HashSet::with_capacity(length);
        for j in 1..=length {
            set.insert(buffer[(i + j - 1)..(i + j)].to_string());
        }

        if set.len() == length {
            result = i + length;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_of_characters_processed_before_the_first_marker_detected() {
        assert_eq!(7, find_nocpbtfmd("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, find_nocpbtfmd("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_nocpbtfmd("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_nocpbtfmd("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_nocpbtfmd("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
        assert_eq!(19, find_nocpbtfmd("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_nocpbtfmd("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_nocpbtfmd("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_nocpbtfmd("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_nocpbtfmd("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
