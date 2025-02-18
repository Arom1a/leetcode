// Solution 1
fn helper(ans: &mut Vec<usize>, used: &mut Vec<bool>, idx: usize, pattern: &Vec<char>) -> bool {
    if idx == ans.len() {
        return true;
    }

    for num_try in 1..=9 {
        if used[num_try - 1] {
            continue;
        }

        ans[idx] = num_try;
        used[num_try - 1] = true;
        if idx > 0 {
            if !(ans[idx - 1] < ans[idx] && pattern[idx - 1] == 'I')
                && !(ans[idx - 1] > ans[idx] && pattern[idx - 1] == 'D')
            {
                ans[idx] = 0;
                used[num_try - 1] = false;
                continue;
            }
        }
        if helper(ans, used, idx + 1, pattern) {
            return true;
        }
        ans[idx] = 0;
        used[num_try - 1] = false;
    }

    false
}

pub fn smallest_number(pattern: String) -> String {
    let mut ans = vec![0; pattern.len() + 1];
    let mut used = vec![false; 9];
    let ptn: Vec<char> = pattern.chars().collect();

    helper(&mut ans, &mut used, 0, &ptn);

    ans.iter().map(|&n| n.to_string()).collect()
}

fn main() {
    assert_eq!(
        smallest_number("IIIDIDDD".to_string()),
        "123549876".to_string()
    );
    assert_eq!(smallest_number("DDD".to_string()), "4321".to_string());
    println!("All passed!");
}
