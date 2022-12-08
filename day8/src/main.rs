use std::fs;
use std::cmp;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<u32>> = input
        .split_terminator("\n")
        .map(|line| line
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect::<Vec<u32>>()
             )
        .collect();

    let visible_trees = find_visible_trees(matrix.clone());
    println!("The number of visible trees is {:?}", visible_trees);
    let scenic_score = find_max_scenic_score(matrix);
    println!("The maximum scenic score is {:?}", scenic_score);
}

fn find_visible_trees(matrix: Vec<Vec<u32>>) -> usize {
    let max_row = matrix.len();
    let max_col = matrix[0].len();
    let mut count = 0;
    for (i, row) in matrix.iter().enumerate() {
        if i > 0 && i < max_row - 1 {
            for (j, _col) in row.iter().enumerate() {
                if j > 0 && j < max_col - 1 {
                    // Check for any instance of 
                    // where the tree is fully visible
                    // from any side

                    let mut visible = false;
                    // check top
                    for t in 0..i {
                        if matrix[t][j] >= matrix[i][j] {
                            visible = false;
                            break;
                        } else {
                            visible = true;
                        }
                    }
                    // check left
                    if !visible {
                        for l in 0..j {
                            if matrix[i][l] >= matrix[i][j] {
                                visible = false;
                                break;
                            } else {
                                visible = true;
                            }
                        }
                    }
                    // check right
                    if !visible {
                        for r in j + 1..max_row {
                            if matrix[i][r] >= matrix[i][j] {
                                visible = false;
                                break;
                            } else {
                                visible = true;
                            }
                        }
                    }
                    // check bottom
                    if !visible {
                        for b in i + 1..max_col {
                            if matrix[b][j] >= matrix[i][j] {
                                visible = false;
                                break;
                            } else {
                                visible = true;
                            }
                        }
                    }
                    if visible {
                        count += 1;
                    }
                }
            }
        }
    }
    // Add together the count of visible middle trees, plus perimeter
    count + max_row + (max_col - 1) + (max_row - 1) + (max_col - 2)
}

fn find_max_scenic_score(matrix: Vec<Vec<u32>>) -> usize {
    let max_row = matrix.len();
    let max_col = matrix[0].len();
    let mut max_scenic_score = 0;
    for (i, row) in matrix.iter().enumerate() {
        if i > 0 && i < max_row - 1 {
            for (j, _col) in row.iter().enumerate() {
                if j > 0 && j < max_col - 1 {
                    // Start at each tree and keep checking
                    // until reaching a tree of the same
                    // height or taller

                    let mut visibilities = vec![0; 4];
                    // check top
                    for t in (0..i).rev() {
                        if matrix[t][j] >= matrix[i][j] {
                            visibilities[0] += 1;
                            break;
                        } else {
                            visibilities[0] += 1;
                        }
                    }
                    // check left
                    for l in (0..j).rev() {
                        if matrix[i][l] >= matrix[i][j] {
                            visibilities[1] += 1;
                            break;
                        } else {
                            visibilities[1] += 1;
                        }
                    }
                    // check right
                    for r in j + 1..max_row {
                        if matrix[i][r] >= matrix[i][j] {
                            visibilities[2] += 1;
                            break;
                        } else {
                            visibilities[2] += 1;
                        }
                    }
                    // check bottom
                    for b in i + 1..max_col {
                        if matrix[b][j] >= matrix[i][j] {
                            visibilities[3] += 1;
                            break;
                        } else {
                            visibilities[3] += 1;
                        }
                    }
                    let product = visibilities
                        .iter()
                        .fold(1, |acc, x| acc * x);
                    max_scenic_score = cmp::max(max_scenic_score, product);
                }
            }
        }
    }
    max_scenic_score
}
