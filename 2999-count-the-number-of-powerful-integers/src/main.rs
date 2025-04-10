// Solution 1: TODO
pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    let mut low = start.to_string();
    let high = finish.to_string();
    let n = high.len();
    let s_chars: Vec<char> = s.chars().collect();

    // Align digits with leading zeroes
    while low.len() < n {
        low = "0".to_owned() + &low;
    }

    let pre_len = n - s.len();
    let low_chars: Vec<char> = low.chars().collect();
    let high_chars: Vec<char> = high.chars().collect();
    let mut memo: Vec<Option<i64>> = vec![None; n];

    fn dfs(
        i: usize,
        limit_low: bool,
        limit_high: bool,
        pre_len: usize,
        limit: i32,
        s: &[char],
        low: &[char],
        high: &[char],
        memo: &mut [Option<i64>],
    ) -> i64 {
        if i == low.len() {
            return 1;
        }

        if !limit_low && !limit_high {
            if let Some(val) = memo[i] {
                return val;
            }
        }

        let lo = if limit_low {
            low[i].to_digit(10).unwrap()
        } else {
            0
        };
        let hi = if limit_high {
            high[i].to_digit(10).unwrap()
        } else {
            9
        };
        let mut res = 0;

        if i < pre_len {
            for digit in lo..=hi.min(limit as u32) {
                res += dfs(
                    i + 1,
                    limit_low && digit == lo,
                    limit_high && digit == hi,
                    pre_len,
                    limit,
                    s,
                    low,
                    high,
                    memo,
                );
            }
        } else {
            let x = s[i - pre_len].to_digit(10).unwrap();
            if lo <= x && x <= hi.min(limit as u32) {
                res += dfs(
                    i + 1,
                    limit_low && x == lo,
                    limit_high && x == hi,
                    pre_len,
                    limit,
                    s,
                    low,
                    high,
                    memo,
                );
            }
        }

        if !limit_low && !limit_high {
            memo[i] = Some(res);
        }

        res
    }

    dfs(
        0,
        true,
        true,
        pre_len,
        limit,
        &s_chars,
        &low_chars,
        &high_chars,
        &mut memo,
    )
}

fn main() {
    assert_eq!(number_of_powerful_int(1, 6000, 4, "124".to_string()), 5);
    assert_eq!(number_of_powerful_int(15, 215, 6, "10".to_string()), 2);
    assert_eq!(number_of_powerful_int(1000, 2000, 4, "3000".to_string()), 0);
    println!("All passed!");
}
