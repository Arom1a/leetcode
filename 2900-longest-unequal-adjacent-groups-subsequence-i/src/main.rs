pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut ans = vec![];
    ans.push(words[0].clone());
    let mut last_group = groups[0];
    for i in 1..words.len() {
        if groups[i] != last_group {
            ans.push(words[i].clone());
            last_group = groups[i];
        }
    }
    ans
}

fn main() {
    let test1 = get_longest_subsequence(
        vec!["e".to_string(), "a".to_string(), "b".to_string()],
        vec![0, 0, 1],
    );
    assert!(
        test1 == vec!["e".to_string(), "b".to_string()]
            || test1 == vec!["a".to_string(), "e".to_string()]
    );
    println!("All passed!");
}
