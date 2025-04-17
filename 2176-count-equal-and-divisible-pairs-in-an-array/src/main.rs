pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
    assert_eq!(count_pairs(vec![1, 2, 3, 4], 1), 0);
    println!("All passed!");
}
