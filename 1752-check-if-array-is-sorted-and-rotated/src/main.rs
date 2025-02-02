// Solution 1
// pub fn check(nums: Vec<i32>) -> bool {
//     for i in 0..(nums.len() - 1) {
//         if nums[i] > nums[i + 1] {
//             for j in 1..nums.len() {
//                 if nums[(i + j) % nums.len()] > nums[(i + j + 1) % nums.len()] {
//                     println!("{}, {}", i, j);
//                     return false;
//                 }
//             }
//             return true;
//         }
//     }
//     return true;
// }

// Solution 1 rewrite
pub fn check(nums: Vec<i32>) -> bool {
    let mut count = 0;
    if nums[nums.len() - 1] > nums[0] {
        count += 1;
    }
    for i in 0..(nums.len() - 1) {
        if nums[i] > nums[i + 1] {
            count += 1;
        }
        if count >= 2 {
            return false;
        }
    }
    true
}

fn main() {
    assert!(check(vec![3, 4, 5, 1, 2]));
    assert!(!check(vec![2, 1, 3, 4]));
    assert!(check(vec![1, 2, 3]));
    println!("All passed!");
}
