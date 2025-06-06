// Solution 1: TODO
pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut cur = 0;
    for &d in &differences {
        cur += d;
        x = x.min(cur);
        y = y.max(cur);
        if y - x > upper - lower {
            return 0;
        }
    }
    (upper - lower) - (y - x) + 1
}

fn main() {
    assert_eq!(number_of_arrays(vec![1, -3, 4], 1, 6), 2);
    assert_eq!(number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
    assert_eq!(number_of_arrays(vec![4, -7, 2], 3, 6), 0);
    println!("All passed!");
}
