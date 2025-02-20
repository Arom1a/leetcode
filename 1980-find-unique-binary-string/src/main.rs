// Solution 1: backtrace
// use std::collections::HashSet;
//
// fn helper(
//     nums_hs: &HashSet<String>,
//     fill: &mut Vec<String>,
//     idx: usize,
//     ans: &mut String,
//     len: usize,
// ) {
//     if idx == len {
//         let curr: String = fill.clone().into_iter().collect();
//         if !nums_hs.contains(&curr) {
//             *ans = curr;
//         }
//     } else {
//         for i in 0..=1 {
//             fill[idx] = i.to_string();
//             helper(nums_hs, fill, idx + 1, ans, len);
//             if ans != "" {
//                 return;
//             }
//             fill[idx] = 2.to_string();
//         }
//     }
// }
//
// pub fn find_different_binary_string(nums: Vec<String>) -> String {
//     let len = nums[0].len();
//     let nums_hs: HashSet<String> = nums.into_iter().collect();
//     let mut fill = vec!["2".to_string(); len];
//     let mut ans = String::new();
//     helper(&nums_hs, &mut fill, 0, &mut ans, len);
//     ans
// }

// Solution 2: concepts of mathematics
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    nums.iter()
        .enumerate()
        .map(|(i, s)| if s.as_bytes()[i] == b'0' { '1' } else { '0' })
        .collect()
}
fn main() {
    assert_eq!(
        find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
        "00"
    );
    assert_eq!(
        find_different_binary_string(vec!["00".to_string(), "10".to_string()]),
        "01"
    );
    assert_eq!(
        find_different_binary_string(vec![
            "111".to_string(),
            "011".to_string(),
            "001".to_string()
        ]),
        "000"
    );
    assert_eq!(
        find_different_binary_string(vec![
            "111".to_string(),
            "011".to_string(),
            "001".to_string(),
            "000".to_string(),
        ]),
        "010"
    );
    println!("All passed!");
}
