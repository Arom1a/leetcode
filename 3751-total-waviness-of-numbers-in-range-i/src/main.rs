fn count_waviness(mut num: i32) -> i32 {
    let mut digits: Vec<i32> = vec![];
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    let mut waviness = 0;
    for i in 1..(digits.len() - 1) {
        let is_peak = digits[i - 1] < digits[i] && digits[i] > digits[i + 1];
        let is_valley = digits[i - 1] > digits[i] && digits[i] < digits[i + 1];
        if is_peak || is_valley {
            waviness += 1;
        }
    }
    waviness
}

pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    let mut ans = 0;
    for i in num1..=num2 {
        ans += count_waviness(i);
    }
    ans
}

fn main() {
    assert_eq!(total_waviness(120, 130), 3);
    assert_eq!(total_waviness(198, 202), 3);
    assert_eq!(total_waviness(4848, 4848), 2);
    println!("All tests passed!");
}
