pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut cnt = 0;
    for i in arr {
        if i % 2 == 1 {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == 3 {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(three_consecutive_odds(vec![2, 6, 4, 1]), false);
    assert_eq!(
        three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
        true
    );
    println!("All passed!");
}
