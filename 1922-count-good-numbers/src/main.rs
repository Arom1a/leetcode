// Solution 1: O(n) too slow
// pub fn count_good_numbers(n: i64) -> i32 {
//     const MOD: i64 = 1e9 as i64 + 7;
//     let mut ans = 1;
//     for i in 0..n {
//         match i % 2 {
//             0 => {
//                 ans = ans * 5 % MOD;
//             }
//             _ => {
//                 ans = ans * 4 % MOD;
//             }
//         }
//     }
//     ans as _
// }

// Solution 2: O(log(n)) /w fast exponentiation
fn pow(x: i64, mut y: i64, r#mod: i64) -> i64 {
    let mut ret = 1;
    let mut mul = x;
    while y > 0 {
        if y % 2 == 1 {
            ret = ret * mul % r#mod;
        }
        mul = mul * mul % r#mod;
        y /= 2;
    }
    ret
}

pub fn count_good_numbers(n: i64) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    (pow(5, (n + 1) / 2, MOD) * pow(4, n / 2, MOD) % MOD) as _
}

fn main() {
    assert_eq!(count_good_numbers(1), 5);
    assert_eq!(count_good_numbers(4), 400);
    assert_eq!(count_good_numbers(50), 564908303);
    println!("All passed!");
}
