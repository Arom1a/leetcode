pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut tmp_sum = nums[0];
    nums.iter().skip(1).for_each(|&x| {
        tmp_sum = std::cmp::max(x, tmp_sum + x);

        max_sum = std::cmp::max(max_sum, tmp_sum);
    });

    max_sum
}

fn main() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(vec![1]), 1);
    assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    println!("All passed!");
}
