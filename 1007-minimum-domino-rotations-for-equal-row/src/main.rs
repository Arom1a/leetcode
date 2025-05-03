// Solution 1: TODO
pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let n = tops.len();
    let mut count_top = [0; 7];
    let mut count_bot = [0; 7];
    let mut same = [0; 7];

    for i in 0..n {
        count_top[tops[i] as usize] += 1;
        count_bot[bottoms[i] as usize] += 1;
        if tops[i] == bottoms[i] {
            same[tops[i] as usize] += 1;
        }
    }

    for x in 1..7 {
        if count_top[x] + count_bot[x] - same[x] == n {
            return (n - std::cmp::max(count_top[x], count_bot[x])) as i32;
        }
    }

    -1
}

fn main() {
    assert_eq!(
        min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
        2
    );
    assert_eq!(
        min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
        -1
    );
    println!("All passed!");
}
