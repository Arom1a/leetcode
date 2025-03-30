// Solution 1
// pub fn partition_labels(s: String) -> Vec<i32> {
//     let s: Vec<char> = s.chars().collect();
//     let mut last_positions: Vec<usize> = vec![];
//     for i in 0..s.len() {
//         for j in (i..s.len()).rev() {
//             if s[i] == s[j] {
//                 last_positions.push(j);
//                 break;
//             }
//         }
//     }
//
//     let mut ans = vec![];
//     let mut bgn_pos = 0;
//     let mut end_pos = 0;
//     for (i, last_pos) in last_positions.iter().enumerate() {
//         end_pos = end_pos.max(*last_pos);
//         if i == end_pos {
//             ans.push((end_pos - bgn_pos + 1) as _);
//             bgn_pos = i + 1;
//         }
//     }
//
//     ans
// }

// Solution 2: improve solution 1
pub fn partition_labels(s: String) -> Vec<i32> {
    let s: Vec<char> = s.chars().collect();
    let mut last_positions: Vec<usize> = vec![0; 26];
    for i in 0..s.len() {
        last_positions[s[i] as usize - 'a' as usize] = i;
    }

    let mut ans = vec![];
    let mut bgn_pos = 0;
    let mut end_pos = 0;
    for i in 0..s.len() {
        end_pos = end_pos.max(last_positions[s[i] as usize - 'a' as usize]);
        if i == end_pos {
            ans.push((end_pos - bgn_pos + 1) as _);
            bgn_pos = i + 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(partition_labels("ababcc".to_string()), vec![4, 2]);
    assert_eq!(
        partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
    assert_eq!(partition_labels("eccbbbbdec".to_string()), vec![10]);
    assert_eq!(partition_labels("eaaaabaaec".to_string()), vec![9, 1]);
    println!("All passed!");
}
