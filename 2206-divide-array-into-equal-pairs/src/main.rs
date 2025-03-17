pub fn divide_array(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    nums.iter()
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(*num).or_insert(0) += 1;
            map
        })
        .values()
        .all(|x| *x % 2 == 0)
}

fn main() {
    assert!(divide_array(vec![3, 2, 3, 2, 2, 2]));
    assert!(!divide_array(vec![1, 2, 3, 4]));
    println!("All passed!");
}
