use crate::Intersect::*;
use crate::Line::*;
use std::collections::HashMap;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Clone)]
struct Frac {
    s: bool, // has minus sign
    p: i32,
    q: i32,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Clone)]
enum Line {
    V,       // vertical
    N(Frac), // slope
    H,       // horizontal
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
enum Intersect {
    X(i32),
    Ni(Frac),
    Y(i32),
}

fn gcd(mut p: i32, mut q: i32) -> i32 {
    while p != 0 {
        let tmp = p;
        p = q % p;
        q = tmp;
    }
    q
}

fn calc_b((x, y): (i32, i32), Frac { s, p, q }: &Frac) -> Intersect {
    let (np, nq) = (y * q - x * p * (if *s { -1 } else { 1 }), q.clone());
    let ns = s ^ (np < 0) ^ (nq < 0);
    let (np, nq) = (np.abs(), nq.abs());
    Ni(Frac {
        s: ns,
        p: np,
        q: nq,
    })
}

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let mut lines: HashMap<Line, HashMap<Intersect, i32>> = HashMap::new();
    let mut midpoints_db: HashMap<(i32, i32), HashMap<Line, i32>> = HashMap::new();
    for (i, d1) in points.iter().enumerate() {
        for d2 in &points[i + 1..] {
            let (x1, y1) = (d1[0], d1[1]);
            let (x2, y2) = (d2[0], d2[1]);
            let (slope, intersect) = if x1 == x2 {
                (V, X(x1))
            } else if y1 == y2 {
                (H, Y(y1))
            } else {
                let (p, q) = (y1 - y2, x1 - x2);
                let s = (p < 0) ^ (q < 0);

                let (p, q) = (p.abs(), q.abs());
                let pqgcd = gcd(p, q);
                let (p, q) = (p / pqgcd, q / pqgcd);

                let slope = Frac { s, p, q };

                let b = calc_b((x1, y1), &slope);
                (N(slope), b)
            };
            *(*midpoints_db
                .entry((x1 + x2, y1 + y2))
                .or_insert(HashMap::new()))
            .entry(slope.clone())
            .or_insert(0) += 1;
            if intersect
                == Ni(Frac {
                    s: false,
                    p: 70,
                    q: 1,
                })
            {}
            *(*lines.entry(slope.clone()).or_insert(HashMap::new()))
                .entry(intersect)
                .or_insert(0) += 1;
        }
    }

    let ans = lines.values().fold(0, |mut acc, map| {
        let num_vec: Vec<&i32> = map.values().collect();
        for (i, &&x) in num_vec.iter().enumerate() {
            for &y in &num_vec[i + 1..] {
                acc += x * y;
            }
        }
        acc
    });
    let parallelogram_count = midpoints_db.values().fold(0, |mut acc, map| {
        let num_vec: Vec<&i32> = map.values().collect();
        for (i, &&x) in num_vec.iter().enumerate() {
            for &y in &num_vec[i + 1..] {
                acc += x * y;
            }
        }
        acc
    });
    ans - parallelogram_count
}

fn main() {
    assert_eq!(
        count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]]),
        1
    );
    assert_eq!(
        count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![1, 1], vec![2, 1]]),
        1
    );
    assert_eq!(
        count_trapezoids(vec![
            vec![-3, 2],
            vec![3, 0],
            vec![2, 3],
            vec![3, 2],
            vec![2, -3]
        ]),
        2
    );
    assert_eq!(
        count_trapezoids(vec![
            vec![-94, -57],
            vec![99, 99],
            vec![99, -78],
            vec![83, -78],
            vec![74, 99],
            vec![-27, -57]
        ],),
        3
    );

    assert_eq!(
        count_trapezoids(vec![
            vec![-33, -9],
            vec![30, -37],
            vec![-10, -9],
            vec![61, -9],
            vec![56, -67],
            vec![36, -9],
            vec![36, 100],
            vec![36, 96],
            vec![-32, 84],
            vec![18, 34],
            vec![-10, -82]
        ]),
        3
    );
    println!("All tests passed!");
}
