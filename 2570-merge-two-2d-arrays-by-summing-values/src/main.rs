// Solution 1: hashmap
// pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     use std::collections::HashMap;
//
//     let mut map = HashMap::new();
//
//     for i in nums1 {
//         let id = i[0];
//         let v = i[1];
//
//         *map.entry(id).or_default() += v;
//     }
//     for i in nums2 {
//         let id = i[0];
//         let v = i[1];
//
//         *map.entry(id).or_default() += v;
//     }
//
//     let mut ans: Vec<Vec<i32>> = map.iter().map(|(id, v)| vec![*id, *v]).collect();
//     ans.sort_unstable();
//     ans
// }

// Solution 2: two ptrs
pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut ans = vec![];

    while i1 < nums1.len() && i2 < nums2.len() {
        match nums1[i1][0].cmp(&nums2[i2][0]) {
            std::cmp::Ordering::Equal => {
                ans.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
                i1 += 1;
                i2 += 1;
            }
            std::cmp::Ordering::Less => {
                ans.push(vec![nums1[i1][0], nums1[i1][1]]);
                i1 += 1;
            }
            std::cmp::Ordering::Greater => {
                ans.push(vec![nums2[i2][0], nums2[i2][1]]);
                i2 += 1;
            }
        }
    }
    while i1 < nums1.len() {
        ans.push(nums1[i1].clone());
        i1 += 1;
    }
    while i2 < nums2.len() {
        ans.push(nums2[i2].clone());
        i2 += 1;
    }

    ans
}

fn main() {
    assert_eq!(
        merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]],
        ),
        vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]
    );
    assert_eq!(
        merge_arrays(
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]],
        ),
        vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
    );
    println!("All passed!");
}
