pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let front = &[1, 0];
    let back = &[0, 1];

    let mut n0 = 0;
    let mut can_insert = 0;

    for &x in front.iter().chain(&flowerbed).chain(back) {
        if x == 0 {
            n0 += 1;
        } else {
            can_insert += (n0 - 1) / 2;
            n0 = 0;
        }
    }

    if can_insert >= n {
        return true;
    }

    false
}

fn main() {
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(can_place_flowers(vec![0, 1, 0, 1, 0], 0), true);
    assert_eq!(can_place_flowers(vec![0, 1, 0, 1, 0], 1), false);
    assert_eq!(can_place_flowers(vec![0, 0, 1, 0, 1, 0], 1), true);
    println!("All passed!");
}
