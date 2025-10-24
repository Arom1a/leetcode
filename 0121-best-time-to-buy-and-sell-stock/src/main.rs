pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut max = 0;
    for p in prices.iter().rev() {
        if max - p > profit {
            profit = max - p
        }
        if p > &max {
            max = *p;
        }
    }
    profit
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    println!("All passe!");
}
