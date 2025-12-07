pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut s, mut f) = (nums[0], nums[nums[0] as usize]);
    while s != f {
        (s, f) = (nums[s as usize], nums[nums[f as usize] as usize]);
    }
    s = 0;
    while s != f {
        (s, f) = (nums[s as usize], nums[f as usize]);
    }
    s
}

fn main() {
    assert_eq!(find_duplicate(vec![3, 1, 3, 4, 3]), 3);
    assert_eq!(find_duplicate(vec![3, 4, 3, 1, 2]), 3);
    assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(find_duplicate(vec![3, 3, 3, 3, 3]), 3);
    assert_eq!(find_duplicate(vec![4, 3, 1, 4, 2]), 4);
    assert_eq!(find_duplicate(vec![1, 3, 4, 2, 1]), 1);
    assert_eq!(
        find_duplicate(vec![
            // 0,  1   2   3  4   5  6   7  8  9  10 11  12  13 14  15 16  17  18  19
            18, 13, 14, 17, 9, 19, 7, 17, 4, 6, 17, 5, 11, 10, 2, 15, 8, 12, 16, 17
        ]),
        17
    );
    println!("All passed!")
}
