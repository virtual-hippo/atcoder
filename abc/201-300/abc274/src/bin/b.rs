use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    for j in 0..w {
        let ans = (0..h).filter(|i| c[*i][j] == '#').count();
        print!("{} ", ans);
    }
}
