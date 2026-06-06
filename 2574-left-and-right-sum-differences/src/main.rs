pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut lsum = 0;
    let mut rsum: i32 = nums.iter().sum();
    let mut ans = Vec::with_capacity(nums.len());
    for i in nums {
        rsum -= i;
        ans.push((lsum - rsum).abs());
        lsum += i;
    }
    ans
}

fn main() {
    assert_eq!(
        left_right_difference(vec![10, 4, 8, 3]),
        vec![15, 1, 11, 22]
    );
    assert_eq!(left_right_difference(vec![1]), vec![0]);
    println!("All tests passed!");
}
