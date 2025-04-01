pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp: Vec<i64> = vec![0; questions.len()];
    dp[questions.len() - 1] = questions[questions.len() - 1][0] as _;

    for (i, q) in questions.iter().enumerate().rev().skip(1) {
        let (point, bp) = (q[0] as i64, q[1] as usize);

        let skip_to_idx = (i + bp + 1).min(questions.len());
        let take_point = point
            + if skip_to_idx < questions.len() {
                dp[skip_to_idx]
            } else {
                0
            };

        let skip_point = dp[i + 1];

        dp[i] = take_point.max(skip_point);
    }

    dp[0]
}

fn main() {
    assert_eq!(
        most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]),
        5
    );
    assert_eq!(
        most_points(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ]),
        7
    );
    println!("All passed!");
}
