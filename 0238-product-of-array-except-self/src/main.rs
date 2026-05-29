pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(nums.len());

    let mut prod = 1;
    for i in 0..nums.len() {
        prefix.push(prod);
        prod *= nums[i];
    }

    let mut prod = 1;
    for i in (0..nums.len()).rev() {
        prefix[i] *= prod;
        prod *= nums[i];
    }

    prefix
}

fn main() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(
        product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
    println!("All tests passed!");
}
