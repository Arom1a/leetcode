pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = n as usize;
    let k = k as usize;

    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n + 1];
    for e in times {
        let u = e[0];
        let v = e[1];
        let w = e[2];
        adj[u as usize].push((v as _, w));
    }

    let mut dist = vec![i32::MAX; n + 1];
    let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    let mut done = vec![false; n + 1];
    dist[0] = 0;
    dist[k] = 0;
    pq.push(Reverse((0, k)));

    while let Some(Reverse((d, u))) = pq.pop() {
        if done[u] {
            continue;
        }
        done[u] = true;
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
            }
            pq.push(Reverse((dist[v], v)));
        }
    }

    let max = *dist.iter().max().unwrap();
    if max == i32::MAX { -1 } else { max }
}

fn main() {
    let times: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n: i32 = 4;
    let k: i32 = 2;
    assert_eq!(network_delay_time(times, n, k), 2);
    let times: Vec<Vec<i32>> = vec![vec![1, 2, 1]];
    let n: i32 = 2;
    let k: i32 = 1;
    assert_eq!(network_delay_time(times, n, k), 1);
    let times: Vec<Vec<i32>> = vec![vec![1, 2, 1]];
    let n: i32 = 2;
    let k: i32 = 2;
    assert_eq!(network_delay_time(times, n, k), -1);
    println!("All tests passed!");
}
