pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    use std::collections::HashMap;

    let prefix: Vec<i32> = nums
        .iter()
        .map(|x| (*x % modulo == k) as i32)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    let mut ans = 0;
    let mut cnt = HashMap::new();
    cnt.insert(0, 1);
    for i in prefix {
        ans += cnt.get(&((i - k + modulo) % modulo)).unwrap_or(&0);
        *cnt.entry(i % modulo).or_default() += 1;
    }
    ans
}

fn main() {
    assert_eq!(count_interesting_subarrays(vec![3, 2, 4], 2, 1), 3);
    assert_eq!(count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0), 2);
    assert_eq!(count_interesting_subarrays(vec![4, 5], 1, 0), 3);
    println!("All passed!");
}
