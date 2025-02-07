pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut colors = HashMap::new();
    let mut ans: Vec<i32> = Vec::with_capacity(queries.len());
    let mut frequency = HashMap::new();
    for i in queries {
        let last_color = *colors.get(&i[0]).unwrap_or(&0);
        if last_color != 0 {
            if frequency.get(&last_color) == Some(&1) {
                frequency.remove(&last_color);
            } else {
                frequency.entry(last_color).and_modify(|x| *x -= 1);
            }
        }
        colors.entry(i[0]).and_modify(|x| *x = i[1]).or_insert(i[1]);
        frequency.entry(i[1]).and_modify(|x| *x += 1).or_insert(1);
        ans.push(frequency.iter().count() as i32);
    }

    ans
}

fn main() {
    assert_eq!(
        query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
        vec![1, 2, 2, 3]
    );
    assert_eq!(
        query_results(
            4,
            vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
        ),
        vec![1, 2, 2, 3, 4]
    );
    assert_eq!(
        query_results(
            1,
            vec![vec![0, 1], vec![0, 4], vec![0, 4], vec![0, 1], vec![1, 2]]
        ),
        vec![1, 1, 1, 1, 2]
    );
    println!("All passed!");
}
