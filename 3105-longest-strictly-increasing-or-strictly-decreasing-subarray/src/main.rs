use std::cmp::{max, Ordering};

// Solution 1
// pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
//     let mut len = 1;
//     let mut tmp = 1;
//     let mut last: Ordering = Ordering::Equal;
//     nums.windows(2).for_each(|w| {
//         let this: Ordering = w[0].cmp(&w[1]);
//         if this == Ordering::Equal {
//             tmp = 1;
//         } else {
//             if tmp == 1 {
//                 tmp = 2;
//             }
//             if this == last {
//                 tmp += 1;
//             } else {
//                 tmp = 2;
//             }
//         }
//         len = max(len, tmp);
//         last = this;
//     });
//
//     len
// }

// Leetcode idea: sliding window-like
pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut inc_len = 1;
    let mut dec_len = 1;
    let mut max_len = 1;
    nums.windows(2).for_each(|w| {
        match w[0].cmp(&w[1]) {
            Ordering::Equal => {
                inc_len = 1;
                dec_len = 1;
            }
            Ordering::Less => {
                inc_len += 1;
                dec_len = 1;
            }
            Ordering::Greater => {
                inc_len = 1;
                dec_len += 1;
            }
        }
        max_len = max(max(inc_len, dec_len), max_len);
    });

    max_len
}

fn main() {
    assert_eq!(longest_monotonic_subarray(vec![1, 10, 10]), 2);
    assert_eq!(longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    assert_eq!(longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    assert_eq!(longest_monotonic_subarray(vec![3, 2, 1]), 3);
    assert_eq!(longest_monotonic_subarray(vec![1, 7, 8, 4, 1]), 3);
    println!("All passed!");
}
