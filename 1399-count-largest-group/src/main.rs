fn digit_sum(mut n: usize) -> usize {
    let mut ans = 0;
    while n != 0 {
        ans += n % 10;
        n /= 10;
    }
    ans
}

pub fn count_largest_group(n: i32) -> i32 {
    let mut groups = [0; 37];
    for i in 1..=n as usize {
        groups[digit_sum(i)] += 1;
    }
    let max_size = *groups.iter().max().unwrap();
    println!("{:?}", groups);
    let mut ans = 0;
    for i in groups {
        if i == max_size {
            ans += 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(count_largest_group(13), 4);
    assert_eq!(count_largest_group(2), 2);
    println!("All passed!");
}
