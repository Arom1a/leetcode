struct ProductOfNumbers {
    prefix_product: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            prefix_product: vec![1],
        }
    }

    fn add(&mut self, nums: i32) {
        let last = self.prefix_product[self.prefix_product.len() - 1];
        if nums == 0 {
            if last == 1 {
                let len = self.prefix_product.len();
                self.prefix_product[len - 1] = 0;
                self.prefix_product.push(1);
            } else {
                self.prefix_product.fill(0);
                self.prefix_product.push(1);
            }
            // println!("{:?}", self.prefix_product);
            return;
        }
        self.prefix_product.push(last * nums);
        // println!("{:?}", self.prefix_product);
    }

    fn get_product(&self, k: i32) -> i32 {
        let len = self.prefix_product.len();
        if self.prefix_product[len - 1 - k as usize] == 0 {
            return 0;
        }
        self.prefix_product[len - 1] / self.prefix_product[len - 1 - k as usize]
    }

    fn pppprrrriiiinnnntttt(&self) {
        println!("{:?}", self.prefix_product);
    }
}

fn main() {
    let mut test = ProductOfNumbers::new();
    test.add(3);
    test.pppprrrriiiinnnntttt();
    test.add(0);
    test.pppprrrriiiinnnntttt();
    test.add(2);
    test.pppprrrriiiinnnntttt();
    test.add(5);
    test.pppprrrriiiinnnntttt();
    test.add(4);
    test.pppprrrriiiinnnntttt();
    assert_eq!(test.get_product(2), 20);
    assert_eq!(test.get_product(3), 40);
    assert_eq!(test.get_product(4), 0);
    test.add(8);
    test.pppprrrriiiinnnntttt();
    assert_eq!(test.get_product(2), 32);
    test.add(0);
    test.pppprrrriiiinnnntttt();
    assert_eq!(test.get_product(1), 0);
    assert_eq!(test.get_product(2), 0);
    test.add(1);
    assert_eq!(test.get_product(1), 1);
    println!("All passed!");
}
