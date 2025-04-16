pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::HashMap;

    let n = nums.len();
    let (mut ans, mut count) = (0, 0);
    let mut map = HashMap::new();

    let (mut l, mut r) = (0, 0);
    while l < n {
        while count < k && r < n {
            *map.entry(nums[r]).or_insert(0) += 1;
            count += map.get(&nums[r]).unwrap() - 1;
            r += 1;
        }
        if count >= k {
            ans += (n - r + 1) as i64;
        }

        match map.get_mut(&nums[l]) {
            Some(v) => {
                *v -= 1;
                count -= *v;
            }
            _ => {}
        }
        l += 1;
    }

    ans
}

fn main() {
    assert_eq!(count_good(vec![1, 1, 1, 1, 1], 10), 1);
    assert_eq!(count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    println!("All passed!");
}
