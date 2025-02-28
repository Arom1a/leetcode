// Solution editorial: TODO, revisit after longest common subsequence (LCS)
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];

    let v1: Vec<char> = str1.chars().collect();
    let v2: Vec<char> = str2.chars().collect();

    for r in 0..=v1.len() {
        dp[r][0] = r;
    }
    for c in 0..=v2.len() {
        dp[0][c] = c;
    }

    for r in 1..=v1.len() {
        for c in 1..=v2.len() {
            if v1[r - 1] == v2[c - 1] {
                dp[r][c] = dp[r - 1][c - 1] + 1;
            } else {
                dp[r][c] = std::cmp::min(dp[r - 1][c], dp[r][c - 1]) + 1;
            }
        }
    }

    let mut ans = "".to_string();
    let mut r = v1.len();
    let mut c = v2.len();

    while r > 0 && c > 0 {
        if v1[r - 1] == v2[c - 1] {
            ans.push(v1[r - 1]);
            r -= 1;
            c -= 1;
            continue;
        }
        match dp[r - 1][c].cmp(&dp[r][c - 1]) {
            std::cmp::Ordering::Less => {
                ans.push(v1[r - 1]);
                r -= 1;
            }
            _ => {
                ans.push(v2[c - 1]);
                c -= 1;
            }
        }
    }
    while r > 0 {
        ans.push(v1[r - 1]);
        r -= 1;
    }
    while c > 0 {
        ans.push(v2[c - 1]);
        c -= 1;
    }

    ans.chars().rev().collect()
}

fn main() {
    assert_eq!(
        shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac".to_string()
    );
    assert_eq!(
        shortest_common_supersequence("cab".to_string(), "abac".to_string()),
        "cabac".to_string()
    );
    assert_eq!(
        shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
        "aaaaaaaa".to_string()
    );
    assert_eq!(
        shortest_common_supersequence("bbbaaaba".to_string(), "bbababbb".to_string()),
        "bbbaaababbb".to_string()
    );
    println!("All passed!");
}
