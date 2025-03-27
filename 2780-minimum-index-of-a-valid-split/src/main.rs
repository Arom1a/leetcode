// Solution 1
fn is_split_valid(i: usize, len: usize, frequency_vec: &Vec<i32>, total_frequency: i32) -> bool {
    if frequency_vec[i] <= (i as i32 + 1) / 2 {
        return false;
    }
    if total_frequency - frequency_vec[i] <= (len as i32 - (i as i32 + 1)) / 2 {
        return false;
    }

    true
}

pub fn minimum_index(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for i in &nums {
        *map.entry(i).or_insert(0) += 1;
    }
    let (&total_frequency, &&dom) = map.iter().map(|i| (i.1, i.0)).max().unwrap();

    let mut frequency_vec = vec![];
    let mut frequency = 0;
    for i in &nums {
        if *i == dom {
            frequency += 1;
        }
        frequency_vec.push(frequency);
    }

    println!("{}", total_frequency);
    println!("{:?}", frequency_vec);

    for i in 0..nums.len() {
        if is_split_valid(i, nums.len(), &frequency_vec, total_frequency) {
            return i as _;
        }
    }
    -1
}

fn main() {
    assert_eq!(minimum_index(vec![1, 2, 2, 2]), 2);
    assert_eq!(minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
    assert_eq!(minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    println!("All passed!");
}
