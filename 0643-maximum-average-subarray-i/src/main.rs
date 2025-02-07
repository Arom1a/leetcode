// Solution 1: dumb
// pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
//     let max_subarray_sum: i32 = nums
//         .windows(k as usize)
//         .max_by(|w1, w2| w1.iter().sum::<i32>().cmp(&w2.iter().sum()))
//         .unwrap_or(&[])
//         .iter()
//         .sum::<i32>();
//
//     max_subarray_sum as f64 / k as f64
// }

// Solution 2
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sums: Vec<i32> = Vec::with_capacity(nums.len() - k as usize + 1);

    let mut s = 0;
    for i in 0..(k as usize) {
        s += nums[i];
    }
    sums.push(s);
    for i in 0..(nums.len() - k as usize) {
        s -= nums[i];
        s += nums[i + k as usize];
        sums.push(s);
    }

    *sums.iter().max().unwrap() as f64 / k as f64
}

fn main() {
    assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75000);
    assert_eq!(find_max_average(vec![5], 1), 5.00000);
    println!("All passed!");
}
