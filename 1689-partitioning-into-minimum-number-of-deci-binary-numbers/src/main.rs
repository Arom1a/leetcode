pub fn min_partitions(n: String) -> i32 {
    n.chars().max().unwrap().to_digit(10).unwrap() as i32
}

fn main() {
    assert_eq!(3, min_partitions("32".to_string()));
    assert_eq!(8, min_partitions("82734".to_string()));
    assert_eq!(9, min_partitions("27346209830709182346".to_string()));
    println!("All tests passed!");
}
