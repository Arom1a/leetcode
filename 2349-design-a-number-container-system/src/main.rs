use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    i2n: HashMap<i32, i32>,
    n2i: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            i2n: HashMap::new(),
            n2i: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&num) = self.i2n.get(&index) {
            self.n2i.entry(num).or_default().remove(&index);
        }
        self.n2i.entry(number).or_default().insert(index);
        self.i2n.insert(index, number);
    }

    fn find(&self, number: i32) -> i32 {
        match self.n2i.get(&number) {
            Some(bts) => *bts.first().unwrap_or(&-1),
            None => -1,
        }
    }
}

fn main() {
    let mut a = NumberContainers::new();
    assert_eq!(a.find(10), -1);
    a.change(2, 10);
    a.change(1, 10);
    assert_eq!(a.find(10), 1);
    assert_eq!(a.find(10), 1);
    a.change(3, 10);
    a.change(5, 10);
    assert_eq!(a.find(10), 1);
    a.change(1, 20);
    assert_eq!(a.find(10), 2);
    assert_eq!(a.find(20), 1);
    println!("All passed!");
}
