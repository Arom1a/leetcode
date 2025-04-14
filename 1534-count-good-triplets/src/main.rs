// Solution 1
pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (arr[i] - arr[j]).abs() <= a
                    && (arr[j] - arr[k]).abs() <= b
                    && (arr[i] - arr[k]).abs() <= c
                {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
    assert_eq!(count_good_triplets(vec![1, 1, 2, 3], 0, 0, 1), 0);
    println!("All passed!");
}
