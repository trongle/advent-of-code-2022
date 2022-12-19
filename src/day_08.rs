pub fn day_08() {
    let tree_grid = TreeGrid::new(include_str!("day_08.txt"));
    println!("{}, {}", part_1(&tree_grid), part_2(&tree_grid));
}

struct TreeGrid {
    trees: Vec<Vec<usize>>,
    row_len: usize,
    col_len: usize,
}

impl TreeGrid {
    pub fn new(input: &str) -> Self {
        let trees = input
            .lines()
            .map(|line| {
                line.split("")
                    .filter(|c| !c.is_empty())
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();
        let row_len = trees.len();
        let col_len = trees[0].len();

        TreeGrid {
            trees,
            row_len,
            col_len,
        }
    }
}

fn part_1(tree_grid: &TreeGrid) -> usize {
    let total_trees_on_edge = tree_grid.col_len * 2 + (tree_grid.row_len * 2 - 4);

    let mut result = total_trees_on_edge;

    for row in 1..tree_grid.row_len - 1 {
        'row_loop: for col in 1..tree_grid.col_len - 1 {
            let tree = tree_grid.trees[row][col];

            // Look up
            for u in (0..row).rev() {
                if tree <= tree_grid.trees[u][col] {
                    // Look right
                    for r in col + 1..tree_grid.col_len {
                        if tree <= tree_grid.trees[row][r] {
                            // Look down
                            for d in row + 1..tree_grid.row_len {
                                if tree <= tree_grid.trees[d][col] {
                                    // Look left
                                    for l in (0..col).rev() {
                                        if tree <= tree_grid.trees[row][l] {
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
        }
    }

    result
}

fn part_2(tree_grid: &TreeGrid) -> usize {
    let mut scenic_points: Vec<usize> = Vec::new();
    for row in 0..tree_grid.row_len {
        for col in 0..tree_grid.col_len {
            let tree = tree_grid.trees[row][col];
            let mut up_side_point = 0;
            let mut right_side_point = 0;
            let mut down_side_point = 0;
            let mut left_side_point = 0;

            // Look up
            for u in (0..row).rev() {
                if tree <= tree_grid.trees[u][col] {
                    up_side_point += 1;
                    break;
                }
                up_side_point += 1;
            }

            // Look right
            for r in col + 1..tree_grid.col_len {
                if tree <= tree_grid.trees[row][r] {
                    right_side_point += 1;
                    break;
                }
                right_side_point += 1;
            }

            // Look down
            for d in row + 1..tree_grid.row_len {
                if tree <= tree_grid.trees[d][col] {
                    down_side_point += 1;
                    break;
                }
                down_side_point += 1;
            }

            // Look left
            for l in (0..col).rev() {
                if tree <= tree_grid.trees[row][l] {
                    left_side_point += 1;
                    break;
                }
                left_side_point += 1;
            }

            match (
                up_side_point,
                right_side_point,
                down_side_point,
                left_side_point,
            ) {
                (0, 0, _, _) => scenic_points.push(down_side_point * left_side_point),
                (_, 0, 0, _) => scenic_points.push(up_side_point * left_side_point),
                (_, _, 0, 0) => scenic_points.push(up_side_point * right_side_point),
                (0, _, _, 0) => scenic_points.push(right_side_point * down_side_point),
                (0, _, _, _) => {
                    scenic_points.push(right_side_point * down_side_point * left_side_point)
                }
                (_, 0, _, _) => {
                    scenic_points.push(up_side_point * down_side_point * left_side_point)
                }
                (_, _, 0, _) => {
                    scenic_points.push(up_side_point * right_side_point * left_side_point)
                }
                (_, _, _, 0) => {
                    scenic_points.push(up_side_point * right_side_point * down_side_point)
                }
                (_, _, _, _) => scenic_points
                    .push(up_side_point * right_side_point * down_side_point * left_side_point),
            }
        }
    }

    *scenic_points.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let tree_grid = TreeGrid::new("30373\n25512\n65332\n33549\n35390");

        let result = part_1(&tree_grid);

        assert_eq!(21, result);
    }

    #[test]
    fn test_part_2() {
        let tree_grid = TreeGrid::new("30373\n25512\n65332\n33549\n35390");

        let result = part_2(&tree_grid);

        assert_eq!(16, result);
    }
}
