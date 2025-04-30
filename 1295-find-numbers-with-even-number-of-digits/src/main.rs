// Solution 1: O(n * logn)
// pub fn find_numbers(nums: Vec<i32>) -> i32 {
//     let mut ans = 0;
//     for i in nums {
//         if i.to_string().len() % 2 == 0 {
//             ans += 1;
//         }
//     }
//     ans
// }

// Solution 2: O(n)
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in nums {
        if i < 10 {
            continue;
        }
        if 100 <= i && i < 1000 {
            continue;
        }
        if 10000 <= i && i < 100000 {
            continue;
        }
        ans += 1;
    }
    ans
}

fn main() {
    assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
    println!("All passed!");
}
