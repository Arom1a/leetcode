// Solution 1: sliding window
// pub fn number_of_substrings(s: String) -> i32 {
//     let mut ans = 0;
//     let mut l = 0;
//     let mut r = 0;
//     let mut c_types = 0;
//     let mut c_count = [0, 0, 0];
//     let v: Vec<char> = s.chars().collect();
//
//     while r < v.len() {
//         match v[r] {
//             'a' => {
//                 if c_count[0] == 0 {
//                     c_types += 1;
//                 }
//                 c_count[0] += 1;
//             }
//             'b' => {
//                 if c_count[1] == 0 {
//                     c_types += 1;
//                 }
//                 c_count[1] += 1;
//             }
//             'c' => {
//                 if c_count[2] == 0 {
//                     c_types += 1;
//                 }
//                 c_count[2] += 1;
//             }
//             _ => panic!("no way"),
//         }
//
//         while c_types == 3 {
//             ans += (v.len() - r) as i32;
//             match v[l] {
//                 'a' => {
//                     c_count[0] -= 1;
//                     if c_count[0] == 0 {
//                         c_types -= 1;
//                     }
//                 }
//                 'b' => {
//                     c_count[1] -= 1;
//                     if c_count[1] == 0 {
//                         c_types -= 1;
//                     }
//                 }
//                 'c' => {
//                     c_count[2] -= 1;
//                     if c_count[2] == 0 {
//                         c_types -= 1;
//                     }
//                 }
//                 _ => panic!("no way"),
//             }
//             l += 1;
//         }
//         r += 1;
//     }
//
//     ans
// }

// Solution 2: iterative
pub fn number_of_substrings(s: String) -> i32 {
    let mut last_pos = [-1, -1, -1];
    let mut ans = 0;
    let v: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        match v[i] {
            'a' => last_pos[0] = i as i32,
            'b' => last_pos[1] = i as i32,
            'c' => last_pos[2] = i as i32,
            _ => panic!("no way"),
        }
        ans += last_pos.iter().min().unwrap() + 1;
    }

    ans
}

fn main() {
    assert_eq!(number_of_substrings("abcabc".to_string()), 10);
    assert_eq!(number_of_substrings("aaacb".to_string()), 3);
    assert_eq!(number_of_substrings("abc".to_string()), 1);
    println!("All passed!");
}
