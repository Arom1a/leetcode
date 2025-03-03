// Solution 1
// pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
//     let mut sm_count: usize = 0;
//     let mut eq_count: usize = 0;
//
//     for i in &nums {
//         match i.cmp(&pivot) {
//             std::cmp::Ordering::Less => sm_count += 1,
//             std::cmp::Ordering::Equal => eq_count += 1,
//             std::cmp::Ordering::Greater => {}
//         }
//     }
//
//     let mut ans = vec![pivot; nums.len()];
//     let mut sm: usize = 0;
//     let mut lg: usize = sm_count + eq_count;
//
//     for i in nums {
//         match i.cmp(&pivot) {
//             std::cmp::Ordering::Less => {
//                 ans[sm] = i;
//                 sm += 1;
//             }
//             std::cmp::Ordering::Equal => {}
//             std::cmp::Ordering::Greater => {
//                 ans[lg] = i;
//                 lg += 1;
//             }
//         }
//     }
//
//     ans
// }

// Solution 2: one loop
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut ans = vec![pivot; nums.len()];
    let mut place_sm: usize = 0;
    let mut place_lg: usize = nums.len() - 1;

    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < nums.len() {
        if nums[i] < pivot {
            ans[place_sm] = nums[i];
            place_sm += 1;
        }
        if nums[j] > pivot {
            ans[place_lg] = nums[j];
            place_lg -= 1;
        }
        i = i.wrapping_add(1);
        j = j.wrapping_sub(1);
    }

    ans
}

fn main() {
    assert_eq!(
        pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
        vec![9, 5, 3, 10, 10, 12, 14]
    );
    assert_eq!(pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
    println!("All passed!");
}
