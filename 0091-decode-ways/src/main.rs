// ways(i) = number of ways to decode string from i..n
//     define for i in [0, n]

// ways(n) = 1
// ways(i) = 0                              if s[i] == 0
//           ways(i+1) + ways(i+2)                  if s[i] == 1 && i+2 in bound
//           ways(i+1) + ways(i+2)                  if s[i] == 2 && 0 <= s[i+1] <= 6 && i+2 in bound of s
//           ways(i+1)                      o.w.
// bypass the bound checks by setting ways(n..n+5) = 0
fn num_decodings(s: String) -> i32 {
    let n = s.len();
    let s: Vec<char> = s.chars().collect();
    let mut dp = vec![None; n + 1];
    fn ways(i: usize, n: usize, s: &Vec<char>, dp: &mut Vec<Option<i32>>) -> i32 {
        if i >= n {
            return (i == n) as _;
        }
        if s[i] == '0' {
            return 0;
        }
        if let Some(ans) = dp[i] {
            return ans;
        }

        let mut ans = ways(i + 1, n, s, dp);
        if i + 1 < n {
            let a = s[i];
            let b = s[i + 1];

            // Valid two-digit codes: 10..=19 and 20..=26
            if a == '1' || (a == '2' && b >= '0' && b <= '6') {
                ans += ways(i + 2, n, s, dp);
            }
        }

        dp[i] = Some(ans);
        ans
    }

    ways(0, n, &s, &mut dp)
}

fn main() {
    assert_eq!(num_decodings("12".to_string()), 2);
    assert_eq!(num_decodings("226".to_string()), 3);
    assert_eq!(num_decodings("06".to_string()), 0);
    println!("All tests passed!");
}
