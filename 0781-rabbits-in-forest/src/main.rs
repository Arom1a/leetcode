// Solution 1: TODO
pub fn num_rabbits(mut answers: Vec<i32>) -> i32 {
    answers.sort_unstable();

    answers
        .chunk_by(|a, b| a == b) // Split into contiguous groups of equal answers
        .map(|group| {
            let group_size = group[0] + 1;
            (group[0] + group.len() as i32) / group_size * group_size
        })
        .sum()
}

fn main() {
    assert_eq!(num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(num_rabbits(vec![10, 10, 10]), 11);
    println!("All passed!");
}
