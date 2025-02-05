use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i: usize = 0;
    let mut j: usize = height.len() - 1;

    let mut ans: i32 = 0;

    while i != j {
        ans = max(ans, (j - i) as i32 * min(height[i], height[j]));
        if height[i] > height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(max_area(vec![1, 1]), 1);
    println!("All passed!");
}
