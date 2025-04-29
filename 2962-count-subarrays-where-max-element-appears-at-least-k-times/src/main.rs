pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max = *nums.iter().max().unwrap();
    let mut l = 0;
    let mut cnt = 0;
    let mut ans = 0;

    for r in 0..nums.len() {
        if nums[r] == max {
            cnt += 1;
        }

        while l <= r && cnt >= k {
            if nums[l] == max {
                cnt -= 1;
            }
            l += 1;
        }
        ans += l as i64;
    }

    ans
}

fn main() {
    assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    assert_eq!(count_subarrays(vec![1, 4, 2, 1], 3), 0);
    println!("All passed!");
}
