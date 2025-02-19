// Solution 1: can be improved
fn helper(vec: Vec<String>, s: i32) -> bool {
    if vec.len() == 0 && s == 0 {
        return true;
    }

    if s < 0 {
        return false;
    }

    for i in 0..vec.len() {
        let left: String = {
            let mut tmp = "".to_string();
            vec[0..(i + 1)].iter().for_each(|s| {
                tmp += s;
            });
            tmp
        };
        let right: Vec<String> = vec[(i + 1)..].to_vec();
        let left_num: i32 = left.parse().unwrap();

        if helper(right, s - left_num) {
            return true;
        }
    }

    false
}

pub fn punishment_number(n: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=n {
        let sq = i * i;
        let sq_digits: Vec<String> = sq.to_string().chars().map(|c| c.to_string()).collect();
        if helper(sq_digits, i) {
            ans += sq;
        }
    }
    ans
}

// Not a solution: jkd
// pub fn punishment_number(n: i32) -> i32 {
//     let all = [
//         1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756,
//         792, 909, 918, 945, 964, 990, 991, 999, 1000,
//     ];
//     let mut ans = 0;
//     all.iter().for_each(|&x| {
//         if n >= x {
//             ans += x * x;
//         }
//     });
//     ans
// }

fn main() {
    assert_eq!(punishment_number(10), 182);
    assert_eq!(punishment_number(37), 1478);
    println!("All passed!");
}
