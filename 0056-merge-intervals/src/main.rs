pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable();
    let mut ans = vec![];
    let (mut l, mut r) = (intervals[0][0], intervals[0][1]);
    for i in intervals.iter().skip(1) {
        let (li, ri) = (i[0], i[1]);
        if r >= li {
            r = r.max(ri);
        } else {
            ans.push(vec![l, r]);
            (l, r) = (li, ri);
        }
    }
    ans.push(vec![l, r]);
    ans
}

fn main() {
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18],]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18],]
    );
    println!("All tests passed!");
}
