fn longest_palindrome(s_ori: String) -> String {
    let s: Vec<char> = s_ori.chars().collect();
    let n = s.len();
    let mut dp = vec![vec![false; n]; n];

    for l in 1..=n {
        for i in 0..=(n - l) {
            let j = i + l - 1;
            if l == 1 {
                dp[i][j] = true;
                continue;
            }
            if l == 2 && s[i] == s[j] {
                dp[i][j] = true;
                continue;
            }

            if dp[i + 1][j - 1] && s[i] == s[j] {
                dp[i][j] = true;
                continue;
            }
        }
    }

    let mut max_len = 1;
    let mut max_ij = (0, 0);
    for i in 0..n {
        for j in i..n {
            if dp[i][j] && j - i + 1 > max_len {
                max_len = j - i + 1;
                max_ij = (i, j);
            }
        }
    }
    let (i, j) = max_ij;
    s_ori[i..=j].to_string()
}

fn main() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    println!("All tests passed!");
}
