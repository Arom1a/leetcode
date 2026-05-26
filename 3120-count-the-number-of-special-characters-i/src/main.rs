// my
// pub fn number_of_special_chars(word: String) -> i32 {
//     let mut up: Vec<bool> = vec![false; 30];
//     let mut lo: Vec<bool> = vec![false; 30];
//
//     #[allow(nonstandard_style)]
//     let ord_A = 'A' as u8 as usize;
//     let ord_a = 'a' as u8 as usize;
//
//     for c in word.chars() {
//         let ord = c as u8 as usize;
//         if ord >= ord_a {
//             lo[ord - ord_a] = true;
//         } else {
//             up[ord - ord_A] = true;
//         }
//     }
//
//     let mut cnt = 0;
//     for idx in 0..30 {
//         if up[idx] && lo[idx] {
//             cnt += 1;
//         }
//     }
//
//     cnt
// }

// editorial
pub fn number_of_special_chars(word: String) -> i32 {
    use std::collections::HashSet;
    let s: HashSet<char> = word.chars().collect();
    let mut cnt = 0;
    for c in 'a'..='z' {
        if s.contains(&c) && s.contains(&c.to_ascii_uppercase()) {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3);
    assert_eq!(number_of_special_chars("abc".to_string()), 0);
    assert_eq!(number_of_special_chars("abBCab".to_string()), 1);
    println!("All tests passed!");
}
