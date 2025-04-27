pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 2..nums.len() {
        if (nums[i - 2] + nums[i]) * 2 == nums[i - 1] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(count_subarrays(vec![1, 2, 1, 4, 1]), 1);
    assert_eq!(count_subarrays(vec![1, 1, 1]), 0);
    println!("All passed!");
}
