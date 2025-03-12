pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let zero_l = nums.partition_point(|&x| x < 0);
    let zero_r = nums.partition_point(|&x| x < 1);

    let neg_count = zero_l;
    let pos_count = nums.len() - zero_r;

    std::cmp::max(neg_count, pos_count) as _
}

fn main() {
    assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    assert_eq!(maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    assert_eq!(maximum_count(vec![5, 20, 66, 1314]), 4);
    println!("All passed!");
}
