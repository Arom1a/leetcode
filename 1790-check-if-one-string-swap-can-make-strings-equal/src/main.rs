pub fn are_almost_equal(s1: String, s2: String) -> bool {
    assert!(s1.len() == s2.len());
    let len = s1.len();
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    let mut storage: Vec<usize> = vec![];

    for i in 0..len {
        if v1[i] != v2[i] {
            storage.push(i);
        }
        if storage.len() > 2 {
            return false;
        }
    }

    if storage.len() == 0
        || (storage.len() == 2
            && v1[storage[0]] == v2[storage[1]]
            && v1[storage[1]] == v2[storage[0]])
    {
        return true;
    }

    false
}

fn main() {
    assert_eq!(
        are_almost_equal("bank".to_string(), "kanb".to_string()),
        true
    );
    assert_eq!(
        are_almost_equal("attack".to_string(), "defend".to_string()),
        false
    );
    assert_eq!(
        are_almost_equal("kelb".to_string(), "kelb".to_string()),
        true
    );
    println!("All passed!");
}
