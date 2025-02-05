pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let vs: Vec<char> = s.chars().collect();
    let vt: Vec<char> = t.chars().collect();
    while i < s.len() && j < t.len() {
        if vs[i] == vt[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    if i == s.len() {
        return true;
    }

    false
}

fn main() {
    assert_eq!(
        is_subsequence("abc".to_string(), "ahbgdc".to_string()),
        true
    );
    assert_eq!(
        is_subsequence("axc".to_string(), "ahbgdc".to_string()),
        false
    );
    println!("All passed!");
}
