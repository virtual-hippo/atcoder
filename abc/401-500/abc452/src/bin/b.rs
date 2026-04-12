use itertools::*;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let ans = iproduct!(0..h, 0..w)
        .map(|(i, j)| if i == 0 || i == h - 1 || j == 0 || j == w - 1 { '#' } else { '.' })
        .collect_vec();

    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i * w + j]);
        }
        println!();
    }
}
