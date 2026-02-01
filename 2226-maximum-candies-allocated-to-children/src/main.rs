fn check_num(candies: &Vec<i32>, k: i64, n: i32) -> bool {
    let mut child_num = 0;
    for &i in candies {
        child_num += (i / n) as i64;
        if child_num >= k {
            return true;
        }
    }

    false
}

// Solution 1: standard binary search
// pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
//     let sum: i64 = candies.iter().map(|&x| x as i64).sum();
//     let max = (sum / k) as i32;
//
//     let mut l = 0;
//     let mut r = max + 1;
//     while l != r {
//         let mid = l + (r - l + 1) / 2;
//         if check_num(&candies, k, mid) {
//             l = mid;
//         } else {
//             r = mid - 1;
//         }
//     }
//
//     l
// }

// Solution 2: [meguru method](https://x.com/meguru_comp/status/697008509376835584)
pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let max: i32 = *candies.iter().max().unwrap();
    let mut ok = 0;
    let mut ng = max + 1;
    while (ng - ok).abs() > 1 {
        let m = (ok & ng) + ((ng ^ ok) >> 1);
        if check_num(&candies, k, m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}

fn main() {
    assert_eq!(maximum_candies(vec![5, 8, 6], 3), 5);
    assert_eq!(maximum_candies(vec![2, 5], 11), 0);
    assert_eq!(maximum_candies(vec![4, 7, 5], 16), 1);
    println!("All passed!");
}
