fn test(num: i32, target: i32) -> bool {
    return num <= target;
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut ok = 0;
    let mut ng = nums.len();
    while (ok as i32 - ng as i32).abs() > 1 {
        let m = (ok & ng) + ((ok ^ ng) >> 1);
        if test(nums[m], target) {
            ok = m;
        } else {
            ng = m;
        }
    }

    return if nums[ok] == target { ok as i32 } else { -1 };
}

fn main() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    println!("All tests passed!");
}
