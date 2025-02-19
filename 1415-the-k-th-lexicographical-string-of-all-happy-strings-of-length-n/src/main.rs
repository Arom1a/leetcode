// Solution 1
fn helper(n: i32, k: i32, tmp: &mut Vec<char>, ans: &mut String, idx: usize, count: &mut i32) {
    if idx > 1 && tmp[idx - 2] == tmp[idx - 1] {
        return;
    }
    if idx as i32 == n {
        *count += 1;
        if *count == k {
            *ans = tmp.iter().collect();
        }
        return;
    }

    for c in ['a', 'b', 'c'] {
        tmp[idx] = c;
        helper(n, k, tmp, ans, idx + 1, count);
        if *ans != "" {
            return;
        }
        tmp[idx] = 'x';
    }
}

pub fn get_happy_string(n: i32, k: i32) -> String {
    if 3_u64.pow(n as u32) < k as u64 {
        return "".to_string();
    }

    let mut tmp: Vec<char> = vec!['x'; n as usize];
    // let mut count = 0;
    let mut ans: String = String::new();

    helper(n, k, &mut tmp, &mut ans, 0, &mut 0);

    ans
}

fn main() {
    assert_eq!(get_happy_string(1, 3), "c");
    assert_eq!(get_happy_string(1, 4), "");
    assert_eq!(get_happy_string(3, 9), "cab");
    assert_eq!(get_happy_string(10, 100), "abacbabacb");
    println!("All passed!");
}
