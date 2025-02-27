// Solution 1
// pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//
//     let max = *arr.last().unwrap();
//
//     let mut map = HashMap::new();
//
//     for i in 0..arr.len() {
//         for j in i + 1..arr.len() {
//             let sum = arr[i] + arr[j];
//             if sum > max {
//                 break;
//             }
//
//             let search = arr[j..].binary_search(&sum);
//             match search {
//                 Ok(rst) => {
//                     *map.entry((i, j)).or_default() = rst + j;
//                 }
//                 Err(_) => {}
//             }
//         }
//     }
//
//     let mut ans = 0;
//     let mut cur = 3;
//     for i in &map {
//         let mut key = (i.0.1, *i.1);
//         while let Some(e) = map.get(&key) {
//             key = (key.1, *e);
//             cur += 1;
//         }
//
//         ans = ans.max(cur);
//         cur = 3;
//     }
//
//     ans
// }

// Solution 2: improve solution 1 using hashset (it's actually slower...)
// pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//
//     let max = *arr.last().unwrap() as usize;
//
//     let mut map = HashMap::new();
//     let val2idx: HashMap<usize, usize> = arr
//         .clone()
//         .into_iter()
//         .enumerate()
//         .map(|x| (x.1 as usize, x.0))
//         .collect();
//
//     for i in 0..arr.len() {
//         for j in i + 1..arr.len() {
//             let sum = (arr[i] + arr[j]) as usize;
//             if sum > max {
//                 break;
//             }
//
//             let search = val2idx.get(&sum);
//             match search {
//                 Some(rst) => {
//                     *map.entry((i, j)).or_default() = *rst;
//                 }
//                 None => {}
//             }
//         }
//     }
//
//     let mut ans = 0;
//     let mut cur = 3;
//     for i in &map {
//         let mut key = (i.0.1, *i.1);
//         while let Some(e) = map.get(&key) {
//             key = (key.1, *e);
//             cur += 1;
//         }
//
//         ans = ans.max(cur);
//         cur = 3;
//     }
//
//     ans
// }

// Solution 3: dp
// pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//
//     let mut ans = 0;
//     let mut dp: Vec<Vec<i32>> = vec![vec![1; arr.len()]; arr.len()];
//     let val2idx: HashMap<i32, usize> = arr
//         .clone()
//         .into_iter()
//         .enumerate()
//         .map(|(v, i)| (i, v))
//         .collect();
//     for j in 0..arr.len() {
//         for i in 0..j {
//             let d = arr[j] - arr[i];
//             match val2idx.get(&d) {
//                 Some(pre) => {
//                     dp[i][j] = dp[*pre][i] + 1;
//                 }
//                 None => {
//                     dp[i][j] = 2;
//                 }
//             }
//             ans = ans.max(dp[i][j]);
//         }
//     }
//
//     if ans < 3 { 0 } else { ans }
// }

// Solution 4: dp without hashmap, two_sum instead
pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    use std::cmp::Ordering;

    let mut ans = 0;
    let mut dp: Vec<Vec<i32>> = vec![vec![2; arr.len()]; arr.len()];

    for (i, n) in arr.iter().enumerate() {
        let mut p1: usize = 0;
        let mut p2: usize = i;
        while p1 < p2 {
            match n.cmp(&(arr[p1] + arr[p2])) {
                Ordering::Equal => {
                    dp[p2][i] = dp[p2][i].max(dp[p1][p2]) + 1;
                    ans = ans.max(dp[p2][i]);
                    p1 += 1;
                    p2 -= 1;
                }
                Ordering::Less => {
                    p2 -= 1;
                }
                Ordering::Greater => {
                    p1 += 1;
                }
            }
        }
    }

    if ans < 3 { 0 } else { ans }
}

fn main() {
    assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    assert_eq!(len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
    println!("All passed!");
}
