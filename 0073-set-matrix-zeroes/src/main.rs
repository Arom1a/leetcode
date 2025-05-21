// Solution 1: O(m + n) space
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    use std::collections::HashSet;
    let mut zero_row = HashSet::new();
    let mut zero_col = HashSet::new();
    let h = matrix.len();
    let w = matrix[0].len();
    for row in 0..h {
        for col in 0..w {
            if matrix[row][col] == 0 {
                zero_row.insert(row);
                zero_col.insert(col);
            }
        }
    }

    for i in &zero_row {
        for j in 0..w {
            matrix[*i][j] = 0;
        }
    }
    for i in 0..h {
        for j in &zero_col {
            matrix[i][*j] = 0;
        }
    }
}

fn main() {
    let mut test1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut test1);
    assert_eq!(test1, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

    let mut test2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    set_zeroes(&mut test2);
    assert_eq!(
        test2,
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
    );
    println!("All passed!");
}
