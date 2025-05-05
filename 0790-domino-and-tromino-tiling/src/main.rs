pub fn num_tilings(n: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in 1..=i {
            if j == 1 {
                dp[i] += dp[i - 1];
            } else if j == 2 {
                dp[i] += dp[i - 2];
            } else {
                dp[i] += (dp[i - j] * 2) % MOD;
            }
            dp[i] %= MOD;
        }
    }
    dp[n]
}

fn main() {
    assert_eq!(num_tilings(3), 5);
    assert_eq!(num_tilings(1), 1);
    assert_eq!(num_tilings(4), 11);
    println!("All passed!");
}
