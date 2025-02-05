// Solution 1: O(nlogn)
pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums: Vec<i32> = nums.into_iter().filter(|&x| x < k).collect();
    if nums.len() == 0 {
        return 0;
    }
    nums.sort_unstable();

    let mut i: usize = 0;
    let mut j: usize = nums.len() - 1;

    let mut count = 0;

    while i < j {
        match (nums[i] + nums[j]).cmp(&k) {
            std::cmp::Ordering::Equal => {
                count += 1;
                i += 1;
                j -= 1;
            }
            std::cmp::Ordering::Greater => {
                j -= 1;
            }
            std::cmp::Ordering::Less => {
                i += 1;
            }
        }
    }

    count
}

// Solution 2: O(n)
// pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
//     let mut map = std::collections::HashMap::new();
//     let mut count = 0;
//
//     for n in nums {
//         let conjugate = k - n;
//         if let Some(c) = map.get_mut(&conjugate) {
//             if *c > 0 {
//                 *c -= 1;
//                 count += 1;
//                 continue;
//             }
//         }
//         map.entry(n).and_modify(|c| *c += 1).or_insert(1);
//     }
//
//     count
// }

fn main() {
    assert_eq!(max_operations(vec![1, 2, 3, 4], 5), 2);
    assert_eq!(max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    println!("All passed!");
}
