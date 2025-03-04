// Solution 1
pub fn check_powers_of_three(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }

    true
}

fn main() {
    assert_eq!(check_powers_of_three(12), true);
    assert_eq!(check_powers_of_three(91), true);
    assert_eq!(check_powers_of_three(21), false);
    assert_eq!(check_powers_of_three(0), false);
    assert_eq!(check_powers_of_three(10), true);
    println!("All passed!");
}
