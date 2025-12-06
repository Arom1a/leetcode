use std::collections::VecDeque;

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn color_bfs(cg: &mut Vec<Vec<i32>>, r: usize, c: usize, ctu: i32) {
    let h = cg.len() as i32;
    let w = cg[0].len() as i32;
    let mut q = VecDeque::new();
    q.push_back((r, c));
    cg[r][c] = ctu;
    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in DIR {
            let (ix, iy) = (x as i32 + dx, y as i32 + dy);
            if ix < 0 || ix >= h || iy < 0 || iy >= w {
                continue;
            }
            let (nx, ny) = (ix as usize, iy as usize);
            if cg[nx][ny] != -1 {
                continue;
            }
            q.push_back((nx, ny));
            cg[nx][ny] = ctu;
        }
    }
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let h = grid.len();
    let w = grid[0].len();

    let mut cg = vec![vec![-1; w]; h];
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == '0' {
                cg[r][c] = 0;
            }
        }
    }

    let mut ctu = 0;
    for r in 0..h {
        for c in 0..w {
            if cg[r][c] != -1 {
                continue;
            }
            ctu += 1;
            color_bfs(&mut cg, r, c, ctu);
        }
    }

    ctu
}

fn main() {
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '0'],
            vec!['0', '0', '0'],
            vec!['0', '0', '0'],
        ]),
        1
    );
    assert_eq!(
        num_islands(vec![
            vec!['1', '1', '0'],
            vec!['0', '0', '0'],
            vec!['0', '1', '0'],
        ]),
        2
    );
    println!("All tests passed!");
}
