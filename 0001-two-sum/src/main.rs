pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
    nums.sort_unstable_by_key(|&x| x.1);
    let (mut l, mut r) = (0 as usize, nums.len() - 1);

    while l < r {
        let s = nums[l].1 + nums[r].1;

        match s.cmp(&target) {
            std::cmp::Ordering::Equal => {
                return vec![nums[l].0.try_into().unwrap(), nums[r].0.try_into().unwrap()];
            }
            std::cmp::Ordering::Greater => {
                r -= 1;
            }
            std::cmp::Ordering::Less => {
                l += 1;
            }
        }
    }

    panic!("there is sure to be an answer");
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    println!("All passed!");
}
