// Solution 1
// pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//     let (mut min_s1, mut min_s2) = (0, 0);
//     let (mut zero1, mut zero2) = (0, 0);
//     for i in nums1 {
//         if i == 0 {
//             zero1 += 1;
//             min_s1 += 1;
//         } else {
//             min_s1 += i as i64;
//         }
//     }
//     for i in nums2 {
//         if i == 0 {
//             zero2 += 1;
//             min_s2 += 1;
//         } else {
//             min_s2 += i as i64;
//         }
//     }
//
//     match min_s1.cmp(&min_s2) {
//         std::cmp::Ordering::Equal => {
//             return min_s1;
//         }
//         std::cmp::Ordering::Less => {
//             if zero1 == 0 {
//                 return -1;
//             }
//             return min_s2;
//         }
//         std::cmp::Ordering::Greater => {
//             if zero2 == 0 {
//                 return -1;
//             }
//             return min_s1;
//         }
//     }
// }

// Solution 2: simplify logic
pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let (mut min_s1, mut min_s2) = (0, 0);
    let (mut zero1, mut zero2) = (0, 0);
    for i in nums1 {
        if i == 0 {
            zero1 += 1;
            min_s1 += 1;
        } else {
            min_s1 += i as i64;
        }
    }
    for i in nums2 {
        if i == 0 {
            zero2 += 1;
            min_s2 += 1;
        } else {
            min_s2 += i as i64;
        }
    }

    if (min_s1 < min_s2 && zero1 == 0) || (min_s1 > min_s2 && zero2 == 0) {
        return -1;
    }
    min_s1.max(min_s2)
}

fn main() {
    assert_eq!(min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
    assert_eq!(min_sum(vec![2, 0, 2, 0], vec![1, 4]), -1);
    println!("All passed!");
}
