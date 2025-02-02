// Solution 1: %
// pub fn is_array_special(nums: Vec<i32>) -> bool {
//     for i in 0..(nums.len() - 1) {
//         if (nums[i] % 2) == (nums[i + 1] % 2) {
//             return false;
//         }
//     }
//     return true;
// }

// Solution 2: bitwise
pub fn is_array_special(nums: Vec<i32>) -> bool {
    for i in 0..(nums.len() - 1) {
        if (nums[i] & 1) ^ (nums[i + 1] & 1) == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Hello, world!");
}
