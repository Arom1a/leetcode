// Solution 1
// pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
//     let mut schedual: Vec<(i32, bool)> = meetings
//         .into_iter()
//         .map(|day| [(day[0], false), (day[1], true)])
//         .flatten()
//         .collect();
//     schedual.sort_unstable();
//
//     let mut meeting_count = 0;
//     let mut last_day = 0;
//     let mut ans = 0;
//     for &day in &schedual {
//         match meeting_count.cmp(&0) {
//             std::cmp::Ordering::Greater => {}
//             std::cmp::Ordering::Equal => ans += day.0 - (last_day + 1),
//             std::cmp::Ordering::Less => panic!("not possible"),
//         }
//         last_day = day.0;
//
//         if !day.1 {
//             meeting_count += 1;
//         } else {
//             meeting_count -= 1;
//         }
//     }
//     ans += days - last_day;
//
//     ans
// }

// Solution 2: improve 1
pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut last_end = 0;

    meetings.sort_unstable();

    for meeting in &meetings {
        let (start, end) = (meeting[0], meeting[1]);

        if start > last_end + 1 {
            ans += start - (last_end + 1);
        }

        last_end = last_end.max(end);
    }
    ans += days - last_end;

    ans
}


fn main() {
    assert_eq!(
        count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10],]),
        2
    );
    assert_eq!(count_days(5, vec![vec![2, 4], vec![1, 3],]), 1);
    assert_eq!(count_days(6, vec![vec![1, 6],]), 0);
    assert_eq!(
        count_days(
            14,
            vec![
                vec![6, 11],
                vec![7, 13],
                vec![8, 9],
                vec![5, 8],
                vec![3, 13],
                vec![11, 13],
                vec![1, 3],
                vec![5, 10],
                vec![8, 13],
                vec![3, 9]
            ]
        ),
        1
    );
    println!("All passed!");
}
