fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // dp(i): min number of jump to reach idx i
    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;

    for i in 0..n {
        for j in 1..=nums[i] as usize {
            if i + j >= n {
                break;
            }

            dp[i + j] = dp[i + j].min(dp[i] + 1);
        }
    }
    println!("{:?}", dp);

    dp[n - 1]
}

fn main() {
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    println!("All tests passed!");
}
