// Solution 1
pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let r = grid[0][0] % x;
    for i in grid.iter().flatten() {
        if *i % x != r {
            return -1;
        }
    }
    let mut nums: Vec<i32> = grid.into_iter().flatten().collect();
    nums.sort_unstable();

    let len = nums.len();
    nums.select_nth_unstable(len / 2);
    let expt = nums[len / 2];

    let mut ans = 0;
    for i in &nums {
        ans += (*i - expt).abs() / x;
    }

    ans
}

fn main() {
    assert_eq!(min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
    assert_eq!(min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
    assert_eq!(min_operations(vec![vec![1, 2], vec![3, 4]], 2), -1);
    assert_eq!(min_operations(vec![vec![4, 7], vec![1, 13]], 3), 5);
    println!("All passed!");
}
