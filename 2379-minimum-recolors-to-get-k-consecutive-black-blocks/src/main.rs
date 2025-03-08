// Solution 1
// pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
//     let block_vec: Vec<char> = blocks.chars().collect();
//
//     block_vec
//         .windows(k as usize)
//         .map(|w| {
//             let mut w_count = 0;
//             for i in w {
//                 if *i == 'W' {
//                     w_count += 1;
//                 }
//             }
//             w_count
//         })
//         .min()
//         .unwrap()
// }

// Solution 2: optimize 1
pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let block_vec: Vec<char> = blocks.chars().collect();
    let mut w_count = 0;
    let mut ans = i32::MAX;
    let mut l = 0;

    for r in 0..block_vec.len() {
        if block_vec[r] == 'W' {
            w_count += 1;
        }

        if r as i32 - l as i32 + 1 == k {
            ans = ans.min(w_count);
            if block_vec[l] == 'W' {
                w_count -= 1;
            }
            l += 1;
        }
    }

    ans
}

fn main() {
    assert_eq!(minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
    assert_eq!(minimum_recolors("WBWBBBW".to_string(), 2), 0);
    println!("All passed!");
}
