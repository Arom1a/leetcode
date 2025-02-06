// Solution 1
// pub fn tuple_same_product(mut nums: Vec<i32>) -> i32 {
//     nums.sort_unstable();
//     let mut ans = 0;
//     const ARR: [(i32, i32); 3] = [(1, 0), (-1, 1), (1, 0)];
//     for l1 in 0..nums.len() {
//         for r1 in l1..nums.len() {
//             let mut l2: i32 = l1 as i32 + 1;
//             let mut r2: i32 = r1 as i32 - 1;
//             while l2 < r2 {
//                 match (nums[l1 as usize] * nums[r1 as usize])
//                     .cmp(&(nums[l2 as usize] * nums[r2 as usize]))
//                 {
//                     std::cmp::Ordering::Equal => {
//                         ans += 8;
//                         l2 += 1;
//                         r2 -= 1;
//                     }
//                     std::cmp::Ordering::Greater => {
//                         l2 += 1;
//                     }
//                     std::cmp::Ordering::Less => {
//                         r2 -= 1;
//                     }
//                 }
//             }
//         }
//     }
//
//     ans
// }

// Solution 2
pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut p = HashMap::new();
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            p.entry(nums[i] * nums[j])
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    let mut ans: i32 = 0;
    for (_k, v) in p {
        if v > 1 {
            ans += 8 * v * (v - 1) / 2;
        }
    }

    ans
}

fn main() {
    assert_eq!(tuple_same_product(vec![2, 3, 4, 6]), 8);
    assert_eq!(tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    assert_eq!(tuple_same_product(vec![1, 10, 2, 5, 25, 4]), 24);
    assert_eq!(tuple_same_product(vec![2, 3, 4, 6, 8, 12]), 40);
    println!("All passed");
}
