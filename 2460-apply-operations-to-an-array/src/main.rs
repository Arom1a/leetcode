// Solution 1: straightforward logic
// pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
//     for i in 0..nums.len() - 1 {
//         if nums[i] == nums[i + 1] {
//             nums[i] *= 2;
//             nums[i + 1] = 0;
//         }
//     }
//
//     let mut idx = 0;
//     let mut ans = vec![0; nums.len()];
//     for i in nums {
//         if i != 0 {
//             ans[idx] = i;
//             idx += 1;
//         }
//     }
//
//     ans
// }

// Solution 2: in-place & one iter
pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let mut place_idx = 0;

    for i in 0..nums.len() {
        if i < nums.len() - 1 && nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }

        if nums[i] != 0 {
            nums[place_idx] = nums[i];
            if i != place_idx {
                nums[i] = 0;
            }
            place_idx += 1;
        }
    }

    nums
}

fn main() {
    assert_eq!(
        apply_operations(vec![1, 2, 2, 1, 1, 0]),
        vec![1, 4, 2, 0, 0, 0]
    );
    assert_eq!(apply_operations(vec![0, 1]), vec![1, 0]);
    println!("All passed!");
}
