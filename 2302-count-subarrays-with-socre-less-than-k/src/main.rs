// Solution 1
// pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
//     let (mut l, mut r) = (0, 0);
//     let mut sum: i64 = 0;
//     let mut score: i64 = 0;
//
//     let mut ans = 0;
//     while r < nums.len() {
//         sum += nums[r] as i64;
//         score += sum + nums[r] as i64 * (r - l) as i64;
//
//         while score as i64 >= k && l <= r {
//             sum -= nums[l] as i64;
//             score -= sum + nums[l] as i64 * ((r - l) as i64 + 1);
//             l += 1;
//         }
//         ans += (r - l) as i64 + 1;
//         r += 1;
//     }
//     ans
// }

// Solution 2: improve 1: do not need `score`
pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let (mut l, mut r) = (0, 0);
    let mut sum: i64 = 0;

    let mut ans = 0;
    while r < nums.len() {
        sum += nums[r] as i64;

        while sum * ((r - l) as i64 + 1) >= k && l <= r {
            sum -= nums[l] as i64;
            l += 1;
        }
        ans += (r - l) as i64 + 1;
        r += 1;
    }
    ans
}

fn main() {
    assert_eq!(count_subarrays(vec![2, 1, 4, 3, 5], 10), 6);
    assert_eq!(count_subarrays(vec![1, 1, 1], 5), 5);
    println!("All passed!");
}
