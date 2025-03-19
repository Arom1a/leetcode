pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 2 {
        if nums[i] == 0 {
            for j in 0..=2 {
                nums[i + j] ^= 1;
            }
            count += 1;
        }
    }
    if nums[nums.len() - 2] == 0 || nums[nums.len() - 1] == 0 {
        return -1;
    }
    count
}

fn main() {
    assert_eq!(min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
    assert_eq!(min_operations(vec![0, 1, 1, 1]), -1);
    println!("All passed!");
}
