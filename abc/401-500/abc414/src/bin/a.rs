use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize, usize); n],
    }

    let ans = xy.into_iter().filter(|&(x, y)| x <= l && r <= y).count();

    println!("{}", ans);
}
