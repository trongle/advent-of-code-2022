fn calculate_a_score_of_one_round_part_01(round: &str) -> usize {
    match round {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("Round {} is incorrect.", round),
    }
}

fn calculate_a_score_of_one_round_part_02(round: &str) -> usize {
    match round {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!("Round {} is incorrect.", round),
    }
}

pub fn day_02() {
    let rounds = include_str!("day_02.txt");

    let part_1: usize = rounds
        .lines()
        .map(|r| calculate_a_score_of_one_round_part_01(r))
        .sum();

    let part_2: usize = rounds
        .lines()
        .map(|r| calculate_a_score_of_one_round_part_02(r))
        .sum();

    println!("{}, {}", part_1, part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // ROCK:     A X (+1)
    // PAPER:    B Y (+2)
    // SCISSORS: C Z ()+3)
    //
    // A X < B Y < C Z < A X
    //
    // WIN: +6
    // DRAW: +3
    // LOSE: +0
    fn calculate_a_score_of_one_round_part_01_successfully() {
        let result = vec![
            ("A X", 4),
            ("A Y", 8),
            ("A Z", 3),
            ("B X", 1),
            ("B Y", 5),
            ("B Z", 9),
            ("C X", 7),
            ("C Y", 2),
            ("C Z", 6),
        ];

        result.iter().for_each(|r| {
            let score = calculate_a_score_of_one_round_part_01(r.0);
            assert_eq!(r.1, score)
        });
    }

    #[test]
    // ROCK:     A
    // PAPER:    B
    // SCISSORS: C
    //
    // A < B < C < A
    //
    // NEED LOSE  +0: X => IF A THEN C +3, IF B THEN A +1, IF C THEN B +2
    // NEED WIN   +6: Z => IF A THEN B +2, IF B THEN C +3, IF C THEN A +1
    // NEED DRAW  +3: Y => If A THEN A +1, IF B THEN B +2, IF C THEN C +3
    fn calculate_a_score_of_one_round_part_02_successfully() {
        let result = vec![
            ("A X", 3),
            ("A Y", 4),
            ("A Z", 8),
            ("B X", 1),
            ("B Y", 5),
            ("B Z", 9),
            ("C X", 2),
            ("C Y", 6),
            ("C Z", 7),
        ];

        result.iter().for_each(|r| {
            let score = calculate_a_score_of_one_round_part_02(r.0);
            assert_eq!(r.1, score)
        });
    }
}
