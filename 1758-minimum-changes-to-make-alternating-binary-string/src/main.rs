pub fn min_operations(s: String) -> i32 {
    let mut start_0 = 0;
    let mut start_1 = 0;

    for (i, c) in s.chars().enumerate() {
        let num = c as usize - '0' as usize;
        if num == (i % 2) {
            start_0 += 1;
        } else {
            start_1 += 1;
        }
    }

    start_0.min(start_1)
}

fn main() {
    assert_eq!(1, min_operations("0100".to_string()));
    assert_eq!(0, min_operations("10".to_string()));
    assert_eq!(2, min_operations("1111".to_string()));
    println!("All tests passed!");
}
