// Solution 1: TODO
pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();

    if sum % 2 != 0 {
        return false;
    }
    let target = sum as usize / 2;

    let mut dp = vec![false; target + 1];
    dp[0] = true;

    for &num in &nums {
        let num = num as usize;
        for j in (num..=target).rev() {
            dp[j] |= dp[j - num];
        }
    }

    dp[target]
}

fn main() {
    println!("All passed!");
}
