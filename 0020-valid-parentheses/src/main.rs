pub fn is_valid(s: String) -> bool {
    let mut cnt = vec![];
    for c in s.chars() {
        match c {
            '(' => cnt.push(')'),
            '{' => cnt.push('}'),
            '[' => cnt.push(']'),
            closing => {
                if Some(closing) != cnt.pop() {
                    return false;
                }
            }
        }
    }
    cnt.is_empty()
}

fn main() {
    assert_eq!(is_valid("()".to_string()), true);
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("(]".to_string()), false);
    assert_eq!(is_valid("([])".to_string()), true);
    assert_eq!(is_valid("([)]".to_string()), false);
    println!("All tests passed!");
}
