// Solution 1: TODO
pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut seen = vec![false; 101];
    for i in (0..nums.len()).rev() {
        if seen[nums[i] as usize] {
            return (i / 3 + 1) as _;
        }
        seen[nums[i] as usize] = true;
    }
    0
}

fn main() {
    assert_eq!(minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]), 2);
    assert_eq!(minimum_operations(vec![4, 5, 6, 4, 4]), 2);
    assert_eq!(minimum_operations(vec![6, 7, 8, 9]), 0);
    println!("All passed!");
}
