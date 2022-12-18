pub fn day_08() {
println!("{}", solve_part_1(include_str!("day_08.txt")));
}

fn solve_part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let row_len = grid.len();
    let col_len = grid[0].len();

    let total_trees_on_edge = col_len * 2 + (row_len * 2 - 4);

    let mut result = total_trees_on_edge;

    for row in 1..row_len - 1 {
        'row_loop: for col in 1..col_len - 1 {
            let tree = grid[row][col];

            // Look up
            for u in (0..row).rev() {
                if tree <= grid[u][col] {
                    // Look right
                    for r in col + 1..col_len {
                        if tree <= grid[row][r] {
                            // Look down
                            for d in row + 1..row_len {
                                if tree <= grid[d][col] {
                                    // Look left
                                    for l in (0..col).rev() {
                                        if tree <= grid[row][l] {
                                            continue 'row_loop;
                                        }
                                    }
                                    result += 1;
                                    continue 'row_loop;
                                }
                            }

                            result += 1;
                            continue 'row_loop;
                        }
                    }
                    result += 1;
                    continue 'row_loop;
                }
            }
            result += 1;
            println!("Up Got {}: [{},{}] - {:?}", tree, row, col, &grid[0..row]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "30373
25512
65332
33549
35390";

        let result = solve_part_1(input);

        assert_eq!(21, result);
    }
}
