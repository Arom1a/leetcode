fn match_singleton(s: char, p: char) -> bool {
    p == '.' || s == p
}

enum Pattern {
    Singleton(char),
    Star(char),
}

fn is_match(s_ori: String, p_ori: String) -> bool {
    let s: Vec<char> = s_ori.chars().collect();
    let pc: Vec<char> = p_ori.chars().collect();
    let mut p: Vec<Pattern> = vec![];
    for i in 0..pc.len() {
        if pc[i] == '*' {
            continue;
        }
        if i == pc.len() - 1 {
            p.push(Pattern::Singleton(pc[i]));
            continue;
        }

        if pc[i + 1] == '*' {
            p.push(Pattern::Star(pc[i]));
        } else {
            p.push(Pattern::Singleton(pc[i]));
        }
    }
    let p = p;
    let n_s = s.len();
    let n_p = p.len();

    fn run_dp(i: usize, j: usize, s: &Vec<char>, p: &Vec<Pattern>) -> bool {
        if j == 0 {
            return i == 0;
        }
        if i == 0 {
            return matches!(p[j - 1], Pattern::Star(_)) && run_dp(i, j - 1, s, p);
        }

        match p[j - 1] {
            Pattern::Singleton(pc) => match_singleton(s[i - 1], pc) && run_dp(i - 1, j - 1, s, p),
            Pattern::Star(pc) => {
                run_dp(i, j - 1, s, p) || (match_singleton(s[i - 1], pc) && run_dp(i - 1, j, s, p))
            }
        }
    }

    run_dp(n_s, n_p, &s, &p)
}

fn main() {
    assert!(!is_match("aa".to_string(), "a".to_string()));
    assert!(is_match("aa".to_string(), "a*".to_string()));
    assert!(is_match("ab".to_string(), ".*".to_string()));
    assert!(!is_match("aaa".to_string(), "aaaa".to_string()));
    println!("All tests passed!");
}
