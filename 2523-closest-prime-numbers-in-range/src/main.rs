// Solution 1
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
//     let mut ans = vec![-1, -1];
//
//     let mut last_num = -1;
//     let mut last_diff = i32::MAX;
//
//     for i in left..=right {
//         if is_prime(i) {
//             if last_num == -1 {
//                 last_num = i;
//             } else {
//                 if last_diff > i - last_num {
//                     ans[0] = last_num;
//                     ans[1] = i;
//                     last_diff = i - last_num;
//                     if last_diff == 2 {
//                         break;
//                     }
//                 }
//                 last_num = i;
//             }
//         }
//     }
//
//     ans
// }

// Solution 2: sieve of eratosthenes
fn sieve(upper_lim: i32) -> Vec<bool> {
    let mut sieve = vec![true; upper_lim as usize + 1];
    sieve[0] = false;
    sieve[1] = false;

    {
        let mut i = 2;
        while i * i <= upper_lim {
            if sieve[i as usize] {
                let mut multiple = i * i;
                while multiple <= upper_lim {
                    sieve[multiple as usize] = false;
                    multiple += i;
                }
            }
            i += 1
        }
    }

    sieve
}

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let sieve = sieve(right);
    let mut primes = vec![];
    for i in left..=right {
        if sieve[i as usize] {
            primes.push(i);
        }
    }

    if primes.len() < 2 {
        return vec![-1, -1];
    }

    let mut min_diff = i32::MAX;
    let mut ans = vec![2, -1];

    for i in 1..primes.len() {
        let diff = primes[i] - primes[i - 1];
        if diff < min_diff {
            min_diff = diff;
            ans[0] = primes[i - 1];
            ans[1] = primes[i]
        }
        if min_diff == 2 {
            break;
        }
    }

    ans
}

fn main() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);

    assert_eq!(closest_primes(10, 19), vec![11, 13]);
    assert_eq!(closest_primes(4, 6), vec![-1, -1]);
    assert_eq!(closest_primes(19, 31), vec![29, 31]);
    assert_eq!(closest_primes(84084, 407043), vec![84179, 84181]);
    println!("All passed!");
}
