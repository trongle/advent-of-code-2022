use std::collections::HashSet;

struct Rope {
    head: Position,
    knots: Vec<Knot>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Position(isize, isize);

struct Knot {
    visited_positions: HashSet<Position>,
    current_position: Position,
}

impl Rope {
    fn new(knot_size: u8) -> Self {
        Rope {
            head: Position(0, 0),
            knots: if knot_size == 1 {
                vec![Knot::new()]
            } else {
                let mut knots = Vec::new();
                for _ in 0..knot_size {
                    knots.push(Knot::new());
                }
                knots
            },
        }
    }

    fn move_head(&mut self, command: &str) {
        let sections: Vec<&str> = command.splitn(2, ' ').collect();
        match (sections[0], sections[1].parse::<isize>().unwrap()) {
            ("R", steps) => {
                for _ in 0..steps {
                    self.head.1 += 1;
                    for i in 0..self.knots.len() {
                        self.move_knot(i);
                    }
                }
            }
            ("U", steps) => {
                for _ in 0..steps {
                    self.head.0 -= 1;
                    for i in 0..self.knots.len() {
                        self.move_knot(i);
                    }
                }
            }
            ("L", steps) => {
                for _ in 0..steps {
                    self.head.1 -= 1;
                    for i in 0..self.knots.len() {
                        self.move_knot(i);
                    }
                }
            }
            ("D", steps) => {
                for _ in 0..steps {
                    self.head.0 += 1;
                    for i in 0..self.knots.len() {
                        self.move_knot(i);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn move_knot(&mut self, index: usize) {
        let head_position = if index == 0 {
            &self.head
        } else {
            &self.knots.get(index - 1).unwrap().current_position
        };
        let knot_position = &self.knots.get(index).unwrap().current_position;

        // Check if the tail is on the same row with the head.
        if knot_position.0 == head_position.0 {
            // Check which direction we need to move the
            // tail...?
            //
            // right...
            if knot_position.1 - head_position.1 == -2 {
                let position = Position(knot_position.0, knot_position.1 + 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
            // or left?
            else if knot_position.1 - head_position.1 == 2 {
                let position = Position(knot_position.0, knot_position.1 - 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
        }
        // Check if the tail is on the same column with the head.
        else if knot_position.1 == head_position.1 {
            // Check which direction we need to move the
            // tail...
            //
            // up...
            if knot_position.0 - head_position.0 == 2 {
                let position = Position(knot_position.0 - 1, knot_position.1);
                self.knots.get_mut(index).unwrap().go(position);
            }
            // or down?
            else if knot_position.0 - head_position.0 == -2 {
                let position = Position(knot_position.0 + 1, knot_position.1);
                self.knots.get_mut(index).unwrap().go(position);
            }
        }
        // Check if the tail is on the top diagonal with the head.
        else if knot_position.0 > head_position.0 {
            // Forwarding top.
            if knot_position.0 - head_position.0 == 2 {
                let position = if knot_position.1 < head_position.1 {
                    Position(knot_position.0 - 1, knot_position.1 + 1) // top-right
                } else {
                    Position(knot_position.0 - 1, knot_position.1 - 1) // top-left
                };
                self.knots.get_mut(index).unwrap().go(position);
            }
            // Backward left.
            else if knot_position.1 - head_position.1 == 2 {
                let position = Position(knot_position.0 - 1, knot_position.1 - 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
            // Forwarding right.
            else if knot_position.1 - head_position.1 == -2 {
                let position = Position(knot_position.0 - 1, knot_position.1 + 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
        }
        // Check if the tail is on the bottom diagonal with the head.
        else if knot_position.0 < head_position.0 {
            // Forwarding bottom
            if knot_position.0 - head_position.0 == -2 {
                let position = if knot_position.1 < head_position.1 {
                    Position(knot_position.0 + 1, knot_position.1 + 1) // bottom-right
                } else {
                    Position(knot_position.0 + 1, knot_position.1 - 1) // bottom-left
                };
                self.knots.get_mut(index).unwrap().go(position);
            }
            // Forwarding right
            else if knot_position.1 - head_position.1 == -2 {
                let position = Position(knot_position.0 + 1, knot_position.1 + 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
            // Forwarding left
            else if knot_position.1 - head_position.1 == 2 {
                let position = Position(knot_position.0 + 1, knot_position.1 - 1);
                self.knots.get_mut(index).unwrap().go(position);
            }
        }
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }
}

impl Knot {
    fn new() -> Self {
        let mut set = HashSet::new();
        set.insert(Position(0, 0));

        Knot {
            visited_positions: set,
            current_position: Position(0, 0),
        }
    }

    fn count_visited_positions(&self) -> usize {
        self.visited_positions.len()
    }

    fn go(&mut self, position: Position) {
        self.visited_positions.insert(position.clone());
        self.current_position = position.clone();
    }
}

pub fn day_09() {
    let input = include_str!("day_09.txt");

    println!("{}, {}", part_1(input), part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut rope = Rope::new(1);
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .into_iter()
        .for_each(|l| {
            rope.move_head(l);
        });

    rope.tail().count_visited_positions()
}

fn part_2(input: &str) -> usize {
    let mut rope = Rope::new(9);
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .into_iter()
        .for_each(|l| {
            rope.move_head(l);
        });

    rope.tail().count_visited_positions()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let result = part_1(input);

        assert_eq!(13, result);
    }

    #[test]
    fn test_part_2() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let result = part_2(input);

        assert_eq!(36, result);
    }

    #[test]
    fn create_a_rope() {
        let rope = Rope::new(1);

        assert_eq!(Position(0, 0), rope.head);
        assert_eq!(Position(0, 0), rope.tail().current_position);
        assert_eq!(1, rope.tail().count_visited_positions());
    }

    #[test]
    fn move_head() {
        let mut rope = Rope::new(1);

        rope.move_head("R 4");
        assert_eq!(Position(0, 4), rope.head);
        assert_eq!(Position(0, 3), rope.tail().current_position);
        assert_eq!(4, rope.tail().count_visited_positions());

        rope.move_head("U 4");
        assert_eq!(Position(-4, 4), rope.head);
        assert_eq!(Position(-3, 4), rope.tail().current_position);
        assert_eq!(7, rope.tail().count_visited_positions());

        rope.move_head("L 3");
        assert_eq!(Position(-4, 1), rope.head);
        assert_eq!(Position(-4, 2), rope.tail().current_position);
        assert_eq!(9, rope.tail().count_visited_positions());

        rope.move_head("D 1");
        assert_eq!(Position(-3, 1), rope.head);
        assert_eq!(Position(-4, 2), rope.tail().current_position);
        assert_eq!(9, rope.tail().count_visited_positions());

        rope.move_head("R 4");
        assert_eq!(Position(-3, 5), rope.head);
        assert_eq!(Position(-3, 4), rope.tail().current_position);
        assert_eq!(10, rope.tail().count_visited_positions());

        rope.move_head("D 1");
        assert_eq!(Position(-2, 5), rope.head);
        assert_eq!(Position(-3, 4), rope.tail().current_position);
        assert_eq!(10, rope.tail().count_visited_positions());

        rope.move_head("L 5");
        assert_eq!(Position(-2, 0), rope.head);
        assert_eq!(Position(-2, 1), rope.tail().current_position);
        assert_eq!(13, rope.tail().count_visited_positions());

        rope.move_head("R 2");
        assert_eq!(Position(-2, 2), rope.head);
        assert_eq!(Position(-2, 1), rope.tail().current_position);
        assert_eq!(13, rope.tail().count_visited_positions());
    }
}
