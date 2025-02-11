// Solution 1: Naive
pub fn remove_occurrences(s: String, part: String) -> String {
    let mut char_stack: Vec<char> = vec![];
    s.chars().for_each(|c| {
        char_stack.push(c);
        if char_stack.len() >= part.len() {
            if (*char_stack
                .windows(part.len())
                .next_back()
                .expect("has checked the lenth to fit"))
            .iter()
            .collect::<String>()
            .eq(&part)
            {
                for _ in 0..part.len() {
                    char_stack.pop();
                }
            }
        }
    });

    char_stack.iter().collect()
}

// Solution 2: KMP algorithm

fn main() {
    assert_eq!(
        remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab".to_string()
    );
    assert_eq!(
        remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
        "ab".to_string()
    );
    println!("All passed!");
}
