// Solution 1
pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let max = *arr.last().unwrap();

    let mut map = HashMap::new();

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let sum = arr[i] + arr[j];
            if sum > max {
                break;
            }

            let search = arr[j..].binary_search(&sum);
            match search {
                Ok(rst) => {
                    *map.entry((i, j)).or_default() = rst + j;
                }
                Err(_) => {}
            }
        }
    }

    let mut ans = 0;
    let mut cur = 3;
    for i in &map {
        let mut key = (i.0.1, *i.1);
        while let Some(e) = map.get(&key) {
            key = (key.1, *e);
            cur += 1;
        }

        ans = ans.max(cur);
        cur = 3;
    }

    ans
}

fn main() {
    assert_eq!(len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
    assert_eq!(len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
    println!("All passed!");
}
