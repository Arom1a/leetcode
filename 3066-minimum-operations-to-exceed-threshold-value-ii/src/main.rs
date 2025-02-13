pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<Reverse<u64>> = nums.into_iter().map(|x| Reverse(x as u64)).collect();

    let mut count = 0;
    while heap.peek().unwrap().0 < k as u64 {
        let val = Reverse(
            heap.pop().expect("more than 1 value").0 * 2
                + heap.pop().expect("more than 2 values").0,
        );
        heap.push(val);
        count += 1;
    }

    count
}

fn main() {
    assert_eq!(min_operations(vec![2, 11, 10, 1, 3], 10), 2);
    assert_eq!(min_operations(vec![9, 9, 9], 10), 2);
    assert_eq!(
        min_operations(vec![999999999, 999999999, 999999999], 1000000000),
        2
    );
    println!("All passed!");
}
