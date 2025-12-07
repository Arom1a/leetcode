pub fn count_odds(low: i32, high: i32) -> i32 {
    (high + 1) / 2 - low / 2
}

fn main() {
    assert_eq!(count_odds(3, 4), 1);
    assert_eq!(count_odds(3, 5), 2);
    assert_eq!(count_odds(4, 5), 1);
    assert_eq!(count_odds(4, 6), 1);
    assert_eq!(count_odds(4, 4), 0);
    println!("All tests passed!");
}
