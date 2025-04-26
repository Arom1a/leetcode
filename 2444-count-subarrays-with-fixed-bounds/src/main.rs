// Solution 1: TODO
pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let (mut a, mut b, mut c) = (-1, -1, 0);
    nums.into_iter()
        .enumerate()
        .map(|(i, x)| {
            if x < min_k || x > max_k {
                a = i as i32;
                b = i as i32;
                c = 0
            }
            if x == min_k {
                if a < b {
                    c += b - a
                };
                a = i as i32
            }
            if x == max_k {
                if b < a {
                    c += a - b
                };
                b = i as i32
            }
            c as i64
        })
        .sum()
}

fn main() {
    assert_eq!(count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    assert_eq!(count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    println!("All passed!");
}
