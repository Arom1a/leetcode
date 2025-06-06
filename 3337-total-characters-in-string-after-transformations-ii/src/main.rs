// Solution 1: TODO
const MOD: i64 = 1_000_000_007;
const L: usize = 26;

struct Mat {
    a: [[i64; L]; L],
}

impl Mat {
    fn new() -> Self {
        Mat { a: [[0; L]; L] }
    }

    fn copy_from(&mut self, other: &Mat) {
        for i in 0..L {
            for j in 0..L {
                self.a[i][j] = other.a[i][j];
            }
        }
    }

    fn mul(&self, other: &Mat) -> Mat {
        let mut result = Mat::new();
        for i in 0..L {
            for j in 0..L {
                for k in 0..L {
                    result.a[i][j] = (result.a[i][j] + self.a[i][k] * other.a[k][j]) % MOD;
                }
            }
        }
        result
    }
}

fn I() -> Mat {
    let mut m = Mat::new();
    for i in 0..L {
        m.a[i][i] = 1;
    }
    m
}

fn quickmul(x: &Mat, mut y: i32) -> Mat {
    let mut ans = I();
    let mut cur = Mat::new();
    cur.copy_from(x);
    while y > 0 {
        if y & 1 == 1 {
            ans = ans.mul(&cur);
        }
        cur = cur.mul(&cur);
        y >>= 1;
    }
    ans
}

pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
    let mut T = Mat::new();
    for i in 0..L {
        for j in 1..=nums[i] as usize {
            T.a[(i + j) % L][i] = 1;
        }
    }

    let res = quickmul(&T, t);
    let mut f = [0; L];
    for ch in s.chars() {
        f[(ch as u8 - b'a') as usize] += 1;
    }
    let mut ans: i64 = 0;
    for i in 0..L {
        for j in 0..L {
            ans = (ans + res.a[i][j] * f[j]) % MOD;
        }
    }
    ans as i32
}

fn main() {
    assert_eq!(
        length_after_transformations(
            "abcyy".to_string(),
            2,
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2
            ]
        ),
        7
    );
    assert_eq!(
        length_after_transformations(
            "azbk".to_string(),
            1,
            vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
            ]
        ),
        8
    );
    println!("All passed!");
}
