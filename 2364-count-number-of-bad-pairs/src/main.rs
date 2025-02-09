pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let diff: Vec<i32> = nums.iter().enumerate().map(|x| *x.1 - x.0 as i32).collect();
    let frequency = diff
        .iter()
        .fold(HashMap::new(), |mut map: HashMap<i32, i32>, &x| {
            *map.entry(x).or_default() += 1;
            map
        });
    let mut good_num: i64 = 0;
    for (_, v) in frequency {
        good_num += v as i64 * (v as i64 - 1) / 2;
    }

    nums.len() as i64 * (nums.len() as i64 - 1) / 2 - good_num
}

fn main() {
    assert_eq!(count_bad_pairs(vec![4, 1, 3, 3]), 5);
    assert_eq!(count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    println!("All passed!");
}
