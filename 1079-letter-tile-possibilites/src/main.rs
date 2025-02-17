// Solution 1: backtracking
fn helper(frequency: &mut [i32; 26]) -> i32 {
    let mut count = 0;

    for i in 0..26 {
        if frequency[i] == 0 {
            continue;
        }

        count += 1;
        frequency[i] -= 1;
        count += helper(frequency);
        frequency[i] += 1;
    }

    count
}

pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut frequency = [0; 26];
    for c in tiles.chars() {
        frequency[(c as u8 - b'A') as usize] += 1;
    }

    helper(&mut frequency)
}

fn main() {
    assert_eq!(num_tile_possibilities("AAB".to_string()), 8);
    assert_eq!(num_tile_possibilities("AAABBC".to_string()), 188);
    assert_eq!(num_tile_possibilities("V".to_string()), 1);
    println!("All passed!");
}
