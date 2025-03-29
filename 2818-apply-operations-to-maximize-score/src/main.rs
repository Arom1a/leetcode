// Solution 1: TODO
use std::collections::{BinaryHeap, VecDeque};

struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
        let n = nums.len();
        let mut prime_scores = vec![0; n];

        // Compute prime score for each number
        for (index, &num) in nums.iter().enumerate() {
            let mut num = num;
            let mut factor = 2;
            while factor * factor <= num {
                if num % factor == 0 {
                    prime_scores[index] += 1;
                    while num % factor == 0 {
                        num /= factor;
                    }
                }
                factor += 1;
            }
            if num >= 2 {
                prime_scores[index] += 1;
            }
        }

        let mut next_dominant = vec![n as i32; n];
        let mut prev_dominant = vec![-1; n];
        let mut stack = VecDeque::new();

        for index in 0..n {
            while let Some(&top_index) = stack.back() {
                if prime_scores[top_index] >= prime_scores[index] {
                    break;
                }
                stack.pop_back();
                next_dominant[top_index] = index as i32;
            }
            if let Some(&top_index) = stack.back() {
                prev_dominant[index] = top_index as i32;
            }
            stack.push_back(index);
        }

        let mut num_of_subarrays = vec![0_i64; n];
        for index in 0..n {
            num_of_subarrays[index] = (next_dominant[index] as i64 - index as i64)
                * (index as i64 - prev_dominant[index] as i64);
        }

        let mut pq = BinaryHeap::new();
        for (index, &num) in nums.iter().enumerate() {
            pq.push((num, index));
        }

        let mut score = 1_i64;

        while k > 0 {
            if let Some((num, index)) = pq.pop() {
                let operations = num_of_subarrays[index].min(k as i64);
                score = (score * Self::power(num as i64, operations)) % Self::MOD;
                k -= operations as i32;
            }
        }

        score as i32
    }

    fn power(mut base: i64, mut exponent: i64) -> i64 {
        let mut result = 1;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % Self::MOD;
            }
            base = (base * base) % Self::MOD;
            exponent /= 2;
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
    assert_eq!(
        Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3),
        4788
    );
    println!("All passed!");
}
