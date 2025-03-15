pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len() + 1];
    let mut max = 0;
    for (i, n) in nums.iter().enumerate() {
        dp[i + 1] = *n + max;
        max = max.max(dp[i]);
    }

    max.max(dp[dp.len() - 1])
}

fn main() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    println!("All passed!");
}
