fn RLE(str: &Vec<char>) -> Vec<char> {
    let mut char = str[0];
    let mut cnt = 1;
    let mut rtn = vec![];

    for i in 1..str.len() {
        if str[i] == char {
            cnt += 1;
        } else {
            for i in cnt.to_string().chars() {
                rtn.push(i);
            }
            rtn.push(char);
            char = str[i];
            cnt = 1;
        }
    }
    for i in cnt.to_string().chars() {
        rtn.push(i);
    }
    rtn.push(char);

    rtn
}

pub fn count_and_say(n: i32) -> String {
    let mut ans = vec!['1'];

    for _ in 1..n {
        ans = RLE(&ans);
    }

    ans.into_iter().collect()
}

fn main() {
    assert_eq!(count_and_say(4), "1211".to_string());
    assert_eq!(count_and_say(1), "1".to_string());
    println!("All passed!");
}
