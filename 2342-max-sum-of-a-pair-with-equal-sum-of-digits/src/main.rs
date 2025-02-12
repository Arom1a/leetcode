fn digit_sum(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n % 10 + digit_sum(n / 10)
    }
}

// Solution 1
// pub fn maximum_sum(nums: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//
//     let mut ds2count: HashMap<i32, i32> = HashMap::new();
//     let mut ds2num: HashMap<i32, Vec<i32>> = HashMap::new();
//
//     for i in nums {
//         let ds = digit_sum(i);
//         *ds2count.entry(ds).or_default() += 1;
//         ds2num.entry(ds).or_default().push(i);
//     }
//
//     let mut max_sum = -1;
//     for (&ds, &_count) in ds2count.iter().filter(|&x| x.1 >= &2) {
//         let mut max1 = 0;
//         let mut max2 = 0;
//         for i in ds2num.get(&ds).expect("it should exist") {
//             if i > &max1 {
//                 if max1 > max2 {
//                     max2 = max1;
//                 }
//                 max1 = *i;
//             } else if i > &max2 {
//                 max2 = *i;
//             }
//         }
//         max_sum = max_sum.max(max1 + max2);
//     }
//
//     max_sum
// }

// Solution 2: using array instead of hashmap to speed up
// pub fn maximum_sum(nums: Vec<i32>) -> i32 {
//     use std::collections::HashMap;
//
//     let mut ds2count: HashMap<i32, i32> = HashMap::new();
//     let mut ds2max: HashMap<i32, i32> = HashMap::new();
//     let mut max_sum = -1;
//
//     for i in nums {
//         let ds = digit_sum(i);
//         *ds2count.entry(ds).or_default() += 1;
//         if *ds2count.get(&ds).expect("just inserted") > 1 {
//             max_sum = max_sum.max(*ds2max.get(&ds).expect("should be one") + i);
//         }
//         ds2max
//             .entry(ds)
//             .and_modify(|x| *x = (*x).max(i))
//             .or_insert(i);
//     }
//
//     max_sum
// }
pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut ds2count = [0; 81];
    let mut ds2max = [0; 81];
    let mut max_sum = -1;

    for i in nums {
        let ds = digit_sum(i);
        ds2count[ds as usize - 1] += 1;
        if ds2count[ds as usize - 1] > 1 {
            max_sum = max_sum.max(ds2max[ds as usize - 1] + i);
        }
        ds2max[ds as usize - 1] = ds2max[ds as usize - 1].max(i);
    }

    max_sum
}

fn main() {
    assert_eq!(digit_sum(123), 6);
    assert_eq!(digit_sum(1230), 6);
    assert_eq!(digit_sum(4284), 18);
    assert_eq!(digit_sum(5830002), 18);
    assert_eq!(maximum_sum(vec![18, 43, 36, 13, 7]), 54);
    assert_eq!(maximum_sum(vec![10, 12, 19, 14]), -1);
    assert_eq!(
        maximum_sum(vec![
            279, 169, 463, 252, 94, 455, 423, 315, 288, 64, 494, 337, 409, 283, 283, 477, 248, 8,
            89, 166, 188, 186, 128
        ]),
        872
    );
    println!("All passed!");
}
