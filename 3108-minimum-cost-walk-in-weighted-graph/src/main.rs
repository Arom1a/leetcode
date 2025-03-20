// Solution 1
fn union(node1: i32, node2: i32, parent: &mut Vec<i32>, depth: &mut Vec<i32>) {
    let mut root1 = find(node1, parent);
    let mut root2 = find(node2, parent);

    if root1 == root2 {
        return;
    }

    if depth[root1 as usize] < depth[root2 as usize] {
        std::mem::swap(&mut root1, &mut root2);
    }

    parent[root2 as usize] = root1;

    if depth[root1 as usize] == depth[root2 as usize] {
        depth[root1 as usize] += 1;
    }
}

fn find(node: i32, parent: &mut Vec<i32>) -> i32 {
    if parent[node as usize] == -1 {
        return node;
    }
    parent[node as usize] = find(parent[node as usize], parent);
    return parent[node as usize];
}

pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut parent = vec![-1; n];
    let mut depth = vec![0; n];

    let mut component_cost = vec![-1; n];

    for e in &edges {
        union(e[0], e[1], &mut parent, &mut depth);
    }

    for e in &edges {
        let root = find(e[0], &mut parent);
        component_cost[root as usize] &= e[2];
    }

    let mut ans = vec![];
    for q in &query {
        let start = q[0];
        let end = q[1];

        if find(start, &mut parent) != find(end, &mut parent) {
            ans.push(-1);
        } else {
            let root = find(start, &mut parent);
            ans.push(component_cost[root as usize]);
        }
    }

    ans
}

fn main() {
    assert_eq!(
        minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]],
        ),
        vec![1, -1]
    );
    assert_eq!(
        minimum_cost(
            3,
            vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
            vec![vec![1, 2]]
        ),
        vec![0]
    );
    assert_eq!(
        minimum_cost(
            7,
            vec![
                vec![6, 0, 0],
                vec![4, 1, 1],
                vec![5, 4, 0],
                vec![1, 2, 2],
                vec![3, 0, 2],
                vec![2, 0, 1],
                vec![0, 4, 2],
                vec![1, 6, 1],
                vec![1, 3, 1],
                vec![3, 0, 1]
            ],
            vec![
                vec![5, 4],
                vec![0, 5],
                vec![4, 0],
                vec![1, 5],
                vec![1, 3],
                vec![1, 5]
            ]
        ),
        vec![0, 0, 0, 0, 0, 0]
    );
    println!("All passed!");
}
