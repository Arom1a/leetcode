pub fn asteroids_destroyed(mass: i32, mut asters: Vec<i32>) -> bool {
    let mut mass = mass as i64;
    asters.sort_unstable();
    for i in asters {
        let i = i as i64;
        if mass >= i {
            mass += i;
        } else {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(asteroids_destroyed(10, vec![3, 9, 19, 5, 21]), true);
    assert_eq!(asteroids_destroyed(5, vec![4, 9, 23, 4]), false);
    println!("All tests passed!");
}
