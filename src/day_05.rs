use std::{cell::RefCell, collections::HashMap};

pub fn day_05() {
    let input: Vec<&str> = include_str!("day_05.txt").split("\n\n").collect();
    let (input_1, input_2) = (input[0], input[1]);

    // Part 1
    let stacks = parse_stacks_of_creates(input_1);
    input_2
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|command| {
            do_command(command, &stacks, false);
        });

    for (k, v) in stacks {
        println!("key: {}, value: {:?} ", k, v.borrow().last());
    }
    
    println!("==============");

    // Part 2
    let stacks = parse_stacks_of_creates(input_1);
    input_2
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|command| {
            do_command(command, &stacks, true);
        });

    for (k, v) in stacks {
        println!("key: {}, value: {:?} ", k, v.borrow().last());
    }
}

pub fn parse_stacks_of_creates(text: &str) -> HashMap<u8, RefCell<Vec<&str>>> {
    let mut map: HashMap<u8, RefCell<Vec<&str>>> = HashMap::new();

    let mut lines: Vec<&str> = text.lines().collect();

    lines
        .pop()
        .unwrap()
        .split("   ")
        .map(|s| s.trim().parse::<u8>())
        .for_each(|n| {
            if let Ok(n) = n {
                map.insert(n, RefCell::new(Vec::new()));
            }
        });

    lines.reverse();
    lines.iter().for_each(|line| {
        let mut nth: u8 = 1;
        for i in (0..line.len()).step_by(4) {
            let word = &line[i..(i + 3)].trim();
            if !word.is_empty() {
                if let Some(value) = map.get(&nth) {
                    value.borrow_mut().push(word);
                }
            }
            nth += 1;
        }
    });

    map
}

fn do_command(command: &str, stacks: &HashMap<u8, RefCell<Vec<&str>>>, is_batch: bool) {
    let (moving, from, to) = parse_command(command);

    let from = stacks.get(&from).unwrap();
    let to = stacks.get(&to).unwrap();

    if !is_batch {
        for _i in 0..moving {
            to.borrow_mut().push(from.borrow_mut().pop().unwrap());
        }
    } else {
        let mut tmp = Vec::new();
        for _i in 0..moving {
            tmp.push(from.borrow_mut().pop().unwrap());
        }
        while !tmp.is_empty() {
            to.borrow_mut().push(tmp.pop().unwrap());
        }
    }
}

fn parse_command(text: &str) -> (u8, u8, u8) {
    let command: Vec<&str> = text.split(' ').collect();
    (
        command[1].parse().unwrap(),
        command[3].parse().unwrap(),
        command[5].parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stacks_of_crates() {
        let text = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        let result = parse_stacks_of_creates(text);

        assert_eq!(3, result.len());
        assert_eq!(Some(&RefCell::new(vec!["[Z]", "[N]"])), result.get(&1));
        assert_eq!(
            Some(&RefCell::new(vec!["[M]", "[C]", "[D]"])),
            result.get(&2)
        );
        assert_eq!(Some(&RefCell::new(vec!["[P]"])), result.get(&3));
    }

    #[test]
    fn test_parse_command() {
        let text = "mrve 1 from 2 to 1";

        let command = parse_command(text);

        assert_eq!(1, command.0);
        assert_eq!(2, command.1);
        assert_eq!(1, command.2);
    }

    #[test]
    fn test_do_command() {
        let mut map = HashMap::new();
        map.insert(1, RefCell::from(vec!["[Z]", "[N]"]));
        map.insert(2, RefCell::from(vec!["[M]", "[C]", "[D]"]));
        map.insert(3, RefCell::from(vec!["[P]"]));

        do_command("move 1 from 2 to 1", &map, false);
        do_command("move 3 from 1 to 3", &map, false);
        do_command("move 2 from 2 to 1", &map, false);
        do_command("move 1 from 1 to 2", &map, false);

        assert_eq!(Some(&RefCell::from(vec!["[C]"])), map.get(&1));
        assert_eq!(Some(&RefCell::from(vec!["[M]"])), map.get(&2));
        assert_eq!(
            Some(&RefCell::from(vec!["[P]", "[D]", "[N]", "[Z]"])),
            map.get(&3)
        );
    }

    #[test]
    fn test_do_command_in_batch_mode() {
        let mut map = HashMap::new();
        map.insert(1, RefCell::from(vec!["[Z]", "[N]"]));
        map.insert(2, RefCell::from(vec!["[M]", "[C]", "[D]"]));
        map.insert(3, RefCell::from(vec!["[P]"]));

        do_command("move 1 from 2 to 1", &map, true);
        do_command("move 3 from 1 to 3", &map, true);
        do_command("move 2 from 2 to 1", &map, true);
        do_command("move 1 from 1 to 2", &map, true);

        assert_eq!(Some(&RefCell::from(vec!["[M]"])), map.get(&1));
        assert_eq!(Some(&RefCell::from(vec!["[C]"])), map.get(&2));
        assert_eq!(
            Some(&RefCell::from(vec!["[P]", "[Z]", "[N]", "[D]"])),
            map.get(&3)
        );
    }
}
