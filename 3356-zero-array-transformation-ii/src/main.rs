pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut diff: Vec<i32> = vec![0; nums.len() + 1];
    let mut dec = 0;
    let mut k = 0;

    for i in 0..nums.len() {
        while dec + diff[i] < nums[i] {
            if k == queries.len() {
                return -1;
            }

            let (l, r, val) = (queries[k][0], queries[k][1], 1);
            k += 1;

            if (r as usize) < i {
                continue;
            }

            diff[(l as usize).max(i)] += val;
            diff[(r + 1) as usize] -= val;
        }
        dec += diff[i];
    }
    k as _
}

fn main() {
    assert_eq!(
        min_zero_array(
            vec![2, 0, 2],
            vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
        ),
        2
    );
    assert_eq!(
        min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
        -1
    );
    assert_eq!(
        min_zero_array(vec![3, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]),
        -1
    );
    println!("All passed!");
}
