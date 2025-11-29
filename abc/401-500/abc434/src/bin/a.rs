use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let w = w * 1000;

    let ans = (1..)
        .find(|&n| w < n * b)
        .unwrap();

    println!("{}", ans);
}
