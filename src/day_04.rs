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
    let (part_1, part_2) = parse_pair(pair);

    part_1[0] <= part_2[0] && part_1.last() >= part_2.last()
        || part_2[0] <= part_1[0] && part_2.last() >= part_1.last()
}

fn is_overlap(pair: &str) -> bool {
    let (part_1, part_2) = parse_pair(pair);

    part_1.contains(&part_2[0])
        || part_1.contains(&part_2.last().unwrap())
        || part_2.contains(&part_1[0])
        || part_2.contains(&part_1.last().unwrap())
}

fn parse_pair(text: &str) -> (Vec<usize>, Vec<usize>) {
    let pair: Vec<&str> = text.split(',').collect();
    let part_1: Vec<usize> = pair[0].split('-').map(|n| n.parse().unwrap()).collect();
    let part_2: Vec<usize> = pair[1].split('-').map(|n| n.parse().unwrap()).collect();

    (
        (part_1[0]..=part_1[1]).collect::<Vec<usize>>(),
        (part_2[0]..=part_2[1]).collect::<Vec<usize>>(),
    )
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

    #[test]
    fn test_parse_pair() {
        let (part_1, part_2) = parse_pair("5-7,7-9");

        assert_eq!(vec![5, 6, 7], part_1);
        assert_eq!(vec![7, 8, 9], part_2);
    }
}
