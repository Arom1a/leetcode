// Solution 1: dumb
// pub fn clear_digits(s: String) -> String {
//     let mut char_stack: Vec<usize> = vec![];
//     let mut del: Vec<bool> = vec![false; s.len()];
//
//     for (i, c) in s.chars().enumerate() {
//         if !c.is_digit(10) {
//             char_stack.push(i);
//         } else {
//             let d = char_stack
//                 .pop()
//                 .expect("it is guaranteed to delete all digits");
//             del[d] = true;
//         }
//     }
//
//     s.chars()
//         .enumerate()
//         .filter(|&(i, x)| !x.is_digit(10) && !del[i])
//         .map(|(_, x)| x)
//         .collect()
// }

// Solution 2: cleaner and better
pub fn clear_digits(s: String) -> String {
    let mut char_stack: Vec<char> = vec![];

    for c in s.chars() {
        if c.is_digit(10) {
            char_stack.pop();
        } else {
            char_stack.push(c);
        }
    }

    char_stack.iter().collect()
}

fn main() {
    assert_eq!(clear_digits("abc".to_string()), "abc".to_string());
    assert_eq!(clear_digits("cb34".to_string()), "".to_string());
    assert_eq!(clear_digits("a1b".to_string()), "b".to_string());
    println!("All passed!");
}
