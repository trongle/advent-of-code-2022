pub fn day_04() {
    let part_01_result = include_str!("day_04.txt")
        .lines()
        .filter(|p| is_fully_contain(p))
        .count();

    let part_02_result = include_str!("day_04.txt")
        .lines()
        .filter(|p| is_overlap(p))
        .count();

    println!("{}, {}", part_01_result, part_02_result);
}

fn is_fully_contain(pair: &str) -> bool {
    let pair: Vec<&str> = pair.split(',').collect();
    let (part_1, part_2): (Vec<usize>, Vec<usize>) = (
        pair[0].split('-').map(|n| n.parse().unwrap()).collect(),
        pair[1].split('-').map(|n| n.parse().unwrap()).collect(),
    );

    part_1[0] <= part_2[0] && part_1[1] >= part_2[1]
        || part_2[0] <= part_1[0] && part_2[1] >= part_1[1]
}

fn is_overlap(pair: &str) -> bool {
    let pair: Vec<&str> = pair.split(',').collect();
    let (part_1, part_2): (Vec<usize>, Vec<usize>) = (
        pair[0].split('-').map(|n| n.parse().unwrap()).collect(),
        pair[1].split('-').map(|n| n.parse().unwrap()).collect(),
    );

    let range_1 = (part_1[0]..=part_1[1]).collect::<Vec<usize>>();
    let range_2 = (part_2[0]..=part_2[1]).collect::<Vec<usize>>();

    range_1.contains(&range_2[0])
        || range_1.contains(&range_2.last().unwrap())
        || range_2.contains(&range_1[0])
        || range_2.contains(&range_1.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_is_fully_contain() {
        assert!(is_fully_contain("2-8,3-7"));
        assert!(!is_fully_contain("2-3,4-5"));
    }

    #[test]
    fn test_pair_is_overlap() {
        assert!(is_overlap("5-7,7-9"));
        assert!(is_overlap("2-8,3-7"));
        assert!(is_overlap("6-6,4-6"));
        assert!(is_overlap("2-6,4-8"));
        assert!(!is_overlap("2-4,6-8"));
        assert!(!is_overlap("2-3,4-5"));
    }
}
