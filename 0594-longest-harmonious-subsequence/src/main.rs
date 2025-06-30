pub fn find_lhs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for i in nums {
        *freq.entry(i).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<(i32, i32)> = freq.into_iter().collect();
    freq_vec.sort_unstable();
    println!("{:?}", freq_vec);
    let mut ans = 0;
    for i in 1..freq_vec.len() {
        if freq_vec[i].0 - freq_vec[i - 1].0 == 1 {
            ans = ans.max(freq_vec[i - 1].1 + freq_vec[i].1);
        }
    }
    ans
}

fn main() {
    assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(find_lhs(vec![1, 1, 1, 1]), 0);
    println!("All passed!");
}
