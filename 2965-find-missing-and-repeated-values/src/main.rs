// Solution 1
// pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
//     let n = grid.len();
//     let mut ans = vec![0, 0];
//     let mut seen = vec![false; n * n];
//
//     for i in grid {
//         for j in i {
//             if seen[(j - 1) as usize] == true {
//                 ans[0] = j;
//             } else {
//                 seen[(j - 1) as usize] = true;
//             }
//         }
//     }
//
//     for i in 0..(n * n) {
//         if seen[i] == false {
//             ans[1] = (i + 1) as i32;
//             break;
//         }
//     }
//
//     ans
// }

// Solution 2: math, O(1) space
pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len() as i64;
    let n_sqr = n * n;

    let expt_sum = n_sqr * (n_sqr + 1) / 2;
    let expt_sqr = expt_sum * (2 * n_sqr + 1) / 3;

    let (actl_sum, actl_sqr) =
        grid.iter()
            .flatten()
            .fold((0_i64, 0_i64), |(actl_sum, actl_sqr), x| {
                let x = *x as i64;
                (actl_sum + x, actl_sqr + (x * x))
            });

    let num_diff = actl_sum - expt_sum;
    let num_sum = (actl_sqr - expt_sqr) / num_diff;

    vec![
        ((num_sum + num_diff) / 2) as _,
        ((num_sum - num_diff) / 2) as _,
    ]
}

fn main() {
    assert_eq!(
        find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
        vec![2, 4]
    );
    assert_eq!(
        find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]),
        vec![9, 5]
    );
    println!("All passed!");
}
