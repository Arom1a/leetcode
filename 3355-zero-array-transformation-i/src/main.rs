// Solution 1
pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let mut diff: Vec<i32> = vec![0; nums.len() + 1];

    for i in &queries {
        diff[(i[0]) as usize] -= 1;
        diff[(i[1] + 1) as usize] += 1;
    }

    let mut s = 0;
    for i in 0..nums.len() {
        s += diff[i];
        if s + nums[i] > 0 {
            return false;
        }
    }
    true
}

// Solution 2
// pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
//     let mut diff: Vec<i32> = vec![0; nums.len() + 1];
//     let mut dec = 0;
//     let mut k = 0;
//
//     for i in 0..nums.len() {
//         while dec + diff[i] < nums[i] {
//             if k == queries.len() {
//                 return false;
//             }
//
//             let (l, r, val) = (queries[k][0], queries[k][1], 1);
//             k += 1;
//
//             if (r as usize) < i {
//                 continue;
//             }
//
//             diff[(l as usize).max(i)] += val;
//             diff[(r + 1) as usize] -= val;
//         }
//         dec += diff[i];
//     }
//     true
// }

fn main() {
    assert_eq!(is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]), true);
    assert_eq!(
        is_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3], vec![0, 2]]),
        false
    );
    assert_eq!(is_zero_array(vec![7], vec![vec![0, 0]]), false);
    assert_eq!(is_zero_array(vec![2], vec![vec![0, 0], vec![0, 0]]), true);
    println!("All passed!");
}
