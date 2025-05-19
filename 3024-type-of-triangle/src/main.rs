pub fn triangle_type(nums: Vec<i32>) -> String {
    if !(nums[0] + nums[1] > nums[2] && nums[1] + nums[2] > nums[0] && nums[2] + nums[0] > nums[1])
    {
        return "none".to_string();
    }

    if nums[0] == nums[1] && nums[1] == nums[2] {
        return "equilateral".to_string();
    }
    if nums[0] == nums[1] || nums[1] == nums[2] || nums[2] == nums[0] {
        return "isosceles".to_string();
    }
    "scalene".to_string()
}

fn main() {
    assert_eq!(triangle_type(vec![3, 3, 3]), "equilateral".to_string());
    assert_eq!(triangle_type(vec![3, 3, 2]), "isosceles".to_string());
    assert_eq!(triangle_type(vec![3, 4, 5]), "scalene".to_string());
    println!("All passed!");
}
