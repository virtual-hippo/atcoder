use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut grid = vec![vec![usize::MAX; n]; n];

    grid[0][(n - 1) / 2] = 1;
    let (mut r, mut c) = (0, (n - 1) / 2);
    let mut k = 1;

    for _ in 0..(n * n) - 1 {
        (r, c) = if grid[(n + r - 1) % n][(c + 1) % n] == usize::MAX {
            ((n + r - 1) % n, (c + 1) % n)
        } else {
            ((r + 1) % n, c)
        };

        k = k + 1;
        grid[r][c] = k;
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}
