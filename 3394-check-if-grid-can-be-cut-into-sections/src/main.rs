// Solution 2
fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
    check_cuts(&mut rectangles, 0) || check_cuts(&mut rectangles, 1)
}

fn check_cuts(rectangles: &mut Vec<Vec<i32>>, dim: usize) -> bool {
    rectangles.sort_by_key(|rect| rect[dim]);

    let mut gap_count = 0;
    let mut furthest_end = rectangles[0][dim + 2];

    for rect in rectangles.iter().skip(1) {
        if furthest_end <= rect[dim] {
            gap_count += 1;
        }
        furthest_end = furthest_end.max(rect[dim + 2]);
    }

    gap_count >= 2
}

fn main() {
    assert_eq!(
        check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5],
            ]
        ),
        true
    );
    assert_eq!(
        check_valid_cuts(
            4,
            vec![
                vec![0, 0, 1, 1],
                vec![2, 0, 3, 4],
                vec![0, 2, 2, 3],
                vec![3, 0, 4, 3],
            ]
        ),
        true
    );
    assert_eq!(
        check_valid_cuts(
            4,
            vec![
                vec![0, 2, 2, 4],
                vec![1, 0, 3, 2],
                vec![2, 2, 3, 4],
                vec![3, 0, 4, 2],
                vec![3, 2, 4, 4],
            ]
        ),
        false
    );
    println!("All passed!");
}
