// Solution 1: bad, TODO: O(n)
pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let mut even_nums = HashSet::new();
    for i in 0..digits.len() {
        for j in 0..digits.len() {
            for k in 0..digits.len() {
                if i == j || j == k || k == i {
                    continue;
                }
                let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                if num >= 100 && num % 2 == 0 {
                    even_nums.insert(num);
                }
            }
        }
    }
    let mut ans: Vec<i32> = even_nums.into_iter().collect();
    ans.sort_unstable();
    ans
}

fn main() {
    assert_eq!(
        find_even_numbers(vec![2, 1, 3, 0]),
        vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
    );
    assert_eq!(
        find_even_numbers(vec![2, 2, 8, 8, 2]),
        vec![222, 228, 282, 288, 822, 828, 882]
    );
    assert_eq!(find_even_numbers(vec![3, 7, 5]), vec![]);
    println!("All passed!");
}
