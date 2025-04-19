fn good_pairs_strict_below_bound(nums: &Vec<i32>, bound: i32) -> i64 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    let mut ans = 0;
    while l < r {
        if nums[l] + nums[r] >= bound {
            r -= 1;
        } else {
            ans += r as i64 - l as i64;
            l += 1;
        }
    }

    ans
}

pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();
    good_pairs_strict_below_bound(&nums, upper + 1) - good_pairs_strict_below_bound(&nums, lower)
}

fn main() {
    assert_eq!(count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    assert_eq!(count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    println!("All passed!");
}
