pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let (mut res, mut imax, mut dmax) = (0, 0, 0);
    for i in 0..n {
        res = res.max(dmax * nums[i] as i64);
        dmax = dmax.max(imax - nums[i] as i64);
        imax = imax.max(nums[i] as i64);
    }

    res
}

fn main() {
    assert_eq!(maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
    assert_eq!(maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
    assert_eq!(maximum_triplet_value(vec![1, 2, 3]), 0);
    assert_eq!(maximum_triplet_value(vec![2, 3, 1]), 0);
    println!("All passed!");
}
