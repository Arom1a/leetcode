// Solution 1: TODO
pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;

    queries.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut heap = BinaryHeap::new();
    let mut delta_array = vec![0; nums.len() + 1];
    let mut operations = 0;
    let mut j = 0;

    for i in 0..nums.len() {
        operations += delta_array[i];
        while j < queries.len() && queries[j][0] == i as i32 {
            heap.push(queries[j][1]);
            j += 1;
        }
        while operations < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
            operations += 1;
            let end = heap.pop().unwrap() as usize;
            delta_array[end + 1] -= 1;
        }
        if operations < nums[i] {
            return -1;
        }
    }
    heap.len() as i32
}

fn main() {
    assert_eq!(
        max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
        1
    );
    assert_eq!(
        max_removal(
            vec![1, 1, 1, 1],
            vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]
        ),
        2
    );
    assert_eq!(max_removal(vec![1, 2, 3, 4], vec![vec![0, 3]]), -1);
    println!("All passed!");
}
