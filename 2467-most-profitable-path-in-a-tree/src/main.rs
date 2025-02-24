use std::collections::HashSet;
use std::usize;

// Solution 1: to be optimized
fn dfs_a(
    map: &Vec<Vec<usize>>,
    node: usize,
    income: i32,
    max_income: &mut i32,
    b_path: &HashSet<usize>,
    d: usize,
    b_depth: usize,
    amount: &Vec<i32>,
    visited: &mut Vec<bool>,
) {
    println!("node: {}", node);
    println!("income: {}", income);
    println!("b_depth: {}", b_depth);
    if map[node].len() == 1 && node != 0 {
        *max_income = (*max_income).max(income);
        return;
    }

    for i in &map[node] {
        if visited[*i] {
            continue;
        }
        match b_path.contains(i) {
            false => {
                visited[*i] = true;
                dfs_a(
                    map,
                    *i,
                    income + amount[*i],
                    max_income,
                    b_path,
                    d + 1,
                    b_depth,
                    amount,
                    visited,
                );
                visited[*i] = false;
            }
            true => {
                match ((d + 1) as f32)
                    .partial_cmp(&(b_depth as f32 / 2.0))
                    .unwrap()
                {
                    std::cmp::Ordering::Equal => {
                        println!("eq");
                        visited[*i] = true;
                        dfs_a(
                            map,
                            *i,
                            income + amount[*i] / 2,
                            max_income,
                            b_path,
                            d + 1,
                            b_depth,
                            amount,
                            visited,
                        );
                        visited[*i] = false;
                    }
                    std::cmp::Ordering::Less => {
                        println!("ls");
                        visited[*i] = true;
                        dfs_a(
                            map,
                            *i,
                            income + amount[*i],
                            max_income,
                            b_path,
                            d + 1,
                            b_depth,
                            amount,
                            visited,
                        );
                        visited[*i] = false;
                    }
                    std::cmp::Ordering::Greater => {
                        println!("ge");
                        visited[*i] = true;
                        dfs_a(
                            map,
                            *i,
                            income,
                            max_income,
                            b_path,
                            d + 1,
                            b_depth,
                            amount,
                            visited,
                        );
                        visited[*i] = false;
                    }
                }
            }
        }
    }
}

fn dfs_b(
    top_down: &Vec<Vec<usize>>,
    bob: usize,
    b_path: &mut HashSet<usize>,
    tmp: &mut HashSet<usize>,
    visited: &mut Vec<bool>,
) {
    // println!("{}", bob);
    if bob == 0 {
        *b_path = tmp.clone();
        return;
    }

    for i in &top_down[bob] {
        if visited[*i] {
            continue;
        }

        tmp.insert(*i);
        visited[*i] = true;
        dfs_b(top_down, *i, b_path, tmp, visited);
        visited[*i] = false;
        tmp.remove(i);
        if b_path.len() != 0 {
            return;
        }
    }
}

pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    let edges: Vec<Vec<usize>> = edges
        .iter()
        .map(|v| v.iter().map(|&n| n as usize).collect())
        .collect();
    let bob = bob as usize;
    let mut map: Vec<Vec<usize>> = vec![vec![]; amount.len()];
    for i in edges {
        map[i[0]].push(i[1]);
        map[i[1]].push(i[0]);
    }
    println!("{:?}", map);

    let mut b_path: HashSet<usize> = HashSet::new();
    let mut tmp: HashSet<usize> = HashSet::new();
    let mut visited: Vec<bool> = vec![false; amount.len()];
    visited[bob] = true;
    dfs_b(&map, bob, &mut b_path, &mut tmp, &mut visited);
    b_path.insert(bob);
    b_path.remove(&0);
    let b_depth = b_path.len();
    println!("{:?}-{}", b_path, b_depth);

    let mut ans = i32::MIN;
    visited = vec![false; amount.len()];
    visited[0] = true;
    dfs_a(
        &map,
        0,
        amount[0],
        &mut ans,
        &b_path,
        0,
        b_depth,
        &amount,
        &mut visited,
    );

    ans
}

fn main() {
    assert_eq!(
        most_profitable_path(
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
            3,
            vec![-2, 4, 2, -4, 6],
        ),
        6
    );
    assert_eq!(
        most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]),
        -7280
    );
    assert_eq!(
        most_profitable_path(
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            3,
            vec![-5644, -6018, 1188, -8502]
        ),
        -11662
    );
    assert_eq!(
        most_profitable_path(
            vec![vec![0, 2], vec![0, 4], vec![1, 3], vec![1, 2]],
            1,
            vec![3958, -9854, -8334, -9388, 3410]
        ),
        7368
    );
    assert_eq!(
        most_profitable_path(
            vec![
                vec![0, 2],
                vec![1, 4],
                vec![1, 6],
                vec![2, 4],
                vec![3, 6],
                vec![3, 7],
                vec![5, 7]
            ],
            4,
            vec![-6896, -1216, -1208, -1108, 1606, -7704, -9212, -8258]
        ),
        -34998
    );
    println!("All passed!");
}
