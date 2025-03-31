// Solution 1: TODO
pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let n = weights.len();
    let k = k as usize;
    let mut pair_weights = vec![0; n - 1];
    for i in 0..(n - 1) {
        pair_weights[i] += weights[i] + weights[i + 1];
    }
    pair_weights.sort_unstable();

    let mut ans = 0;
    for i in 0..(k - 1) {
        ans += (pair_weights[n - 2 - i] - pair_weights[i]) as i64;
    }

    ans
}

fn main() {
    assert_eq!(put_marbles(vec![1, 3, 5, 1], 2), 4);
    assert_eq!(put_marbles(vec![1, 3], 2), 0);
    println!("All passed!");
}
