// Solution 1
// pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
//     let mut ans = 0;
//     for mut i in low..=high {
//         let len = i.to_string().len();
//         if len % 2 == 1 {
//             continue;
//         }
//
//         let (mut sum1, mut sum2) = (0, 0);
//         for j in 0..len {
//             let digit = i % 10;
//             i /= 10;
//             if j < len / 2 {
//                 sum2 += digit;
//             } else {
//                 sum1 += digit;
//             }
//         }
//         if sum1 == sum2 {
//             ans += 1;
//         }
//     }
//     ans
// }

// Solution 2
pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let mut ans = 0;

    for i in low..=high {
        if i < 100 && i % 11 == 0 {
            ans += 1;
        } else if 1000 <= i && i < 10000 {
            let left = i / 1000 + (i % 1000) / 100;
            let right = (i % 100) / 10 + i % 10;
            if left == right {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    assert_eq!(count_symmetric_integers(1, 100), 9);
    assert_eq!(count_symmetric_integers(1200, 1230), 4);
    println!("All passed!");
}
