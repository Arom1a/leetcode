// version 1: is_ascii_uppercase is slow
// pub fn number_of_special_chars(word: String) -> i32 {
//     let mut first_upper_occurance: [Option<usize>; 26] = [None; 26];
//     for (i, c) in word.chars().enumerate() {
//         if c.is_ascii_uppercase() {
//             let idx = (c as u8 - b'A') as usize;
//             if first_upper_occurance[idx].is_none() {
//                 first_upper_occurance[idx] = Some(i);
//             }
//         }
//     }
//
//     let mut is_special: [bool; 26] = [false; 26];
//     for (i, c) in word.chars().enumerate() {
//         if c.is_ascii_uppercase() {
//             continue;
//         }
//
//         let idx = (c as u8 - b'a') as usize;
//         if let Some(upper_idx) = first_upper_occurance[idx] {
//             if i < upper_idx {
//                 is_special[idx] = true;
//             } else {
//                 is_special[idx] = false;
//             }
//         }
//     }
//
//     is_special.map(|x| x as i32).iter().sum()
// }

// version 2
pub fn number_of_special_chars(word: String) -> i32 {
    let mut first_upper_occurance: [Option<usize>; 26] = [None; 26];
    let mut last_lower_occurance: [Option<usize>; 26] = [None; 26];
    for (i, c) in word.bytes().enumerate() {
        if c < b'a' {
            let idx = (c - b'A') as usize;
            if first_upper_occurance[idx].is_none() {
                first_upper_occurance[idx] = Some(i);
            }
        } else {
            let idx = (c - b'a') as usize;
            last_lower_occurance[idx] = Some(i);
        }
    }
    (0..26)
        .filter(
            |&i| match (last_lower_occurance[i], first_upper_occurance[i]) {
                (Some(l), Some(u)) => l < u,
                _ => false,
            },
        )
        .count() as i32
}

fn main() {
    assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3);
    assert_eq!(number_of_special_chars("abc".to_string()), 0);
    assert_eq!(number_of_special_chars("AbBCab".to_string()), 0);
    println!("All tests passed!");
}
