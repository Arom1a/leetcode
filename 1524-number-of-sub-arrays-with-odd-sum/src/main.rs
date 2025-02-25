// Solution 1: O(n^2), didn't pass
// pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
//     let parity: Vec<bool> = arr.iter().map(|x| x % 2 != 0).collect();
//     let mut count = 0;
//     for i in 1..=arr.len() {
//         let mut p = false;
//         for j in 0..i {
//             p ^= parity[j];
//         }
//         count += p as i32;
//
//         for j in 0..(arr.len() - i) {
//             p ^= parity[j] ^ parity[j + i];
//             count += p as i32;
//         }
//     }
//
//     count
// }

// Solution 2: O(n) space
// pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
//     let parity: Vec<bool> = arr.iter().map(|x| x % 2 != 0).collect();
//     // .0 is odd count, .1 is even count
//     let mut counts: Vec<(i32, i32)> = vec![(parity[0] as i32, !parity[0] as i32)];
//     for i in 1..arr.len() {
//         match parity[i] {
//             true => {
//                 counts.push((counts[i - 1].1 + 1, counts[i - 1].0));
//             }
//             false => {
//                 counts.push((counts[i - 1].0, counts[i - 1].1 + 1));
//             }
//         }
//     }
//     let mut result = 0;
//     for i in counts {
//         result = (result + i.0) % (1E9 as i32 + 7);
//     }
//     result
// }

// Solution 3: O(1) space
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut odd_count = 0;
    let mut even_count = 1;
    let mut sum = 0;

    for i in arr {
        sum = (sum + i) & 1;
        match sum {
            1 => {
                count += even_count;
                odd_count += 1;
            }
            _ => {
                count += odd_count;
                even_count += 1;
            }
        }
        count %= 1e9 as i32 + 7;
    }

    count
}

fn main() {
    assert_eq!(num_of_subarrays(vec![1, 3, 5]), 4);
    assert_eq!(num_of_subarrays(vec![2, 4, 6]), 0);
    assert_eq!(num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    println!("All passed!");
}
