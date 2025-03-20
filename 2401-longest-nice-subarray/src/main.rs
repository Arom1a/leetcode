// Solution 1
// pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
//     let mut l = 0;
//     let mut r = 1;
//     let mut max_len = 1;
//
//     while r < nums.len() {
//         let mut cond = true;
//         for i in l..r {
//             if nums[r] & nums[i] != 0 {
//                 cond = false;
//                 break;
//             }
//         }
//         if cond {
//             r += 1;
//             max_len = max_len.max(r - l);
//         } else {
//             l += 1;
//             r = l;
//         }
//         if max_len == 30 {
//             break;
//         }
//     }
//
//     max_len as _
// }

// Solution 2
pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let mut used_bits = 0;
    let mut max_len = 0;

    while r < nums.len() {
        if (used_bits & nums[r]) != 0 {
            used_bits ^= nums[l];
            l += 1;
            continue;
        }

        used_bits |= nums[r];
        r += 1;
        max_len = max_len.max(r - l);
        if max_len == 30 {
            break;
        }
    }

    max_len as _
}

fn main() {
    assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48]), 3);
    assert_eq!(longest_nice_subarray(vec![3, 8, 48]), 3);
    assert_eq!(longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    assert_eq!(
        longest_nice_subarray(vec![
            84139415, 693324769, 614626365, 497710833, 615598711, 264, 65552, 50331652, 1, 1048576,
            16384, 544, 270532608, 151813349, 221976871, 678178917, 845710321, 751376227,
            331656525, 739558112, 267703680
        ]),
        8
    );
    println!("All passed!");
}
