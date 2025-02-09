// Solution 1
// pub fn largest_altitude(gain: Vec<i32>) -> i32 {
//     [0].iter()
//         .chain(gain.iter())
//         .scan(0, |sum, &x| {
//             *sum += x;
//             Some(*sum)
//         })
//         .max()
//         .unwrap()
// }

// Solution 2
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max_altitude = 0;
    let mut current_altitude = 0;

    for g in gain {
        current_altitude += g;
        max_altitude = max_altitude.max(current_altitude);
    }

    max_altitude
}

fn main() {
    assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    println!("All passed!");
}
