fn digit_sum(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n % 10 + digit_sum(n / 10)
    }
}

// Solution 1
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut ds2count: HashMap<i32, i32> = HashMap::new();
    let mut ds2num: HashMap<i32, Vec<i32>> = HashMap::new();

    for i in nums {
        let ds = digit_sum(i);
        *ds2count.entry(ds).or_default() += 1;
        ds2num.entry(ds).or_default().push(i);
    }

    let mut max_sum = -1;
    for (&ds, &_count) in ds2count.iter().filter(|&x| x.1 >= &2) {
        let mut max1 = 0;
        let mut max2 = 0;
        for i in ds2num.get(&ds).expect("it should exist") {
            if i > &max1 {
                if max1 > max2 {
                    max2 = max1;
                }
                max1 = *i;
            } else if i > &max2 {
                max2 = *i;
            }
        }
        max_sum = max_sum.max(max1 + max2);
    }

    max_sum
}

fn main() {
    assert_eq!(digit_sum(123), 6);
    assert_eq!(digit_sum(1230), 6);
    assert_eq!(digit_sum(4284), 18);
    assert_eq!(digit_sum(5830002), 18);
    // assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    println!("All passed!");
}
