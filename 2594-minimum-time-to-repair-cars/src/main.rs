fn is_time_ok(ranks: &Vec<i32>, cars: i64, t: i64) -> bool {
    let mut can_repair = 0;
    for i in ranks {
        can_repair += (t / *i as i64).isqrt();
        if can_repair >= cars {
            return true;
        }
    }

    false
}

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let cars = cars as i64;
    let mut l = 0;
    let mut r = *ranks.iter().min().unwrap() as i64 * cars * cars + 1;

    while (l - r).abs() > 1 {
        let mid = l + (r - l) / 2;
        if is_time_ok(&ranks, cars, mid) {
            r = mid;
        } else {
            l = mid;
        }
    }

    r
}

fn main() {
    assert!(is_time_ok(&vec![4, 2, 3, 1], 10, 16));
    assert_eq!(repair_cars(vec![4, 2, 3, 1], 10), 16);
    assert_eq!(repair_cars(vec![5, 1, 8], 6), 16);
    println!("All passed!");
}
