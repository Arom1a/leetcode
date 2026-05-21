pub fn reorganize_string(s: String) -> String {
    const MAX_CHAR: usize = 30;

    let s: Vec<char> = s.chars().collect();
    let mut n = s.len();
    let mut map = [0; MAX_CHAR];

    for c in s {
        map[(c as u8 - b'a') as usize] += 1;
    }

    let mut ans = String::with_capacity(n);

    while n > 0 {
        let impossible = (n + 1) / 2;

        for i in map {
            if i > impossible {
                return "".to_string();
            }
        }

        let force = n / 2;
        let mut force_idx = None;
        for (idx, &i) in map.iter().enumerate() {
            if i > force {
                force_idx = Some(idx as u8);
                break;
            }
        }

        if let Some(idx) = force_idx {
            let char_idx = (idx + b'a') as char;
            if ans.ends_with(char_idx) {
                return "".to_string();
            }
            ans.push(char_idx);
            map[idx as usize] -= 1;
        } else {
            let mut idx_choose = None;
            for (idx, &i) in map.iter().enumerate() {
                let char_idx = (idx as u8 + b'a') as char;
                if i > 0 && !ans.ends_with(char_idx) {
                    idx_choose = Some(idx as u8);
                    break;
                }
            }

            if let Some(idx) = idx_choose {
                let char_idx = (idx + b'a') as char;
                ans.push(char_idx);
                map[idx as usize] -= 1;
            } else {
                return "".to_string();
            }
        }
        n -= 1;
    }

    ans
}

fn main() {
    assert_eq!(reorganize_string("aab".to_string()), "aba".to_string());
    assert_eq!(reorganize_string("aaab".to_string()), "".to_string());
    assert_eq!(reorganize_string("vvvlo".to_string()), "vlvov".to_string());
    println!("All tests passed!");
}
