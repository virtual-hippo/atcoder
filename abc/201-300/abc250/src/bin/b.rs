use itertools::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let h = n * a;
    let w = n * b;

    let ans = iproduct!(0..h, 0..w).fold(vec![vec![]; h], |mut grid, (i, j)| -> Vec<Vec<char>> {
        if (i / a) % 2 == 0 && (j / b) % 2 == 0 {
            grid[i].push('.');
        }
        if (i / a) % 2 == 1 && (j / b) % 2 == 0 {
            grid[i].push('#');
        }
        if (i / a) % 2 == 0 && (j / b) % 2 == 1 {
            grid[i].push('#');
        }
        if (i / a) % 2 == 1 && (j / b) % 2 == 1 {
            grid[i].push('.');
        }
        grid
    });

    iproduct!(0..h, 0..w).for_each(|(i, j)| {
        if j < w - 1 {
            print!("{}", ans[i][j]);
        } else {
            println!("{}", ans[i][j]);
        }
    });
}
