// Solution 1: TODO
pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let n = groups.len();
    let mut dp = vec![1; n];
    let mut prev = vec![-1; n];
    let mut max_index = 0;
    for i in 1..n {
        for j in 0..i {
            if check(&words[i], &words[j]) && dp[j] + 1 > dp[i] && groups[i] != groups[j] {
                dp[i] = dp[j] + 1;
                prev[i] = j as i32;
            }
        }
        if dp[i] > dp[max_index] {
            max_index = i;
        }
    }
    let mut ans = Vec::new();
    let mut i = max_index as i32;
    while i >= 0 {
        ans.push(words[i as usize].clone());
        i = prev[i as usize];
    }
    ans.reverse();
    ans
}

fn check(s1: &String, s2: &String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut diff = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
            if diff > 1 {
                return false;
            }
        }
    }
    diff == 1
}

fn main() {
    let test1 = get_words_in_longest_subsequence(
        vec!["bab".to_string(), "dab".to_string(), "cab".to_string()],
        vec![1, 2, 2],
    );
    assert!(
        test1 == vec!["bab".to_string(), "cab".to_string()]
            || test1 == vec!["bab".to_string(), "dab".to_string()]
    );

    let test2 = get_words_in_longest_subsequence(
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
        vec![1, 2, 3, 4],
    );
    assert!(
        test2
            == vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
    );
    println!("All passed!");
}
