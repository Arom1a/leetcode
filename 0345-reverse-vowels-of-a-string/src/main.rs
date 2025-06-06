pub fn reverse_vowels(s: String) -> String {
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut track_vowel: Vec<char> = vec![];
    let mut track_position: Vec<usize> = vec![];
    for (i, c) in s_vec.iter().enumerate() {
        if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
            track_vowel.push(*c);
            track_position.push(i);
        }
    }
    track_vowel.reverse();
    for i in 0..track_vowel.len() {
        s_vec[track_position[i]] = track_vowel[i];
    }
    s_vec.iter().collect()
}

fn main() {
    assert_eq!(reverse_vowels("IceCreAm".to_string()), "AceCreIm");
    assert_eq!(reverse_vowels("leetcode".to_string()), "leotcede");
    println!("All passed!");
}
