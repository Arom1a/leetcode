// Solution 1
pub fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    n * n + (n - 1) * (n - 1)
}

fn main() {
    assert_eq!(colored_cells(1), 1);
    assert_eq!(colored_cells(2), 5);
    assert_eq!(colored_cells(3), 13);
    println!("All passed!");
}
