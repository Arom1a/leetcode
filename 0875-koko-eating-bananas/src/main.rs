fn can_finish_in_k(piles: &Vec<i32>, h: i32, k: i32) -> bool {
    let mut hour_count = 0;
    for i in piles {
        hour_count += (*i + k - 1) / k;
        if hour_count > h {
            return false;
        }
    }

    true
}

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 0;
    let mut r = piles.iter().max().unwrap() + 1;

    while (l - r).abs() > 1 {
        let mid = l + (r - l) / 2;
        if can_finish_in_k(&piles, h, mid) {
            r = mid;
        } else {
            l = mid;
        }
    }

    r
}

fn main() {
    assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    println!("All passed!");
}
