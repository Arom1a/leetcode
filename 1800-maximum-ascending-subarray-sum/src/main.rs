use std::cmp::{max, Ordering};

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut tmp_sum = nums[0];
    nums.windows(2).for_each(|w| {
        match w[0].cmp(&w[1]) {
            Ordering::Less => {
                tmp_sum += w[1];
            }
            _ => {
                tmp_sum = w[1];
            }
        };
        max_sum = max(max_sum, tmp_sum);
    });

    max_sum
}

fn main() {
    assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    println!("All passed!");
}
