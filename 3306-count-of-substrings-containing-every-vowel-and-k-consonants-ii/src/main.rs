fn at_least_k(v: &Vec<char>, k: i32) -> i64 {
    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;

    let mut vowel_count: [usize; 5] = [0; 5]; // Indexes for 'a', 'e', 'i', 'o', 'u'
    let mut consonant_count = 0;
    let mut vowel_types = 0;

    let is_vowel = |c: char| match c {
        'a' => Some(0),
        'e' => Some(1),
        'i' => Some(2),
        'o' => Some(3),
        'u' => Some(4),
        _ => None,
    };

    while r < v.len() {
        if let Some(idx) = is_vowel(v[r]) {
            if vowel_count[idx] == 0 {
                vowel_types += 1;
            }
            vowel_count[idx] += 1;
        } else {
            consonant_count += 1;
        }

        while consonant_count >= k && vowel_types == 5 {
            ans += (v.len() - r) as i64;
            if let Some(idx) = is_vowel(v[l]) {
                vowel_count[idx] -= 1;
                if vowel_count[idx] == 0 {
                    vowel_types -= 1;
                }
            } else {
                consonant_count -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    ans
}

pub fn count_of_substrings(word: String, k: i32) -> i64 {
    let v: Vec<char> = word.chars().collect();

    at_least_k(&v, k) - at_least_k(&v, k + 1)
}

fn main() {
    assert_eq!(count_of_substrings("aeioqq".to_string(), 1), 0);
    assert_eq!(count_of_substrings("aeiou".to_string(), 1), 0);
    assert_eq!(count_of_substrings("aeiou".to_string(), 0), 1);
    assert_eq!(count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
    assert_eq!(count_of_substrings("iqeaouqi".to_string(), 2), 3);
    assert_eq!(count_of_substrings("buoeia".to_string(), 0), 1);
    assert_eq!(count_of_substrings("aoaiuefi".to_string(), 1), 4);
    println!("All passed!");
}
