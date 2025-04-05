// Solution 1
// fn help(n: usize, nums: &Vec<i32>, idx: usize, accu: i32, ans: &mut i32) {
//     if idx == n {
//         *ans += accu;
//         return;
//     }
//
//     help(n, nums, idx + 1, accu, ans);
//     help(n, nums, idx + 1, accu ^ nums[idx], ans);
// }
//
// pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
//     let mut ans = 0;
//     help(nums.len(), &nums, 0, 0, &mut ans);
//     ans
// }

// Solution 2
pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in &nums {
        ans |= i;
    }
    ans << (nums.len() - 1)
}

fn main() {
    assert_eq!(subset_xor_sum(vec![1, 3]), 6);
    assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
    assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8,]), 480);
    println!("All passed!");
}
