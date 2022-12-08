pub fn day_03() {
    let part_1: usize = include_str!("day_03.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let (cpm1, cpm2) = break_rusksack(line);
            get_priority(
                find_common(cpm1, cpm2, None)
                    .unwrap()
                    .to_string()
                    .as_bytes(),
            ) as usize
        })
        .sum();

    let lines: Vec<&str> = include_str!("day_03.txt").lines().collect();
    let mut part_2: usize = 0;
    for i in (0..lines.len()).step_by(3) {
        let common = find_common(lines[i], lines[i + 1], Some(lines[i + 2]));
        part_2 += get_priority(common.unwrap().to_string().as_bytes()) as usize;
    }

    println!("{}, {}", part_1, part_2);
}

fn break_rusksack(rusksack: &str) -> (&str, &str) {
    let len = rusksack.len();
    let mid = len / 2;

    (&rusksack[0..mid], &rusksack[mid..len])
}

fn find_common(cpm1: &str, cpm2: &str, cpm3: Option<&str>) -> Option<char> {
    for c in cpm1.chars() {
        if cpm2.contains(c) {
            if cpm3.is_some() {
                if cpm3.unwrap().contains(c) {
                    return Some(c);
                }
            } else {
                return Some(c);
            }
        }
    }
    None
}

fn get_priority(c: &[u8]) -> u8 {
    let c = *c.first().unwrap();
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_a_rusksack_into_2_compartments() {
        let rusksack = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let (cpm1, cpm2) = break_rusksack(rusksack);

        assert_eq!(cpm1.len(), cpm2.len());
    }

    #[test]
    fn find_common_item() {
        let (cpm1, cpm2) = ("vJrwpWtwJgWr", "hcsFMMfFFhFp");

        let common = find_common(cpm1, cpm2, None);

        assert_eq!(Some('p'), common);
    }

    #[test]
    fn find_common_item_with_3_elements() {
        let (cpm1, cpm2, cpm3) = (
            "lvcNpRHDCnTLCJlL",
            "RFZggsMrjTFGCJmdmd",
            "srsBZgBqwBqRZbzqtHpzzDNtHDqV",
        );

        let common = find_common(cpm1, cpm2, Some(cpm3));

        assert_eq!(Some('R'), common);
    }

    #[test]
    fn get_priority_of_item() {
        assert_eq!(1, get_priority('a'.to_string().as_bytes()));
        assert_eq!(26, get_priority('z'.to_string().as_bytes()));
        assert_eq!(27, get_priority('A'.to_string().as_bytes()));
        assert_eq!(52, get_priority('Z'.to_string().as_bytes()));
    }
}
