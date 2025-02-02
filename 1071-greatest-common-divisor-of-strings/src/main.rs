use std::mem::swap;

fn gcd(mut n1: usize, mut n2: usize) -> usize {
    while n1 != 0 {
        if n1 < n2 {
            swap(&mut n1, &mut n2);
        }
        n1 %= n2;
    }
    n2
}

// Solution 1
// pub fn gcd_of_strings(str1: String, str2: String) -> String {
//     let l1 = str1.len();
//     let l2 = str2.len();
//     let l = gcd(l1, l2);
//     for i in (1..=l).rev() {
//         if l % i == 0 {
//             if &str1[0..i] != &str2[0..i] {
//                 continue;
//             }
//             let sub = &str1[0..i];
//             let mut vec1: Vec<bool> = vec![];
//             for j in 0..(l1 / i) {
//                 vec1.push(&str1[(j * i)..((j + 1) * i)] == sub);
//             }
//             let mut vec2: Vec<bool> = vec![];
//             for j in 0..(l2 / i) {
//                 vec2.push(&str2[(j * i)..((j + 1) * i)] == sub);
//             }
//             if vec1.iter().all(|&x| x) && vec2.iter().all(|&x| x) {
//                 return sub.to_string();
//             }
//         }
//     }
//     "".to_string()
// }

// Solution 2
pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
        return String::new();
    }
    str1[..gcd(str1.len(), str2.len())].to_string()
}

fn main() {
    println!(
        "{}",
        gcd_of_strings("ABCDEF".to_string(), "ABC".to_string())
    );
    println!(
        "{}",
        gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
    );
}
