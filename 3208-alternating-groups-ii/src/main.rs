// Solution 1: sliding window, O(k) space
// pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
//     let k = k as usize;
//     for i in 0..k - 1 {
//         colors.push(colors[i]);
//     }
//
//     let mut l = 0;
//     let mut r = 1;
//     let mut ans = 0;
//
//     while r < colors.len() {
//         if colors[r - 1] == colors[r] {
//             l = r;
//             r += 1;
//             continue;
//         }
//
//         r += 1;
//
//         if r - l < k {
//             continue;
//         }
//
//         ans += 1;
//         l += 1;
//     }
//
//     ans
// }

// Solution 2: math? and O(1) space
pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = 0;
    let mut last_color = colors[0];
    let mut tmp = 1;

    for i in 0..colors.len() + k - 1 {
        let i = if i >= colors.len() {
            i - colors.len()
        } else {
            i
        };

        if colors[i] == last_color {
            tmp = 1;
            last_color = colors[i];
            continue;
        }

        tmp += 1;

        if tmp >= k {
            ans += 1;
        }
        last_color = colors[i];
    }

    ans
}

fn main() {
    assert_eq!(number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3), 3);
    assert_eq!(
        number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
        2
    );
    assert_eq!(number_of_alternating_groups(vec![1, 1, 0, 1], 4), 0);
    println!("All passed!");
}
