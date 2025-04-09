pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut min = i32::MAX;
    for i in &nums {
        if *i < k {
            return -1;
        }
        set.insert(*i);
        min = min.min(*i);
    }

    (set.len() - (min == k) as usize) as _
}

fn main() {
    assert_eq!(min_operations(vec![5, 2, 5, 4, 5], 2), 2);
    assert_eq!(min_operations(vec![2, 1, 2], 2), -1);
    assert_eq!(min_operations(vec![9, 7, 5, 3], 1), 4);
    println!("All passed!");
}
