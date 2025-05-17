// Solution 1: troll
// pub fn sort_colors(nums: &mut Vec<i32>) {
//     nums.sort_unstable();
// }

// Solution 2
pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut red_cnt, mut white_cnt, mut blue_cnt) = (0, 0, 0);
    nums.iter().for_each(|x| match *x {
        0 => red_cnt += 1,
        1 => white_cnt += 1,
        _ => blue_cnt += 1,
    });
    for i in 0..red_cnt {
        nums[i] = 0;
    }
    for i in 0..white_cnt {
        nums[red_cnt + i] = 1;
    }
    for i in 0..blue_cnt {
        nums[red_cnt + white_cnt + i] = 2;
    }
}

fn main() {
    let mut test1 = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut test1);
    assert_eq!(test1, vec![0, 0, 1, 1, 2, 2]);

    let mut test2 = vec![2, 0, 1];
    sort_colors(&mut test2);
    assert_eq!(test2, vec![0, 1, 2]);
    println!("All passed!");
}
