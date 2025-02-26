// Solution 1
pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let prefix_sum: Vec<i32> = nums
        .iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect();
    let mut max = 0;
    let mut min = 0;

    for i in prefix_sum {
        max = max.max(i);
        min = min.min(i);
    }

    max - min
}

fn main() {
    assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    assert_eq!(
        max_absolute_sum(vec![
            -3, -5, -3, -2, -6, 3, 10, -10, -8, -3, 0, 10, 3, -5, 8, 7, -9, -9, 5, -8
        ]),
        27
    );
    assert_eq!(max_absolute_sum(vec![2, -1]), 2);
    println!("All passed!");
}
