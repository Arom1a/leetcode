// Solution 1: DFS
// fn search_component(
//     node: usize,
//     visited: &mut Vec<bool>,
//     component_idx: usize,
//     node2component: &mut Vec<usize>,
//     adj: &Vec<Vec<usize>>,
// ) {
//     if visited[node] {
//         return;
//     }
//     visited[node] = true;
//     node2component[node] = component_idx;
//
//     for next_node in &adj[node] {
//         search_component(*next_node, visited, component_idx, node2component, adj);
//     }
// }
//
// pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
//     let n = n as usize;
//     let mut adj = vec![vec![]; n];
//     for e in &edges {
//         let (u, v) = (e[0] as usize, e[1] as usize);
//         adj[u].push(v);
//         adj[v].push(u);
//     }
//     let mut visited = vec![false; n];
//     let mut node2component = vec![0; n];
//
//     let mut component_idx = 0;
//     for i in 0..n {
//         if visited[i] {
//             continue;
//         }
//         search_component(i, &mut visited, component_idx, &mut node2component, &adj);
//         component_idx += 1;
//     }
//
//     println!("{:?}", node2component);
//
//     let mut component2edge_num = vec![0; component_idx];
//     let mut component2node_num = vec![0; component_idx];
//
//     for i in &node2component {
//         component2node_num[*i] += 1;
//     }
//     println!("{:?}", component2node_num);
//     for i in &edges {
//         component2edge_num[node2component[i[0] as usize]] += 1;
//     }
//     println!("{:?}", component2edge_num);
//
//     let mut ans = 0;
//     for i in 0..component_idx {
//         let node_num = component2node_num[i];
//         let actl_edge_num = component2edge_num[i];
//         let expt_edge_num = node_num * (node_num - 1) / 2;
//         match actl_edge_num.cmp(&expt_edge_num) {
//             std::cmp::Ordering::Equal => ans += 1,
//             std::cmp::Ordering::Less => {}
//             std::cmp::Ordering::Greater => panic!("no possible"),
//         }
//     }
//
//     ans
// }

// Solution 2: improved DFS
use std::collections::HashSet;

fn dfs(
    node: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    component_info: &mut [usize; 2],
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    component_info[0] += 1;
    component_info[1] += adj[node].len();

    for next in &adj[node] {
        dfs(*next, adj, visited, component_info);
    }
}

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in &edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        adj[u].push(v);
        adj[v].push(u);
    }
    let adj = adj;

    let mut ans = 0;
    let mut visited = HashSet::new();

    for node in 0..n {
        if visited.contains(&node) {
            continue;
        }

        let mut component_info = [0, 0];
        dfs(node, &adj, &mut visited, &mut component_info);

        if component_info[0] * (component_info[0] - 1) == component_info[1] {
            ans += 1;
        }
    }

    ans
}

// Solution 3: BFS
// pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
//     use std::collections::VecDeque;
//     let n = n as usize;
//     let mut adj = vec![vec![]; n];
//     for e in &edges {
//         let (u, v) = (e[0] as usize, e[1] as usize);
//         adj[u].push(v);
//         adj[v].push(u);
//     }
//     let adj = adj;
//
//     let mut visited = vec![false; n];
//     let mut ans = 0;
//
//     for node in 0..n {
//         if !visited[node] {
//             let mut component = vec![];
//             let mut q = VecDeque::new();
//             q.push_back(node);
//             visited[node] = true;
//
//             while let Some(curr_node) = q.pop_front() {
//                 component.push(curr_node);
//
//                 for next_node in &adj[curr_node] {
//                     if !visited[*next_node] {
//                         q.push_back(*next_node);
//                         visited[*next_node] = true;
//                     }
//                 }
//             }
//
//             let mut is_complete = true;
//             for node in &component {
//                 if component.len() - 1 != adj[*node].len() {
//                     is_complete = false;
//                     break;
//                 }
//             }
//             if is_complete {
//                 ans += 1;
//             }
//         }
//     }
//
//     ans
// }

fn main() {
    assert_eq!(
        count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4],]),
        3
    );
    assert_eq!(
        count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
        ),
        1
    );
    println!("All passed!");
}
