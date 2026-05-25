// DP: slow, O(n^2)
// pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//     let n = nums.len();
//     let mut dp = vec![1; n];
//     for i in 0..n {
//         for j in 0..i {
//             if nums[j] < nums[i] {
//                 dp[i] = dp[i].max(dp[j] + 1);
//             }
//         }
//     }
//     *dp.iter().max().unwrap()
// }

// binary search: O(n logn)
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut p = vec![];
    for i in nums {
        let lower_bound = p.partition_point(|&x| x < i);
        if lower_bound == p.len() {
            p.push(i);
        } else {
            p[lower_bound] = i;
        }
    }
    p.len() as _
}

fn main() {
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    println!("All tests passed!");
}
