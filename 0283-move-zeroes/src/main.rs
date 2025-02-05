pub fn move_zeroes(nums: &mut Vec<i32>) {
    let zero_num = nums.iter().filter(|&&x| x == 0).count();
    let mut it: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[it] = nums[i];
            it += 1;
        }
    }
    for i in (nums.len() - zero_num)..nums.len() {
        nums[i] = 0;
    }
}

fn main() {
    let mut test = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut test);
    assert_eq!(test, vec![1, 3, 12, 0, 0]);

    let mut test = vec![0];
    move_zeroes(&mut test);
    assert_eq!(test, vec![0]);
    println!("All passed!");
}
