// TODO(optimizing): each hop we create a new vector. create `prev` and `next` buffers instead to
//                   record the hop distances between rounds, then swap them before the next iteration
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let k = k as usize;

    let mut revadj = vec![vec![]; n];
    for e in flights {
        let s = e[0] as usize;
        let t = e[1] as usize;
        let w = e[2];
        revadj[t].push((s, w));
    }

    let hop = |prev_dist: Vec<i32>, _k: usize| -> Vec<i32> {
        (0..n)
            .map(|v| {
                prev_dist[v].min(
                    revadj[v]
                        .iter()
                        .filter(|&&(u, _w)| prev_dist[u] != i32::MAX)
                        .map(|&(u, w)| prev_dist[u] + w)
                        .min()
                        .unwrap_or(i32::MAX),
                )
            })
            .collect()
    };

    let mut dist = vec![i32::MAX; n];
    dist[src] = 0;

    Some((0..(k + 1)).fold(dist, hop)[dst])
        .filter(|&ans| ans != i32::MAX)
        .unwrap_or(-1)
}

fn main() {
    let n: i32 = 4;
    let flights: Vec<Vec<i32>> = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src: i32 = 0;
    let dst: i32 = 3;
    let k: i32 = 1;
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 700);
    let n: i32 = 3;
    let flights: Vec<Vec<i32>> = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src: i32 = 0;
    let dst: i32 = 2;
    let k: i32 = 1;
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 200);
    let n: i32 = 3;
    let flights: Vec<Vec<i32>> = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    let src: i32 = 0;
    let dst: i32 = 2;
    let k: i32 = 0;
    assert_eq!(find_cheapest_price(n, flights, src, dst, k), 500);
    println!("All tests passed!");
}
