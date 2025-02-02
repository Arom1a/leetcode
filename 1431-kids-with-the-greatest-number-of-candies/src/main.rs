pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max: i32 = *(candies.iter().max().unwrap_or(&0));
    candies
        .into_iter()
        .map(|x| (max - x) <= extra_candies)
        .collect()
}

fn main() {
    assert_eq!(
        kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
    println!("All passed!");
}
