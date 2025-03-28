pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let row_c = grid.len();
    let col_c = grid[0].len();
    let mut ans = vec![0; queries.len()];
    const D: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut sorted_queries: Vec<(i32, usize)> = queries
        .into_iter()
        .enumerate()
        .map(|(i, q)| (q, i))
        .collect();
    sorted_queries.sort_unstable();

    let mut q = BinaryHeap::new();
    let mut visited = vec![vec![false; col_c]; row_c];
    let mut total_points = 0;

    q.push(Reverse((grid[0][0], 0, 0)));
    visited[0][0] = true;

    for (query_value, query_index) in sorted_queries {
        while let Some(Reverse((cell_value, row, col))) = q.pop() {
            if cell_value >= query_value {
                q.push(Reverse((cell_value, row, col))); // Push back as it's not processed
                break;
            }
            total_points += 1;

            for &(row_offset, col_offset) in &D {
                let new_row = row as isize + row_offset;
                let new_col = col as isize + col_offset;

                if new_row >= 0 && new_col >= 0 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if new_row < row_c && new_col < col_c && !visited[new_row][new_col] {
                        q.push(Reverse((grid[new_row][new_col], new_row, new_col)));
                        visited[new_row][new_col] = true;
                    }
                }
            }
        }
        ans[query_index] = total_points;
    }

    ans
}

fn main() {
    assert_eq!(
        max_points(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2]
        ),
        vec![5, 8, 1]
    );
    assert_eq!(
        max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
        vec![0]
    );
    println!("All passed!");
}
