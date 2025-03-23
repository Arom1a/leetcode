// Solution 1
pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    const MOD: u64 = 1e9 as u64 + 7;

    use std::collections::BinaryHeap;

    let n = n as usize;

    let mut adj = vec![vec![]; n];
    for i in roads {
        let (u, v, t) = (i[0] as usize, i[1] as usize, i[2] as u64);
        adj[u as usize].push((v, t));
        adj[v as usize].push((u, t));
    }
    let adj = adj;

    let mut shortest_time = vec![u64::MAX; n];
    let mut path_count = vec![0; n];
    let mut q = BinaryHeap::new();

    shortest_time[0] = 0;
    path_count[0] = 1;
    q.push(Reverse((0, 0)));

    while let Some(Reverse((time, node))) = q.pop() {
        if time > shortest_time[node] {
            continue;
        }

        for &(next, t) in &adj[node] {
            match (time + t).cmp(&shortest_time[next]) {
                std::cmp::Ordering::Less => {
                    shortest_time[next] = time + t;
                    path_count[next] = path_count[node];
                    q.push(Reverse((time + t, next)));
                }
                std::cmp::Ordering::Equal => {
                    path_count[next] = (path_count[next] + path_count[node]) % MOD;
                }
                _ => {}
            }
        }
    }

    path_count[n - 1] as _
}

fn main() {
    assert_eq!(
        count_paths(
            7,
            vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2]
            ]
        ),
        4
    );
    assert_eq!(count_paths(2, vec![vec![1, 0, 10],]), 1);
    println!("All passed!");
}
