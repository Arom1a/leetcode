// Solution 1
// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let mut result = String::new();
//     let w1c = word1.chars().collect::<Vec<char>>();
//     let w2c = word2.chars().collect::<Vec<char>>();
//
//     if word1.len() < word2.len() {
//         for i in 0..word1.len() {
//             result.push(w1c[i]);
//             result.push(w2c[i]);
//         }
//         result.push_str(&word2[word1.len()..]);
//     } else {
//         for i in 0..word2.len() {
//             result.push(w1c[i]);
//             result.push(w2c[i]);
//         }
//         result.push_str(&word1[word2.len()..]);
//     }
//
//     result
// }

// Solution 2: 2 ptrs
// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let mut result = String::new();
//     let l1 = word1.len();
//     let l2 = word2.len();
//     let w1c = word1.chars().collect::<Vec<char>>();
//     let w2c = word2.chars().collect::<Vec<char>>();
//
//     let mut i = 0;
//     let mut j = 0;
//     while i < l1 || j < l2 {
//         if i < l1 {
//             result.push(w1c[i]);
//             i += 1;
//         }
//         if j < l2 {
//             result.push(w2c[j]);
//             j += 1;
//         }
//     }
//
//     result
// }

// Solution 3: 1 ptr
// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let mut result = String::new();
//     let l1 = word1.len();
//     let l2 = word2.len();
//     let w1c = word1.chars().collect::<Vec<char>>();
//     let w2c = word2.chars().collect::<Vec<char>>();
//
//     let mut i = 0;
//     while i < std::cmp::max(l1, l2) {
//         if i < l1 {
//             result.push(w1c[i]);
//         }
//         if i < l2 {
//             result.push(w2c[i]);
//         }
//         i += 1;
//     }
//
//     result
// }

// Solution 4: flat_map
// pub fn merge_alternately(word1: String, word2: String) -> String {
//     let l1 = word1.len();
//     let l2 = word2.len();
//     let it1 = word1.chars();
//     let it2 = word2.chars();
//
//     let mut result: String = it1.zip(it2).flat_map(|(c1, c2)| vec![c1, c2]).collect();
//
//
//     if l1 < l2 {
//         result.push_str(&word2[l1..]);
//     } else {
//         result.push_str(&word1[l2..]);
//     }
//
//     result
// }

// Solution 5: while let & next
pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::with_capacity(word1.len() + word2.len());
    let mut it1 = word1.chars();
    let mut it2 = word2.chars();
    while let (Some(c1), Some(c2)) = (it1.next(), it2.next()) {
        result.push(c1);
        result.push(c2);
    }
    result.extend(it1);
    result.extend(it2);
    result
}

fn main() {
    println!(
        "{}",
        merge_alternately("abc".to_string(), "123456789".to_string())
    );
    println!(
        "{}",
        merge_alternately("123456789".to_string(), "abc".to_string())
    );
}
