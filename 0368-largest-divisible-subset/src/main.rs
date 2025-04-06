// Solution 1: TODO
pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();

    let mut res = vec![Vec::new(); nums.len()];
    for i in 0..nums.len() {
        res[i].push(nums[i]);
    }

    for i in 1..nums.len() {
        for j in 0..i {
            if (nums[i] % nums[j] == 0) && (res[i].len() < res[j].len() + 1) {
                let mut tmp = res[j].to_vec();
                tmp.push(nums[i]);
                res[i] = tmp;
            }
        }
    }

    let (mut max_len, mut max_pos) = (0, 0);
    for i in 0..res.len() {
        if res[i].len() > max_len {
            max_len = res[i].len();
            max_pos = i;
        }
    }
    return res[max_pos].to_vec();
}

fn main() {
    assert!(
        largest_divisible_subset(vec![1, 2, 3]) == vec![1, 2]
            || largest_divisible_subset(vec![1, 2, 3]) == vec![1, 3]
    );
    assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    println!("All passed!");
}
