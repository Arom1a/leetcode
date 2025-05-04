// Solution 1: O(n) space or worse
// pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
//     use std::collections::HashMap;
//
//     let mut map = HashMap::new();
//
//     for dmn in dominoes {
//         let key = if dmn[0] < dmn[1] {
//             (dmn[0], dmn[1])
//         } else {
//             (dmn[1], dmn[0])
//         };
//         *map.entry(key).or_insert(0) += 1;
//     }
//
//     map.values().map(|&count| count * (count - 1) / 2).sum()
// }

// Solution 2: O(1) space
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut count = [0; 100];
    let mut ans = 0;
    for dmn in dominoes {
        let key = if dmn[0] < dmn[1] {
            dmn[0] * 10 + dmn[1]
        } else {
            dmn[1] * 10 + dmn[0]
        };
        ans += count[key as usize];
        count[key as usize] += 1;
    }
    ans
}

fn main() {
    assert_eq!(
        num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
    assert_eq!(
        num_equiv_domino_pairs(vec![
            vec![1, 2],
            vec![1, 2],
            vec![1, 1],
            vec![1, 2],
            vec![2, 2]
        ]),
        3
    );
    println!("All passed!");
}
