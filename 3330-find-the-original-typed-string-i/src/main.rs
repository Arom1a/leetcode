pub fn possible_string_count(word: String) -> i32 {
    let word: Vec<char> = word.chars().collect();
    let mut ch = word[0];
    let mut segments = vec![];

    let mut count = 1;
    for i in 1..word.len() {
        if word[i] != ch {
            segments.push(count);
            ch = word[i];
            count = 1;
        } else {
            count += 1;
        }
    }
    segments.push(count);

    let mut ans = 1;
    for i in segments {
        ans += i - 1;
    }
    ans
}

fn main() {
    assert_eq!(possible_string_count("abbcccc".to_string()), 5);
    assert_eq!(possible_string_count("abcd".to_string()), 1);
    assert_eq!(possible_string_count("aaaa".to_string()), 4);
    println!("All passed!");
}
