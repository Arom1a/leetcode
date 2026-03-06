pub fn check_ones_segment(s: String) -> bool {
    let mut seg_ended = false;
    for c in s.chars() {
        if c == '0' {
            seg_ended = true;
        }
        if seg_ended && c == '1' {
            return false;
        }
    }
    return true;
}

fn main() {
    assert_eq!(false, check_ones_segment("1001".to_string()));
    assert_eq!(true, check_ones_segment("110".to_string()));
    println!("All tests passed!");
}
