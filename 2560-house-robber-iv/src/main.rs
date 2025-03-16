fn is_capability_ok(nums: &Vec<i32>, k: i32, cap: i32) -> bool {
    let mut count = 0;
    let mut skip = false;
    for i in nums {
        if skip {
            skip = false;
            continue;
        }
        if *i <= cap {
            count += 1;
            if count >= k {
                return true;
            }
            skip = true;
        }
    }

    false
}

pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.iter().max().unwrap() + 1;

    while (l - r).abs() > 1 {
        let mid = (l + r) / 2;
        if is_capability_ok(&nums, k, mid) {
            r = mid;
        } else {
            l = mid;
        }
    }

    r
}

fn main() {
    assert_eq!(min_capability(vec![2, 3, 5, 9], 2), 5);
    assert_eq!(min_capability(vec![2, 7, 9, 3, 1], 2), 2);
    println!("All passed!");
}
